package ro.zach.language.unit

import androidx.compose.foundation.ScrollState
import androidx.compose.foundation.clickable
import androidx.compose.ui.draw.drawBehind
import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.semantics.selected
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.unit.Dp
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.launch
import ro.zach.language.audio.AudioPlayer
import ro.zach.language.audio.LocalAudioPlayer
import ro.zach.language.audio.RecordingPrefetch
import ro.zach.language.core.AudioEngineRecord
import ro.zach.language.core.CoursePublication
import ro.zach.language.core.UnitLessonRecord
import ro.zach.language.core.UnitOutlineRecord
import ro.zach.language.core.UnitTabRecord
import ro.zach.language.coursedata.CourseStore
import ro.zach.language.coursedata.LocalCourseStore
import ro.zach.language.coursedata.UnitLoadState
import ro.zach.language.design.ColumnIndexRailTracking
import ro.zach.language.design.ColumnScrollTargets
import ro.zach.language.design.FloatingTipHost
import ro.zach.language.design.FloatingTipState
import ro.zach.language.design.IndexRail
import ro.zach.language.design.IndexRailEntry
import ro.zach.language.design.LanguageColors
import ro.zach.language.design.LanguageSpacing
import ro.zach.language.design.LanguageText
import ro.zach.language.design.LanguageType
import ro.zach.language.design.PageHeader
import ro.zach.language.design.PendingStatusMessage
import ro.zach.language.design.TextLink
import ro.zach.language.design.indexRailContent
import ro.zach.language.design.readableWidth
import ro.zach.language.design.rememberColumnScrollTargets
import ro.zach.language.design.rememberFloatingTipState
import ro.zach.language.progressdata.LocalProgressStore
import ro.zach.language.progressdata.ProgressStore

/** The unit page's scrolling viewport. */
const val UNIT_PAGE_TEST_TAG: String = "unit-page"

/** What a shown tab's recording prefetch depends on; a change of any restarts it. */
private data class RecordingPrefetchRequest(val tab: UnitTabKind, val engine: AudioEngineRecord, val contentKey: String?)

enum class UnitTabKind {
    Overview,
    Grammar,
    Dialogues,
    Readings,
    Vocabulary,
    Practice;

    /** The tab's name in test tags and in the web's addresses. */
    fun rawValue(): String {
        return name.lowercase()
    }

    fun label(): String {
        return when (this) {
            Overview -> "Overview"
            Grammar -> "Principles"
            Dialogues -> "Dialogues"
            Readings -> "Reading"
            Vocabulary -> "Vocabulary"
            Practice -> "Practice"
        }
    }

    fun isAvailable(lesson: UnitLessonRecord): Boolean {
        return this != Readings || lesson.readings.isNotEmpty()
    }

    /** What one box of the tab is called, for a tab made of a series of boxes. */
    fun indexedItemNoun(): String? {
        return when (this) {
            Grammar -> "note"
            Dialogues -> "dialogue"
            Readings -> "reading"
            Overview, Vocabulary, Practice -> null
        }
    }

    fun indexedItemCount(lesson: UnitLessonRecord): Int {
        return when (this) {
            Grammar -> lesson.grammar.size
            Dialogues -> lesson.dialogues.size
            Readings -> lesson.readings.size
            Overview, Vocabulary, Practice -> 0
        }
    }

    fun record(): UnitTabRecord {
        return when (this) {
            Overview -> UnitTabRecord.OVERVIEW
            Grammar -> UnitTabRecord.GRAMMAR
            Dialogues -> UnitTabRecord.DIALOGUES
            Readings -> UnitTabRecord.READINGS
            Vocabulary -> UnitTabRecord.VOCABULARY
            Practice -> UnitTabRecord.PRACTICE
        }
    }

    companion object {
        fun availableTabs(lesson: UnitLessonRecord): List<UnitTabKind> {
            return entries.filter { tab -> tab.isAvailable(lesson) }
        }
    }
}

@Composable
fun UnitPage(courseId: String, unitId: String, initialTab: UnitTabKind, modifier: Modifier = Modifier) {
    val courseStore: CourseStore = LocalCourseStore.current
    val audioPlayer: AudioPlayer = LocalAudioPlayer.current
    val shownContentKey: String? = courseStore.shownContentKey(courseId)
    val outlineId: String? = courseStore.outline(courseId)?.id

    LaunchedEffect(courseId, unitId, shownContentKey, outlineId) {
        courseStore.ensurePublicationLoaded(courseId)
        courseStore.ensureOutlineLoaded(courseId)
        courseStore.ensureUnitLoaded(courseId, unitId)
    }

    DisposableEffect(audioPlayer) {
        onDispose { audioPlayer.stop() }
    }

    when (val unitState: UnitLoadState? = courseStore.unitState(courseId, unitId)) {
        is UnitLoadState.Loaded -> UnitLoadedPage(courseId = courseId, unit = unitState.unit, lesson = unitState.lesson, initialTab = initialTab, modifier = modifier)
        is UnitLoadState.Failed -> LanguageText(
            text = "This unit could not be loaded. (${unitState.message})",
            style = LanguageType.bodyText,
            color = LanguageColors.accentActive,
            modifier = modifier.padding(LanguageSpacing.spaceMd),
        )
        UnitLoadState.Loading, null -> PendingStatusMessage(text = "Loading the unit…", modifier = modifier)
    }
}

@Composable
private fun UnitLoadedPage(courseId: String, unit: UnitOutlineRecord, lesson: UnitLessonRecord, initialTab: UnitTabKind, modifier: Modifier) {
    var activeTab: UnitTabKind? by rememberSaveable { mutableStateOf(null) }
    var isStatusHelpShown: Boolean by remember { mutableStateOf(false) }
    val tab: UnitTabKind = activeTab ?: initialTab
    val scrollState: ScrollState = rememberScrollState()
    val railTargets: ColumnScrollTargets = rememberColumnScrollTargets(scrollState)
    val floatingTipState: FloatingTipState = rememberFloatingTipState()
    val itemNoun: String? = tab.indexedItemNoun()
    val scope: CoroutineScope = rememberCoroutineScope()
    val audioPlayer: AudioPlayer = LocalAudioPlayer.current
    val courseStore: CourseStore = LocalCourseStore.current
    val progressStore: ProgressStore = LocalProgressStore.current
    val prefetchRequest: RecordingPrefetchRequest = RecordingPrefetchRequest(
        tab = tab,
        engine = progressStore.settings().audioEngine,
        contentKey = courseStore.shownContentKey(courseId),
    )
    /* A newly chosen tab opens at its top. */
    val chooseTab: (UnitTabKind) -> Unit = { chosenTab: UnitTabKind ->
        audioPlayer.stop()
        activeTab = chosenTab
        railTargets.tracker.recordContentChange()
        scope.launch { scrollState.scrollTo(0) }
    }
    val closeStatusHelp: () -> Unit = { isStatusHelpShown = false }

    ColumnIndexRailTracking(railTargets)

    LaunchedEffect(prefetchRequest) {
        val publication: CoursePublication = courseStore.shownPublication(courseId)
            ?: return@LaunchedEffect

        RecordingPrefetch.prefetchTab(prefetchRequest.tab, unit.id, publication, prefetchRequest.engine)
    }

    FloatingTipHost(
        state = floatingTipState,
        isShown = isStatusHelpShown,
        autoCloseMilliseconds = UNIT_STATUS_HELP_AUTO_CLOSE_MILLISECONDS,
        onDismiss = closeStatusHelp,
        tip = { UnitStatusHelpTip(onClose = closeStatusHelp) },
        modifier = modifier.fillMaxSize(),
    ) {
        Column(modifier = Modifier.fillMaxSize().testTag(UNIT_PAGE_TEST_TAG).verticalScroll(scrollState)) {
            Column(
                modifier = Modifier
                    .indexRailContent(railTargets)
                    .readableWidth()
                    .padding(LanguageSpacing.spaceMd),
                verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceMd),
            ) {
                PageHeader(caption = "${unit.levelCode} · ${unit.order}", title = unit.title)
                UnitStatusPanel(
                    courseId = courseId,
                    unitId = unit.id,
                    isHelpShown = isStatusHelpShown,
                    onToggleHelp = { isStatusHelpShown = !isStatusHelpShown },
                    floatingTipState = floatingTipState,
                )
                UnitTabRow(lesson = lesson, activeTab = tab, onChoose = chooseTab)
                UnitSection(tab = tab, courseId = courseId, unit = unit, lesson = lesson, railTargets = railTargets)
                UnitTabNavigation(lesson = lesson, activeTab = tab, onChoose = chooseTab)
            }
        }

        if (itemNoun != null) {
            IndexRail(
                entries = indexRailEntries(tab = tab, itemNoun = itemNoun, unit = unit, lesson = lesson),
                railAccessibilityLabel = "${itemNoun.replaceFirstChar { character -> character.uppercase() }} index",
                railTestTag = "unit-item-index",
                tracker = railTargets.tracker,
                scrollTarget = railTargets,
                modifier = Modifier.align(Alignment.CenterEnd),
            )
        }
    }
}

private fun indexRailEntries(tab: UnitTabKind, itemNoun: String, unit: UnitOutlineRecord, lesson: UnitLessonRecord): List<IndexRailEntry> {
    val itemCount: Int = tab.indexedItemCount(lesson)

    return (1..itemCount).map { itemNumber: Int ->
        IndexRailEntry(visibleText = "$itemNumber", accessibilityLabel = "Go to $itemNoun ${unit.itemLabel(itemNumber)}")
    }
}

@Composable
private fun UnitTabRow(lesson: UnitLessonRecord, activeTab: UnitTabKind, onChoose: (UnitTabKind) -> Unit) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .horizontalScroll(rememberScrollState())
            .testTag("unit-tabs"),
    ) {
        for (tab in UnitTabKind.availableTabs(lesson)) {
            val isActive: Boolean = tab == activeTab
            val underlineColor: Color = if (isActive) LanguageColors.accentActive else LanguageColors.rule
            val underlineHeight: Dp = if (isActive) LanguageSpacing.selectedBorderWidth else LanguageSpacing.hairline

            Box(
                modifier = Modifier
                    .drawBehind {
                        val height: Float = underlineHeight.toPx()

                        drawRect(color = underlineColor, topLeft = Offset(0f, size.height - height), size = size.copy(height = height))
                    }
                    .clickable(interactionSource = null, indication = null, role = Role.Tab) { onChoose(tab) }
                    .semantics { selected = isActive }
                    .testTag("tab-${tab.rawValue()}")
                    .padding(horizontal = LanguageSpacing.spaceSm + LanguageSpacing.spaceXs, vertical = LanguageSpacing.spaceSm),
            ) {
                LanguageText(
                    text = tab.label().uppercase(),
                    style = LanguageType.labelText.copy(letterSpacing = LanguageType.labelTracking),
                    color = if (isActive) LanguageColors.accentActive else LanguageColors.ink,
                )
            }
        }
    }
}

@Composable
private fun UnitSection(tab: UnitTabKind, courseId: String, unit: UnitOutlineRecord, lesson: UnitLessonRecord, railTargets: ColumnScrollTargets) {
    when (tab) {
        UnitTabKind.Overview -> OverviewSection(courseId = courseId, unit = unit, lesson = lesson)
        UnitTabKind.Grammar -> GrammarSection(unit = unit, notes = lesson.grammar, railTargets = railTargets)
        UnitTabKind.Dialogues -> DialoguesSection(unit = unit, dialogues = lesson.dialogues, railTargets = railTargets)
        UnitTabKind.Readings -> ReadingsSection(unit = unit, readings = lesson.readings, railTargets = railTargets)
        UnitTabKind.Vocabulary -> VocabularySection(items = unit.vocabulary)
        UnitTabKind.Practice -> PracticeSection(courseId = courseId, unit = unit, lesson = lesson)
    }
}

@Composable
private fun UnitTabNavigation(lesson: UnitLessonRecord, activeTab: UnitTabKind, onChoose: (UnitTabKind) -> Unit) {
    val availableTabs: List<UnitTabKind> = UnitTabKind.availableTabs(lesson)
    val position: Int = availableTabs.indexOf(activeTab)

    if (position < 0) {
        return
    }

    Row(modifier = Modifier.fillMaxWidth().padding(top = LanguageSpacing.spaceLg), verticalAlignment = Alignment.Top) {
        if (position > 0) {
            val previousTab: UnitTabKind = availableTabs[position - 1]

            TextLink(text = "← ${previousTab.label()}", onClick = { onChoose(previousTab) })
        }

        Spacer(modifier = Modifier.weight(1f))

        if (position + 1 < availableTabs.size) {
            val nextTab: UnitTabKind = availableTabs[position + 1]

            TextLink(text = "${nextTab.label()} →", onClick = { onChoose(nextTab) }, modifier = Modifier.testTag("next-tab"))
        }
    }
}

