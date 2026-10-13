package ro.zach.language.review

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import ro.zach.language.core.CourseOutlineRecord
import ro.zach.language.core.ReviewDirectionRecord
import ro.zach.language.design.LanguageColors
import ro.zach.language.design.LanguageSpacing
import ro.zach.language.design.LanguageText
import ro.zach.language.design.LanguageType
import ro.zach.language.design.MicroLabel
import ro.zach.language.design.PageHeader
import ro.zach.language.design.SquareDropDown
import ro.zach.language.design.SquareTextButton
import ro.zach.language.design.readableWidth
import ro.zach.language.progressdata.LocalProgressStore
import ro.zach.language.progressdata.ProgressStore

private val reviewDirections: List<ReviewDirectionRecord> = listOf(
    ReviewDirectionRecord.TARGET_TO_SOURCE,
    ReviewDirectionRecord.SOURCE_TO_TARGET,
    ReviewDirectionRecord.MIXED,
)

@Composable
fun ReviewPage(outline: CourseOutlineRecord, modifier: Modifier = Modifier) {
    val progressStore: ProgressStore = LocalProgressStore.current
    val vocabularyIndex: VocabularyIndex = remember(outline) { VocabularyIndex(outline) }
    var sessionItems: List<ReviewItem>? by rememberSaveable { mutableStateOf(null) }
    val shownSessionItems: List<ReviewItem>? = sessionItems

    Column(modifier = modifier.fillMaxSize().verticalScroll(rememberScrollState())) {
        Column(
            modifier = Modifier.readableWidth().padding(LanguageSpacing.spaceMd),
            verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceMd),
        ) {
            if (shownSessionItems != null) {
                ReviewSession(courseId = outline.id, vocabularyIndex = vocabularyIndex, items = shownSessionItems, onExit = { sessionItems = null })
            } else {
                ReviewSetup(
                    outline = outline,
                    onStart = {
                        val cardIds: List<String> = progressStore.dueCardIds(outline.id, REVIEW_SESSION_CARD_LIMIT)

                        sessionItems = buildReviewItems(cardIds, progressStore.settings().reviewDirection)
                    },
                )
            }
        }
    }
}

@Composable
private fun ReviewSetup(outline: CourseOutlineRecord, onStart: () -> Unit) {
    val progressStore: ProgressStore = LocalProgressStore.current
    val dueCount: Int = progressStore.dueCardCount(outline.id)
    val laterCount: Int = maxOf(progressStore.cardCount(outline.id) - dueCount, 0)

    Column(verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceMd)) {
        PageHeader(caption = null, title = "Review")

        Column(verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceXs)) {
            LanguageText(text = "$dueCount", style = LanguageType.figureText, modifier = Modifier.testTag("due-count"))
            LanguageText(text = "words ready to review now", style = LanguageType.bodyText)
            LanguageText(text = "$laterCount scheduled for later", style = LanguageType.dataText, color = LanguageColors.inkMuted)
        }

        LanguageText(
            text = "A word is ready when its next review time has come. Words you remember come back after a longer wait each time; " +
                "words you forget come back in a minute. The number on the Review tab is the count of words ready now.",
            style = LanguageType.captionText,
            color = LanguageColors.inkMuted,
        )

        DirectionPicker(outline = outline)

        if (dueCount == 0) {
            LanguageText(
                text = "No words are ready. Add a unit's words to your deck from its overview, or pass its practice; " +
                    "words you have already reviewed return when their wait is over.",
                style = LanguageType.captionText,
                color = LanguageColors.inkMuted,
            )
        } else {
            SquareTextButton(text = "Start review", onClick = onStart, isPrimary = true, modifier = Modifier.testTag("start-review"))
        }
    }
}

@Composable
private fun DirectionPicker(outline: CourseOutlineRecord) {
    val progressStore: ProgressStore = LocalProgressStore.current

    Column(verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceXs)) {
        MicroLabel(text = "Direction")
        SquareDropDown(
            options = reviewDirections,
            selection = progressStore.settings().reviewDirection,
            label = { direction: ReviewDirectionRecord -> directionLabel(direction, outline) },
            testTag = { direction: ReviewDirectionRecord -> "direction-${direction.name.lowercase()}" },
            onSelect = { direction: ReviewDirectionRecord -> progressStore.updateSettings { settings -> settings.copy(reviewDirection = direction) } },
            modifier = Modifier.testTag("review-direction"),
        )
    }
}

private fun directionLabel(direction: ReviewDirectionRecord, outline: CourseOutlineRecord): String {
    return when (direction) {
        ReviewDirectionRecord.TARGET_TO_SOURCE -> "${outline.targetLanguageName} to ${outline.sourceLanguageName}"
        ReviewDirectionRecord.SOURCE_TO_TARGET -> "${outline.sourceLanguageName} to ${outline.targetLanguageName}"
        ReviewDirectionRecord.MIXED -> "Mixed"
    }
}
