package ro.zach.language.glossary

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.LazyListState
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.lazy.rememberLazyListState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.text.input.ImeAction
import ro.zach.language.app.LocalRootNavigation
import ro.zach.language.app.RootNavigationState
import ro.zach.language.app.UnitDestination
import ro.zach.language.audio.AudioClip
import ro.zach.language.audio.PlayButton
import ro.zach.language.core.CourseLevelRecord
import ro.zach.language.core.CourseOutlineRecord
import ro.zach.language.core.searchKey
import ro.zach.language.design.HairlineRule
import ro.zach.language.design.IndexRail
import ro.zach.language.design.LanguageColors
import ro.zach.language.design.LanguageSpacing
import ro.zach.language.design.LanguageText
import ro.zach.language.design.LanguageType
import ro.zach.language.design.LazyListIndexRailTracking
import ro.zach.language.design.LazyListScrollTargets
import ro.zach.language.design.PageHeader
import ro.zach.language.design.SquareSegmentedControl
import ro.zach.language.design.SquareTextField
import ro.zach.language.design.readableWidth
import ro.zach.language.design.rememberLazyListScrollTargets
import ro.zach.language.unit.LiteralGloss
import ro.zach.language.unit.UnitTabKind

private const val ALL_LEVELS_LABEL: String = "All levels"
private const val HEADER_ITEM_KEY: String = "glossary-header"
/* The header is the list's only item before the entries. */
private const val HEADER_ITEM_COUNT: Int = 1

@Composable
fun GlossaryPage(outline: CourseOutlineRecord, modifier: Modifier = Modifier) {
    var searchText: String by rememberSaveable { mutableStateOf("") }
    var levelFilter: String? by rememberSaveable { mutableStateOf(null) }
    val entries: List<GlossaryEntry> = remember(outline) { GlossaryEntry.indexing(outline) }
    val query: String = remember(searchText) { searchKey(searchText) }
    val matching: List<GlossaryEntry> = remember(entries, query, levelFilter) { matchingGlossaryEntries(entries, query, levelFilter) }
    val railKind: GlossaryRailKind = GlossaryRailKind.forLevelFilter(levelFilter)
    val glossaryRail: GlossaryRail = remember(matching, railKind) { GlossaryRail.dividing(matching, railKind) }
    val itemIndexByEntryNumber: Map<Int, Int> = remember(glossaryRail) { listItemIndexByEntryNumber(glossaryRail) }
    val listState: LazyListState = rememberLazyListState()
    val railTargets: LazyListScrollTargets = rememberLazyListScrollTargets(listState)

    LazyListIndexRailTracking(targets = railTargets, itemIndexByEntryNumber = itemIndexByEntryNumber)

    LaunchedEffect(glossaryRail.entries) {
        railTargets.tracker.recordContentChange()
    }

    Box(modifier = modifier.fillMaxSize()) {
        LazyColumn(
            state = listState,
            contentPadding = PaddingValues(bottom = LanguageSpacing.spaceMd),
            modifier = Modifier.fillMaxSize().testTag("glossary-entries"),
        ) {
            item(key = HEADER_ITEM_KEY) {
                GlossaryHeader(
                    outline = outline,
                    entryCount = matching.size,
                    searchText = searchText,
                    onSearchTextChange = { text: String -> searchText = text },
                    levelFilter = levelFilter,
                    onLevelFilterChange = { levelCode: String? -> levelFilter = levelCode },
                )
            }

            items(items = matching, key = { entry: GlossaryEntry -> entry.id }) { entry: GlossaryEntry ->
                Column(modifier = Modifier.readableWidth().padding(horizontal = LanguageSpacing.spaceMd)) {
                    GlossaryRow(courseId = outline.id, entry = entry)
                    HairlineRule()
                }
            }
        }

        IndexRail(
            entries = glossaryRail.entries,
            railAccessibilityLabel = railKind.railAccessibilityLabel(),
            railTestTag = railKind.railTestTag(),
            tracker = railTargets.tracker,
            scrollTarget = railTargets,
            modifier = Modifier.align(Alignment.CenterEnd),
        )
    }
}

private fun listItemIndexByEntryNumber(glossaryRail: GlossaryRail): Map<Int, Int> {
    return glossaryRail.firstEntryIndexByEntryNumber().mapValues { (_, entryIndex: Int) -> entryIndex + HEADER_ITEM_COUNT }
}

@Composable
private fun GlossaryHeader(
    outline: CourseOutlineRecord,
    entryCount: Int,
    searchText: String,
    onSearchTextChange: (String) -> Unit,
    levelFilter: String?,
    onLevelFilterChange: (String?) -> Unit,
) {
    val levelOptions: List<String?> = listOf<String?>(null) + outline.levels.map { level: CourseLevelRecord -> level.code }

    Column(
        modifier = Modifier
            .readableWidth()
            .padding(start = LanguageSpacing.spaceMd, top = LanguageSpacing.spaceMd, end = LanguageSpacing.spaceMd),
        verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceMd),
    ) {
        PageHeader(caption = "$entryCount entries", title = "Glossary")

        SquareTextField(
            value = searchText,
            onValueChange = onSearchTextChange,
            placeholder = "Search ${outline.targetLanguageName} or ${outline.sourceLanguageName}",
            imeAction = ImeAction.Search,
        )

        SquareSegmentedControl(
            options = levelOptions,
            selection = levelFilter,
            label = { levelCode: String? -> levelCode ?: ALL_LEVELS_LABEL },
            testTag = { levelCode: String? -> "glossary-level-filter-${levelCode ?: "all"}" },
            onSelect = onLevelFilterChange,
        )

        HairlineRule()
    }
}

@Composable
private fun GlossaryRow(courseId: String, entry: GlossaryEntry) {
    val rootNavigation: RootNavigationState = LocalRootNavigation.current
    val sourceLiteral: String? = entry.item.sourceLiteral

    Row(
        modifier = Modifier.fillMaxWidth().padding(vertical = LanguageSpacing.spaceSm),
        horizontalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm),
        verticalAlignment = Alignment.Top,
    ) {
        PlayButton(clip = AudioClip(clipKey = entry.item.clipKey, text = entry.item.target))

        Column(modifier = Modifier.weight(1f), verticalArrangement = Arrangement.spacedBy(LanguageSpacing.phraseLineGap)) {
            LanguageText(text = entry.item.target, style = LanguageType.targetText)
            LanguageText(text = entry.item.source, style = LanguageType.bodyText, color = LanguageColors.inkMuted)

            if (sourceLiteral != null) {
                LiteralGloss(literal = sourceLiteral)
            }
        }

        LanguageText(
            text = entry.unitLabel(),
            style = LanguageType.dataText,
            color = LanguageColors.accentLink,
            modifier = Modifier.clickable(interactionSource = null, indication = null, role = Role.Button) {
                rootNavigation.openUnit(UnitDestination(courseId = courseId, unitId = entry.unit.id, tab = UnitTabKind.Vocabulary))
            },
        )
    }
}
