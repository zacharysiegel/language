package ro.zach.language.practice

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotEquals
import org.junit.Assert.assertNull
import org.junit.Test
import ro.zach.language.core.ExerciseRecord
import ro.zach.language.core.PhrasePairRecord
import kotlin.random.Random

private const val CLIP_KEY: String = "clip"
private const val EXPLANATION: String = "why"
private const val SHUFFLE_SEED_COUNT: Int = 200

class ExerciseRecordsTest {
    private val pair: PhrasePairRecord = PhrasePairRecord(target = "Oi", source = "Hi", sourceLiteral = null, clipKey = CLIP_KEY)
    private val multipleChoice: ExerciseRecord = ExerciseRecord.MultipleChoice(prompt = "p", choices = listOf("a", "b"), answerIndex = 0u, explanation = EXPLANATION)
    private val fillBlank: ExerciseRecord = ExerciseRecord.FillBlank(promptTarget = "p", answers = listOf("a"), hintSource = "h", explanation = EXPLANATION, clipKey = CLIP_KEY)
    private val translateToTarget: ExerciseRecord = ExerciseRecord.TranslateToTarget(promptSource = "p", answers = listOf("a"), explanation = EXPLANATION, clipKey = CLIP_KEY)
    private val orderWords: ExerciseRecord = ExerciseRecord.OrderWords(promptSource = "p", words = listOf("a", "b"), distractors = emptyList(), explanation = EXPLANATION, clipKey = CLIP_KEY)
    private val listeningChoice: ExerciseRecord = ExerciseRecord.ListeningChoice(audioTarget = "t", choices = listOf("a"), answerIndex = 0u, explanation = EXPLANATION, clipKey = CLIP_KEY)
    private val dictation: ExerciseRecord = ExerciseRecord.Dictation(audioTarget = "t", hintSource = "h", explanation = EXPLANATION, clipKey = CLIP_KEY)
    private val match: ExerciseRecord = ExerciseRecord.Match(prompt = null, pairs = listOf(pair), explanation = EXPLANATION)
    private val everyKind: List<ExerciseRecord> = listOf(multipleChoice, fillBlank, translateToTarget, orderWords, listeningChoice, dictation, match)

    @Test
    fun speaksAfterAnswer_isTrueOnlyForTargetSentencesNotJustHeard() {
        val speakingExercises: List<ExerciseRecord> = everyKind.filter { exercise -> exercise.speaksAfterAnswer() }

        assertEquals(listOf(fillBlank, translateToTarget, orderWords), speakingExercises)
    }

    @Test
    fun clipKey_isTheExercisesClipForKindsWithOne() {
        assertNull(multipleChoice.clipKey())
        assertNull(match.clipKey())

        for (exercise in listOf(fillBlank, translateToTarget, orderWords, listeningChoice, dictation)) {
            assertEquals(CLIP_KEY, exercise.clipKey())
        }
    }

    @Test
    fun explanation_isTheExercisesExplanationForEveryKind() {
        for (exercise in everyKind) {
            assertEquals(EXPLANATION, exercise.explanation())
        }
    }

    @Test
    fun shuffledAvoidingIdentity_neverReturnsTheSentenceOrder() {
        val words: List<String> = listOf("Eu", "sou", "de", "Lisboa")

        for (seed in 0 until SHUFFLE_SEED_COUNT) {
            val shuffled: List<String> = shuffledAvoidingIdentity(words, Random(seed))

            assertNotEquals(words, shuffled)
            assertEquals(words.sorted(), shuffled.sorted())
        }
    }

    @Test
    fun shuffledAvoidingIdentity_keepsASingleWord() {
        assertEquals(listOf("Oi"), shuffledAvoidingIdentity(listOf("Oi")))
    }
}
