package ro.zach.language.review

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.key
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.text.font.FontStyle
import ro.zach.language.audio.AudioClip
import ro.zach.language.audio.AudioPlayer
import ro.zach.language.audio.LocalAudioPlayer
import ro.zach.language.audio.PlayButton
import ro.zach.language.audio.PlaybackControlKind
import ro.zach.language.core.VocabularyItemRecord
import ro.zach.language.design.LanguageColors
import ro.zach.language.design.LanguageSpacing
import ro.zach.language.design.LanguageText
import ro.zach.language.design.LanguageType
import ro.zach.language.design.MicroLabel
import ro.zach.language.design.SquareProgressBar
import ro.zach.language.design.SquareTextButton
import ro.zach.language.design.TextLink
import ro.zach.language.design.panel
import ro.zach.language.progressdata.LocalProgressStore
import ro.zach.language.progressdata.ProgressStore

/** How the learner grades a card once its answer shows. */
enum class ReviewGradeKind {
    Forgot,
    Remembered,
}

@Composable
fun ReviewSession(courseId: String, vocabularyIndex: VocabularyIndex, items: List<ReviewItem>, onExit: () -> Unit) {
    val progressStore: ProgressStore = LocalProgressStore.current
    val audioPlayer: AudioPlayer = LocalAudioPlayer.current
    var currentIndex: Int by rememberSaveable { mutableIntStateOf(0) }
    var isRevealed: Boolean by rememberSaveable { mutableStateOf(false) }
    var rememberedCount: Int by rememberSaveable { mutableIntStateOf(0) }
    val showNextCard: () -> Unit = {
        isRevealed = false
        currentIndex += 1
    }
    val grade: (ReviewGradeKind) -> Unit = { gradeKind: ReviewGradeKind ->
        progressStore.gradeCard(courseId, items[currentIndex].cardId, gradeKind == ReviewGradeKind.Remembered)

        if (gradeKind == ReviewGradeKind.Remembered) {
            rememberedCount += 1
        }

        audioPlayer.stop()
        showNextCard()
    }

    if (currentIndex >= items.size) {
        ReviewSessionSummary(rememberedCount = rememberedCount, cardCount = items.size, onDone = onExit)
        return
    }

    val item: ReviewItem = items[currentIndex]
    val entry: VocabularyIndexEntry? = vocabularyIndex.entry(item.cardId)

    key(currentIndex) {
        if (entry != null) {
            ReviewCardView(
                entry = entry,
                promptSide = item.promptSide,
                position = currentIndex,
                total = items.size,
                isRevealed = isRevealed,
                onReveal = { isRevealed = true },
                onGrade = grade,
                onExit = onExit,
            )
        } else {
            MissingCard(onSkip = showNextCard)
        }
    }
}

@Composable
private fun ReviewSessionSummary(rememberedCount: Int, cardCount: Int, onDone: () -> Unit) {
    Column(verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceMd)) {
        LanguageText(text = "Session complete", style = LanguageType.titleText, modifier = Modifier.testTag("review-complete"))
        LanguageText(text = "You remembered $rememberedCount of $cardCount.", style = LanguageType.bodyText)
        SquareTextButton(text = "Done", onClick = onDone, isPrimary = true)
    }
}

@Composable
private fun MissingCard(onSkip: () -> Unit) {
    Column(verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceMd)) {
        LanguageText(text = "This card refers to a word that is no longer in the course.", style = LanguageType.bodyText, color = LanguageColors.accentActive)
        SquareTextButton(text = "Skip", onClick = onSkip)
    }
}

@Composable
private fun ReviewCardView(
    entry: VocabularyIndexEntry,
    promptSide: ReviewCardSide,
    position: Int,
    total: Int,
    isRevealed: Boolean,
    onReveal: () -> Unit,
    onGrade: (ReviewGradeKind) -> Unit,
    onExit: () -> Unit,
) {
    val audioPlayer: AudioPlayer = LocalAudioPlayer.current
    val progressStore: ProgressStore = LocalProgressStore.current
    val item: VocabularyItemRecord = entry.item

    /* The target plays whenever it shows: with the prompt, or with the answer when the prompt is the source. */
    LaunchedEffect(isRevealed) {
        if (promptSide == ReviewCardSide.Target || isRevealed) {
            audioPlayer.play(AudioClip(clipKey = item.clipKey, text = item.target), progressStore.settings().audioEngine, false, PlaybackControlKind.NormalSpeed)
        }
    }

    Column(verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceMd)) {
        Row(modifier = Modifier.fillMaxWidth(), verticalAlignment = Alignment.CenterVertically) {
            LanguageText(text = "Card ${position + 1} of $total", style = LanguageType.captionText, color = LanguageColors.inkMuted, modifier = Modifier.weight(1f))
            TextLink(text = "Quit", onClick = onExit, modifier = Modifier.testTag("quit"))
        }

        SquareProgressBar(value = position, total = total)

        Column(modifier = Modifier.panel(), verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceMd)) {
            MicroLabel(text = entry.unit.title)
            LanguageText(
                text = if (promptSide == ReviewCardSide.Target) item.target else item.source,
                style = LanguageType.headingText,
                modifier = Modifier.testTag("flashcard-prompt"),
            )

            if (isRevealed) {
                ReviewAnswer(item = item, promptSide = promptSide, onGrade = onGrade)
            } else {
                SquareTextButton(text = "Show answer", onClick = onReveal, isPrimary = true, modifier = Modifier.testTag("show-answer"))
            }
        }
    }
}

@Composable
private fun ReviewAnswer(item: VocabularyItemRecord, promptSide: ReviewCardSide, onGrade: (ReviewGradeKind) -> Unit) {
    val literal: String? = item.sourceLiteral
    val note: String? = item.note

    Column(verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm)) {
        Row(horizontalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm), verticalAlignment = Alignment.CenterVertically) {
            PlayButton(clip = AudioClip(clipKey = item.clipKey, text = item.target))
            LanguageText(text = if (promptSide == ReviewCardSide.Target) item.source else item.target, style = LanguageType.titleText)
        }

        if (literal != null) {
            LanguageText(
                text = "literally: $literal",
                style = LanguageType.captionText.copy(fontStyle = FontStyle.Italic),
                color = LanguageColors.inkMuted,
            )
        }

        Row(horizontalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm), verticalAlignment = Alignment.Top) {
            PlayButton(clip = AudioClip(clipKey = item.example.clipKey, text = item.example.target))

            Column(verticalArrangement = Arrangement.spacedBy(LanguageSpacing.phraseLineGap)) {
                LanguageText(text = item.example.target, style = LanguageType.bodyText)
                LanguageText(text = item.example.source, style = LanguageType.captionText, color = LanguageColors.inkMuted)
            }
        }

        if (note != null) {
            LanguageText(text = note, style = LanguageType.captionText, color = LanguageColors.inkMuted)
        }

        Row(horizontalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm)) {
            SquareTextButton(text = "Forgot", onClick = { onGrade(ReviewGradeKind.Forgot) }, modifier = Modifier.testTag("forgot"))
            SquareTextButton(text = "Remembered", onClick = { onGrade(ReviewGradeKind.Remembered) }, isPrimary = true, modifier = Modifier.testTag("remembered"))
        }
    }
}
