use leptos::prelude::*;
use shared::course::OpenResponseExercise;
use shared::progress::{self, OpenResponseTask, SavedOpenResponse, SavedOpenResponseGrade};

use crate::grading::{
    self as web_grading, GRADING_PENDING_NOTICE, GradedFeedbackHeading, GraderFindings, GradingProgressStatus, GradingStore, TaskGradingStatusKind,
    use_grading_store,
};
use crate::progress_state::{ProgressStore, use_progress_store};

/// The unit whose open responses a practice session saves.
#[derive(Debug, Clone)]
pub struct OpenResponseUnitContext {
    pub course_id: String,
    pub unit_id: String,
    pub level: String,
    pub target_language_name: String,
}

impl OpenResponseUnitContext {
    pub fn task_key(&self, exercise: &OpenResponseExercise) -> String {
        progress::open_response_task_key(&self.unit_id, exercise)
    }

    pub fn task(&self, exercise: &OpenResponseExercise) -> OpenResponseTask {
        OpenResponseTask::of(&self.level, exercise)
    }
}

pub fn use_open_response_unit_context() -> OpenResponseUnitContext {
    expect_context::<OpenResponseUnitContext>()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnswerGradeKind {
    Graded(SavedOpenResponseGrade),
    InProgress,
    Pending,
    /// Neither graded nor waiting for the grader.
    Ungraded,
}

/// The grade of the answer given at `answered_milliseconds`, which a newer saved answer leaves ungraded.
pub fn answer_grade(
    saved_open_response: Option<&SavedOpenResponse>,
    answered_milliseconds: i64,
    grading_status: TaskGradingStatusKind,
) -> AnswerGradeKind {
    let saved_grade: Option<SavedOpenResponseGrade> = saved_open_response
        .filter(|saved_open_response| saved_open_response.answered_milliseconds == answered_milliseconds)
        .and_then(|saved_open_response| saved_open_response.grade.clone());

    match (saved_grade, grading_status) {
        (Some(grade), _) => AnswerGradeKind::Graded(grade),
        (None, TaskGradingStatusKind::InProgress) => AnswerGradeKind::InProgress,
        (None, TaskGradingStatusKind::Pending) => AnswerGradeKind::Pending,
        (None, TaskGradingStatusKind::NotQueued) => AnswerGradeKind::Ungraded,
    }
}

/// Changes only when the grade's kind or content changes, so a progress bar in its view is not drawn again.
pub fn tracked_answer_grade(course_id: String, task_key: String, answered_milliseconds: i64) -> Memo<AnswerGradeKind> {
    let progress_store: ProgressStore = use_progress_store();
    let grading_store: GradingStore = use_grading_store();

    Memo::new(move |_| {
        let saved_open_response: Option<SavedOpenResponse> = progress_store.saved_open_response(&course_id, &task_key);
        let grading_status: TaskGradingStatusKind = grading_store.task_grading_status(&course_id, &task_key, answered_milliseconds);

        answer_grade(saved_open_response.as_ref(), answered_milliseconds, grading_status)
    })
}

#[component]
pub fn AnswerGrade(grade: Memo<AnswerGradeKind>, target_language_name: String) -> impl IntoView {
    move || match grade.get() {
        AnswerGradeKind::Graded(grade) => view! {
            <GradedFeedbackHeading heading=web_grading::mistake_count_heading(&grade.findings) />
            <GraderFindings grade target_language_name=target_language_name.clone() />
        }
        .into_any(),
        AnswerGradeKind::InProgress => view! { <GradingProgressStatus /> }.into_any(),
        AnswerGradeKind::Pending => view! { <p class="muted">{GRADING_PENDING_NOTICE}</p> }.into_any(),
        AnswerGradeKind::Ungraded => ().into_any(),
    }
}

/// The learner's saved answer to the task, shown while they write a new one.
#[component]
pub fn SavedOpenResponseAnswer(exercise: OpenResponseExercise) -> impl IntoView {
    let unit_context: OpenResponseUnitContext = use_open_response_unit_context();
    let progress_store: ProgressStore = use_progress_store();
    let task_key: String = unit_context.task_key(&exercise);
    let saved_answer: Memo<Option<(String, i64)>> = {
        let course_id: String = unit_context.course_id.clone();
        let task_key: String = task_key.clone();

        Memo::new(move |_| {
            progress_store
                .saved_open_response(&course_id, &task_key)
                .map(|saved_open_response| (saved_open_response.answer_text, saved_open_response.answered_milliseconds))
        })
    };

    move || {
        let (answer_text, answered_milliseconds): (String, i64) = saved_answer.get()?;
        let grade: Memo<AnswerGradeKind> = tracked_answer_grade(unit_context.course_id.clone(), task_key.clone(), answered_milliseconds);

        Some(view! {
            <div class="saved-answer">
                <div class="field-label">"Your last answer"</div>
                <p class="written-answer">{answer_text}</p>
                <AnswerGrade grade target_language_name=unit_context.target_language_name.clone() />
            </div>
        })
    }
}

/// The grade of an answer listed in a session's summary, while it is still the task's saved answer.
#[component]
pub fn OpenResponseSummaryGrade(exercise: OpenResponseExercise, answer_text: String) -> impl IntoView {
    let unit_context: OpenResponseUnitContext = use_open_response_unit_context();
    let progress_store: ProgressStore = use_progress_store();
    let task_key: String = unit_context.task_key(&exercise);
    let saved_answered_milliseconds: Option<i64> = progress_store
        .saved_open_response(&unit_context.course_id, &task_key)
        .filter(|saved_open_response| saved_open_response.answer_text == answer_text)
        .map(|saved_open_response| saved_open_response.answered_milliseconds);

    saved_answered_milliseconds.map(|answered_milliseconds| {
        let grade: Memo<AnswerGradeKind> = tracked_answer_grade(unit_context.course_id.clone(), task_key, answered_milliseconds);

        view! {
            <div class="summary-grade">
                <AnswerGrade grade target_language_name=unit_context.target_language_name.clone() />
            </div>
        }
    })
}

#[cfg(test)]
mod tests {
    use shared::grading::GraderPromptKind;

    use super::*;

    fn saved_open_response(answered_milliseconds: i64, grade: Option<SavedOpenResponseGrade>) -> SavedOpenResponse {
        SavedOpenResponse {
            task: OpenResponseTask {
                level: "a1".to_string(),
                prompt_source: "Introduce yourself.".to_string(),
                context_source: None,
            },
            answer_text: "Eu chamo Pedro.".to_string(),
            answered_milliseconds,
            grade,
        }
    }

    fn grade() -> SavedOpenResponseGrade {
        SavedOpenResponseGrade {
            grader_prompt: GraderPromptKind::BrazilianPortugueseTaughtInEnglish.identity(),
            answers_the_question: true,
            written_in_target_language: true,
            findings: Vec::new(),
        }
    }

    #[test]
    fn answer_grade_takes_the_saved_grade_only_of_the_same_answer() {
        let graded: SavedOpenResponse = saved_open_response(1, Some(grade()));

        assert_eq!(answer_grade(Some(&graded), 1, TaskGradingStatusKind::NotQueued), AnswerGradeKind::Graded(grade()));
        assert_eq!(answer_grade(Some(&graded), 2, TaskGradingStatusKind::NotQueued), AnswerGradeKind::Ungraded);
        assert_eq!(answer_grade(None, 2, TaskGradingStatusKind::Pending), AnswerGradeKind::Pending);
        assert_eq!(answer_grade(Some(&saved_open_response(2, None)), 2, TaskGradingStatusKind::InProgress), AnswerGradeKind::InProgress);
    }
}
