package ro.zach.language.app

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.WindowInsets
import androidx.compose.foundation.gestures.awaitEachGesture
import androidx.compose.foundation.gestures.awaitFirstDown
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.offset
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.safeDrawing
import androidx.compose.foundation.layout.windowInsetsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.saveable.SaveableStateHolder
import androidx.compose.runtime.saveable.rememberSaveableStateHolder
import androidx.compose.ui.Alignment
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.selected
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.semantics.testTagsAsResourceId
import androidx.compose.ui.text.font.FontWeight
import ro.zach.language.audio.AudioPlayer
import ro.zach.language.audio.LocalAudioPlayer
import ro.zach.language.core.CourseOutlineRecord
import ro.zach.language.coursedata.CourseIndexLoadState
import ro.zach.language.coursedata.CoursePublicationLoadState
import ro.zach.language.coursedata.CourseStore
import ro.zach.language.coursedata.LocalCourseStore
import ro.zach.language.coursedata.OutlineLoadState
import ro.zach.language.coursedata.RepositoryIndexLoadKind
import ro.zach.language.courses.CoursesPage
import ro.zach.language.design.Glyph
import ro.zach.language.design.GlyphKind
import ro.zach.language.design.HairlineRule
import ro.zach.language.design.LanguageColors
import ro.zach.language.design.LanguageSpacing
import ro.zach.language.design.LanguageText
import ro.zach.language.design.LanguageType
import ro.zach.language.design.PendingActivity
import ro.zach.language.design.PendingStatusMessage
import ro.zach.language.design.SquareButtonPendingKind
import ro.zach.language.design.SquareTextButton
import ro.zach.language.design.StatusMessage
import ro.zach.language.design.pendingAccessibility
import ro.zach.language.design.rememberPendingActivity
import ro.zach.language.glossary.GlossaryPage
import ro.zach.language.home.HomePage
import ro.zach.language.progressdata.LocalProgressStore
import ro.zach.language.progressdata.ProgressStore
import ro.zach.language.progresspage.ProgressPage
import ro.zach.language.review.ReviewPage
import ro.zach.language.settings.SettingsSheet
import ro.zach.language.unit.UnitPage

private const val LOAD_FAILURE_PENDING_STATE_DESCRIPTION: String = "Loading"
private const val COURSE_LOADING_MESSAGE: String = "Loading the course…"

/** The tabs, each with its own stack of opened units, under a top bar, with the settings and course list over them. */
@OptIn(ExperimentalComposeUiApi::class)
@Composable
fun RootScreen(courseStore: CourseStore, progressStore: ProgressStore, audioPlayer: AudioPlayer, rootNavigation: RootNavigationState) {
    CompositionLocalProvider(
        LocalCourseStore provides courseStore,
        LocalProgressStore provides progressStore,
        LocalAudioPlayer provides audioPlayer,
        LocalRootNavigation provides rootNavigation,
    ) {
        val saveableStateHolder: SaveableStateHolder = rememberSaveableStateHolder()
        /* A unit left is never shown again, so its saved state is dropped with it. */
        val goBack: () -> Unit = {
            val outcome: BackOutcome = rootNavigation.goBack()

            if (outcome is BackOutcome.LeftUnit) {
                saveableStateHolder.removeState(unitStateKey(outcome.entry))
            }
        }

        BackHandler(enabled = rootNavigation.canGoBack(), onBack = goBack)

        Box(
            modifier = Modifier
                .fillMaxSize()
                .background(LanguageColors.paper)
                .windowInsetsPadding(WindowInsets.safeDrawing)
                .semantics { testTagsAsResourceId = true },
        ) {
            RootTabs(saveableStateHolder = saveableStateHolder, onBack = goBack)

            when (rootNavigation.overlay()) {
                RootOverlayKind.Settings -> OverlayLayer(onDone = rootNavigation::dismissOverlay) {
                    SettingsSheet(courseId = activeCourseId())
                }
                RootOverlayKind.Courses -> OverlayLayer(onDone = rootNavigation::dismissOverlay) {
                    CoursesPage(onChoose = rootNavigation::dismissOverlay)
                }
                RootOverlayKind.None -> Unit
            }
        }
    }
}

@Composable
private fun activeCourseId(): String? {
    return LocalCourseStore.current.activeCourseId(LocalProgressStore.current.lastCourseId())
}

private fun unitStateKey(entry: UnitStackEntry): String {
    return "unit ${entry.entryId}"
}

private fun tabRootStateKey(tab: RootTabKind): String {
    return "tab ${tab.name}"
}

@Composable
private fun RootTabs(saveableStateHolder: SaveableStateHolder, onBack: () -> Unit) {
    val rootNavigation: RootNavigationState = LocalRootNavigation.current
    val selectedTab: RootTabKind = rootNavigation.selectedTab()
    val shownEntry: UnitStackEntry? = rootNavigation.unitStack(selectedTab).lastOrNull()
    val shownUnit: UnitDestination? = shownEntry?.destination
    val courseId: String? = activeCourseId()

    Column(modifier = Modifier.fillMaxSize()) {
        RootTopBar(shownUnit = shownUnit, courseId = courseId, onBack = onBack)
        HairlineRule()

        Box(modifier = Modifier.weight(1f).fillMaxWidth()) {
            /* Each tab's root, and each unit of its stack, keeps its scroll position and state while another is shown. */
            if (shownEntry == null) {
                saveableStateHolder.SaveableStateProvider(key = tabRootStateKey(selectedTab)) {
                    TabRoot(tab = selectedTab, courseId = courseId)
                }
            } else {
                saveableStateHolder.SaveableStateProvider(key = unitStateKey(shownEntry)) {
                    UnitPage(courseId = shownEntry.destination.courseId, unitId = shownEntry.destination.unitId, initialTab = shownEntry.destination.tab)
                }
            }
        }

        HairlineRule()
        RootBottomBar(selectedTab = selectedTab, courseId = courseId)
    }
}

@Composable
private fun TabRoot(tab: RootTabKind, courseId: String?) {
    CourseIndexGate {
        if (courseId == null) {
            CoursesPage(onChoose = {})
        } else {
            CourseGate(courseId = courseId) { outline: CourseOutlineRecord ->
                when (tab) {
                    RootTabKind.Course -> HomePage(outline = outline)
                    RootTabKind.Review -> ReviewPage(outline = outline)
                    RootTabKind.Glossary -> GlossaryPage(outline = outline)
                    RootTabKind.Progress -> ProgressPage(outline = outline)
                }
            }
        }
    }
}

@Composable
private fun RootTopBar(shownUnit: UnitDestination?, courseId: String?, onBack: () -> Unit) {
    val courseStore: CourseStore = LocalCourseStore.current
    val rootNavigation: RootNavigationState = LocalRootNavigation.current
    val courseTitle: String? = courseId?.let { id -> courseStore.courses().firstOrNull { course -> course.id == id }?.title }

    Row(
        modifier = Modifier
            .fillMaxWidth()
            .height(LanguageSpacing.spaceXl)
            .padding(horizontal = LanguageSpacing.spaceSm),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Box(modifier = Modifier.weight(1f).fillMaxHeight(), contentAlignment = Alignment.CenterStart) {
            if (shownUnit != null) {
                TopBarButton(testTag = "back", contentDescription = "Back", onClick = onBack) {
                    Glyph(glyph = GlyphKind.Back, color = LanguageColors.accentLink)
                }
            } else if (courseTitle != null) {
                TopBarButton(testTag = "courses", contentDescription = null, onClick = { rootNavigation.present(RootOverlayKind.Courses) }) {
                    LanguageText(text = courseTitle, style = LanguageType.bodyText, color = LanguageColors.accentLink, maxLines = 1)
                }
            }
        }

        if (shownUnit == null) {
            TopBarButton(testTag = "settings", contentDescription = "Settings", onClick = { rootNavigation.present(RootOverlayKind.Settings) }) {
                Glyph(glyph = GlyphKind.Settings, color = LanguageColors.accentLink, size = LanguageSpacing.spaceMd + LanguageSpacing.spaceXs)
            }
        }
    }
}

@Composable
private fun TopBarButton(testTag: String, contentDescription: String?, onClick: () -> Unit, content: @Composable () -> Unit) {
    Box(
        modifier = Modifier
            .fillMaxHeight()
            .clickable(interactionSource = null, indication = null, role = Role.Button, onClick = onClick)
            .semantics {
                if (contentDescription != null) {
                    this.contentDescription = contentDescription
                }
            }
            .testTag(testTag)
            .padding(horizontal = LanguageSpacing.spaceSm),
        contentAlignment = Alignment.Center,
    ) {
        content()
    }
}

@Composable
private fun RootBottomBar(selectedTab: RootTabKind, courseId: String?) {
    val progressStore: ProgressStore = LocalProgressStore.current
    val rootNavigation: RootNavigationState = LocalRootNavigation.current
    val dueCount: Int = if (courseId == null) 0 else progressStore.dueCardCount(courseId)

    Row(modifier = Modifier.fillMaxWidth()) {
        for (tab in RootTabKind.entries) {
            val isSelected: Boolean = tab == selectedTab
            val color: Color = if (isSelected) LanguageColors.accentActive else LanguageColors.ink
            val badgeCount: Int = if (tab == RootTabKind.Review) dueCount else 0

            Column(
                modifier = Modifier
                    .weight(1f)
                    .clickable(interactionSource = null, indication = null, role = Role.Tab) { rootNavigation.select(tab) }
                    .semantics { selected = isSelected }
                    .testTag(tab.testTag())
                    .padding(vertical = LanguageSpacing.spaceSm),
                horizontalAlignment = Alignment.CenterHorizontally,
                verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceXs),
            ) {
                Box {
                    Glyph(glyph = tab.glyph(), color = color, size = LanguageSpacing.spaceMd + LanguageSpacing.spaceXs)

                    if (badgeCount > 0) {
                        TabBadge(count = badgeCount, modifier = Modifier.align(Alignment.TopEnd).offset(x = LanguageSpacing.spaceSm, y = -LanguageSpacing.spaceXs))
                    }
                }

                LanguageText(text = tab.label(), style = LanguageType.labelText, color = color)
            }
        }
    }
}

@Composable
private fun TabBadge(count: Int, modifier: Modifier = Modifier) {
    LanguageText(
        text = "$count",
        style = LanguageType.indexEntryText,
        color = LanguageColors.paper,
        modifier = modifier
            .background(LanguageColors.accentActive)
            .padding(horizontal = LanguageSpacing.spaceXs),
    )
}

/** A full-screen layer over the tabs, with a Done that closes it. */
@Composable
private fun OverlayLayer(onDone: () -> Unit, content: @Composable () -> Unit) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .background(LanguageColors.paper)
            /* Takes every press, so none reaches the tabs under it. */
            .pointerInput(Unit) {
                awaitEachGesture {
                    awaitFirstDown(requireUnconsumed = false)
                }
            },
    ) {
        Box(
            modifier = Modifier
                .fillMaxWidth()
                .height(LanguageSpacing.spaceXl)
                .padding(horizontal = LanguageSpacing.spaceSm),
        ) {
            Box(modifier = Modifier.align(Alignment.CenterEnd).fillMaxHeight(), contentAlignment = Alignment.CenterEnd) {
                TopBarButton(testTag = "done", contentDescription = null, onClick = onDone) {
                    LanguageText(text = "Done", style = LanguageType.bodyText.copy(fontWeight = FontWeight.SemiBold), color = LanguageColors.accentLink)
                }
            }
        }

        HairlineRule()

        Box(modifier = Modifier.weight(1f).fillMaxWidth()) {
            content()
        }
    }
}

/** Shows its content once a course index has loaded, and a status message before that. */
@Composable
fun CourseIndexGate(content: @Composable () -> Unit) {
    val courseStore: CourseStore = LocalCourseStore.current

    when (val indexState: CourseIndexLoadState = courseStore.indexLoadState()) {
        CourseIndexLoadState.Loading -> PendingStatusMessage(text = "Loading the courses…")
        is CourseIndexLoadState.Failed -> LoadFailure(message = indexState.message, onRetry = { courseStore.refresh(courseId = null) })
        is CourseIndexLoadState.Loaded -> content()
    }
}

/** Shows its content with the course's publication and outline, loading them if needed. */
@Composable
fun CourseGate(courseId: String, content: @Composable (CourseOutlineRecord) -> Unit) {
    val courseStore: CourseStore = LocalCourseStore.current
    val progressStore: ProgressStore = LocalProgressStore.current
    val shownContentKey: String? = courseStore.shownContentKey(courseId)

    LaunchedEffect(courseId) {
        courseStore.ensurePublicationLoaded(courseId)
        progressStore.rememberCourse(courseId)
    }

    LaunchedEffect(courseId, shownContentKey) {
        courseStore.ensureOutlineLoaded(courseId)
    }

    if (!courseStore.listsCourse(courseId)) {
        MissingCourse()
        return
    }

    when (val publicationState: CoursePublicationLoadState? = courseStore.publicationState(courseId)) {
        is CoursePublicationLoadState.Loaded -> OutlineContent(courseId = courseId, content = content)
        is CoursePublicationLoadState.Failed -> LoadFailure(message = publicationState.message, onRetry = { courseStore.retry(courseId) })
        CoursePublicationLoadState.Loading, null -> PendingStatusMessage(text = COURSE_LOADING_MESSAGE)
    }
}

/* Only the repository's course index can tell that a course does not exist; a cached or included one may be behind. */
@Composable
private fun MissingCourse() {
    when (LocalCourseStore.current.repositoryIndexLoadState()) {
        RepositoryIndexLoadKind.Pending -> PendingStatusMessage(text = COURSE_LOADING_MESSAGE)
        RepositoryIndexLoadKind.Loaded, RepositoryIndexLoadKind.Failed -> StatusMessage(text = "That course does not exist.")
    }
}

@Composable
private fun OutlineContent(courseId: String, content: @Composable (CourseOutlineRecord) -> Unit) {
    val courseStore: CourseStore = LocalCourseStore.current

    when (val outlineState: OutlineLoadState? = courseStore.outlineState(courseId)) {
        is OutlineLoadState.Loaded -> content(outlineState.outline)
        is OutlineLoadState.Failed -> LoadFailure(message = outlineState.message, onRetry = { courseStore.retry(courseId) })
        OutlineLoadState.Loading, null -> PendingStatusMessage(text = COURSE_LOADING_MESSAGE)
    }
}

@Composable
private fun LoadFailure(message: String, onRetry: suspend () -> Unit) {
    val retry: PendingActivity = rememberPendingActivity()
    val isPending: Boolean = retry.isIndicated()

    Column(
        modifier = Modifier.fillMaxSize().padding(LanguageSpacing.spaceMd),
        verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceMd),
    ) {
        LanguageText(text = "The course could not be loaded.", style = LanguageType.titleText, color = LanguageColors.accentActive)
        LanguageText(text = message, style = LanguageType.captionText, color = LanguageColors.inkMuted)
        SquareTextButton(
            text = "Try again",
            isPrimary = true,
            pending = if (isPending) SquareButtonPendingKind.PendingInPlaceOfLabel else SquareButtonPendingKind.NotPending,
            onClick = { retry.run(onRetry) },
            modifier = Modifier
                .pendingAccessibility(isIndicated = isPending, value = LOAD_FAILURE_PENDING_STATE_DESCRIPTION)
                .testTag("try-again"),
        )
    }
}
