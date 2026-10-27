package ro.zach.language.home

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.BoxScope
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.pulltorefresh.PullToRefreshBox
import androidx.compose.material3.pulltorefresh.PullToRefreshState
import androidx.compose.material3.pulltorefresh.rememberPullToRefreshState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.text.style.TextAlign
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.launch
import ro.zach.language.app.LocalRootNavigation
import ro.zach.language.app.RootNavigationState
import ro.zach.language.app.UnitDestination
import ro.zach.language.core.CourseLevelRecord
import ro.zach.language.core.CourseOutlineRecord
import ro.zach.language.core.UnitOutlineRecord
import ro.zach.language.core.UnitStatusRecord
import ro.zach.language.coursedata.CourseStore
import ro.zach.language.coursedata.LocalCourseStore
import ro.zach.language.design.Glyph
import ro.zach.language.design.GlyphKind
import ro.zach.language.design.HairlineRule
import ro.zach.language.design.LanguageColors
import ro.zach.language.design.LanguageSpacing
import ro.zach.language.design.LanguageText
import ro.zach.language.design.LanguageType
import ro.zach.language.design.PageHeader
import ro.zach.language.design.SquareTextButton
import ro.zach.language.design.UnitStatusTag
import ro.zach.language.design.readableWidth
import ro.zach.language.progressdata.LocalProgressStore
import ro.zach.language.progressdata.ProgressStore
import ro.zach.language.unit.UnitTabKind

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun HomePage(outline: CourseOutlineRecord, modifier: Modifier = Modifier) {
    val courseStore: CourseStore = LocalCourseStore.current
    val pullToRefreshState: PullToRefreshState = rememberPullToRefreshState()
    val scope: CoroutineScope = rememberCoroutineScope()
    var isRefreshing: Boolean by remember { mutableStateOf(false) }

    PullToRefreshBox(
        isRefreshing = isRefreshing,
        onRefresh = {
            isRefreshing = true
            scope.launch {
                try {
                    courseStore.refresh(outline.id)
                } finally {
                    isRefreshing = false
                }
            }
        },
        state = pullToRefreshState,
        indicator = { PullToRefreshBar(state = pullToRefreshState, isRefreshing = isRefreshing) },
        modifier = modifier.fillMaxSize(),
    ) {
        Column(modifier = Modifier.fillMaxSize().verticalScroll(rememberScrollState())) {
            Column(
                modifier = Modifier.readableWidth().padding(LanguageSpacing.spaceMd),
                verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceLg),
            ) {
                HomeHeader(outline)

                for (level in outline.levels) {
                    LevelSection(outline = outline, level = level, units = outline.units.filter { unit -> unit.levelCode == level.code })
                }
            }
        }
    }
}

/* Progress is a thin red bar over a hairline; there are no spinners. */
@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun BoxScope.PullToRefreshBar(state: PullToRefreshState, isRefreshing: Boolean) {
    val proportion: Float = if (isRefreshing) 1f else state.distanceFraction.coerceIn(0f, 1f)

    if (proportion == 0f) {
        return
    }

    Box(
        modifier = Modifier
            .align(Alignment.TopCenter)
            .fillMaxWidth()
            .height(LanguageSpacing.selectedBorderWidth)
            .background(LanguageColors.rule),
    ) {
        Box(
            modifier = Modifier
                .fillMaxHeight()
                .fillMaxWidth(proportion)
                .background(LanguageColors.accentActive),
        )
    }
}

@Composable
private fun HomeHeader(outline: CourseOutlineRecord) {
    val progressStore: ProgressStore = LocalProgressStore.current
    val rootNavigation: RootNavigationState = LocalRootNavigation.current
    val nextUnit: UnitOutlineRecord? = outline.units.firstOrNull { unit ->
        progressStore.unitProgress(outline.id, unit.id).status != UnitStatusRecord.COMPLETED
    }

    Column(verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm)) {
        PageHeader(caption = "${outline.units.size} units", title = outline.title)

        Row(verticalAlignment = Alignment.CenterVertically) {
            LanguageText(
                text = "${progressStore.completedUnitCount(outline.id)} / ${outline.units.size} units completed",
                style = LanguageType.dataText,
                color = LanguageColors.inkMuted,
            )

            Spacer(modifier = Modifier.weight(1f))

            if (nextUnit != null) {
                SquareTextButton(
                    text = "Continue",
                    isPrimary = true,
                    onClick = { rootNavigation.openUnit(UnitDestination(outline.id, nextUnit.id, UnitTabKind.Overview)) },
                    modifier = Modifier.testTag("continue"),
                )
            }
        }
    }
}

@Composable
private fun LevelSection(outline: CourseOutlineRecord, level: CourseLevelRecord, units: List<UnitOutlineRecord>) {
    val rootNavigation: RootNavigationState = LocalRootNavigation.current

    Column {
        LanguageText(text = "${level.code} · ${level.title}", style = LanguageType.titleText, modifier = Modifier.padding(bottom = LanguageSpacing.spaceSm))

        HairlineRule()

        for (unit in units) {
            UnitRow(
                outline = outline,
                unit = unit,
                onOpen = { rootNavigation.openUnit(UnitDestination(outline.id, unit.id, UnitTabKind.Overview)) },
            )

            HairlineRule()
        }
    }
}

@Composable
private fun UnitRow(outline: CourseOutlineRecord, unit: UnitOutlineRecord, onOpen: () -> Unit) {
    val progressStore: ProgressStore = LocalProgressStore.current

    Row(
        modifier = Modifier
            .fillMaxWidth()
            .clickable(interactionSource = null, indication = null, role = Role.Button, onClick = onOpen)
            .testTag("unit-${unit.id}")
            .padding(vertical = LanguageSpacing.spaceSm + LanguageSpacing.spaceXs),
        horizontalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        LanguageText(
            text = "${unit.order}",
            style = LanguageType.dataText,
            color = LanguageColors.inkMuted,
            textAlign = TextAlign.End,
            modifier = Modifier.width(LanguageSpacing.unitOrderWidth),
        )

        Column(modifier = Modifier.weight(1f), verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceXs)) {
            LanguageText(text = unit.title, style = LanguageType.bodyText, color = LanguageColors.accentLink)
            UnitStatusTag(status = progressStore.unitProgress(outline.id, unit.id).status)
        }

        Glyph(glyph = GlyphKind.Chevron, color = LanguageColors.rule, size = LanguageSpacing.spaceSm + LanguageSpacing.spaceXs)
    }
}
