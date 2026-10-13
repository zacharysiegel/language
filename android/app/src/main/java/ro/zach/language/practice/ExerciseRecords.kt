package ro.zach.language.practice

import ro.zach.language.core.ExerciseRecord

fun ExerciseRecord.clipKey(): String? {
    return when (this) {
        is ExerciseRecord.FillBlank -> clipKey
        is ExerciseRecord.TranslateToTarget -> clipKey
        is ExerciseRecord.OrderWords -> clipKey
        is ExerciseRecord.ListeningChoice -> clipKey
        is ExerciseRecord.Dictation -> clipKey
        is ExerciseRecord.MultipleChoice, is ExerciseRecord.Match -> null
    }
}

fun ExerciseRecord.explanation(): String? {
    return when (this) {
        is ExerciseRecord.MultipleChoice -> explanation
        is ExerciseRecord.FillBlank -> explanation
        is ExerciseRecord.TranslateToTarget -> explanation
        is ExerciseRecord.OrderWords -> explanation
        is ExerciseRecord.ListeningChoice -> explanation
        is ExerciseRecord.Dictation -> explanation
        is ExerciseRecord.Match -> explanation
    }
}

/* Kinds whose answer is a target-language sentence the learner did not just hear, played once the answer is checked. */
fun ExerciseRecord.speaksAfterAnswer(): Boolean {
    return when (this) {
        is ExerciseRecord.FillBlank, is ExerciseRecord.TranslateToTarget, is ExerciseRecord.OrderWords -> true
        is ExerciseRecord.MultipleChoice, is ExerciseRecord.ListeningChoice, is ExerciseRecord.Dictation, is ExerciseRecord.Match -> false
    }
}
