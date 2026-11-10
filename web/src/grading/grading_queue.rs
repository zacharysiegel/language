use std::collections::VecDeque;

use crate::grading::{GraderStateKind, OpenResponseSubmission};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueuedGrading {
    pub course_id: String,
    pub task_key: String,
    pub answered_milliseconds: i64,
    pub submission: OpenResponseSubmission,
}

impl QueuedGrading {
    fn is_for_task(&self, course_id: &str, task_key: &str) -> bool {
        self.course_id == course_id && self.task_key == task_key
    }

    fn is_for_answer(&self, course_id: &str, task_key: &str, answered_milliseconds: i64) -> bool {
        self.is_for_task(course_id, task_key) && self.answered_milliseconds == answered_milliseconds
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GradingPriorityKind {
    /// Checked in the question on screen; graded before every saved answer.
    JustChecked,
    /// Saved earlier without a grade.
    Saved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskGradingStatusKind {
    NotQueued,
    Pending,
    InProgress,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenResponseSessionPresenceKind {
    Open,
    NoneOpen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GradingQueueStepKind {
    Nothing,
    LoadModel,
    /// Takes the next grading off the queue and runs it.
    StartGeneration,
    /// This page cannot load the model, so no waiting answer is graded.
    AbandonWaiting,
    UnloadModel,
}

/// The answers waiting for the grading model, graded one at a time.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GradingQueue {
    waiting: VecDeque<(GradingPriorityKind, QueuedGrading)>,
    running: Option<QueuedGrading>,
}

impl GradingQueue {
    /// Replaces a waiting grading of the same task; does nothing for an answer already waiting or running.
    pub fn enqueue(&mut self, grading: QueuedGrading, priority: GradingPriorityKind) {
        let already_queued: bool = self
            .running
            .iter()
            .chain(self.waiting.iter().map(|(_, waiting_grading)| waiting_grading))
            .any(|queued_grading| queued_grading.is_for_answer(&grading.course_id, &grading.task_key, grading.answered_milliseconds));

        if already_queued {
            return;
        }

        self.waiting
            .retain(|(_, waiting_grading)| !waiting_grading.is_for_task(&grading.course_id, &grading.task_key));

        let insert_position: usize = match priority {
            GradingPriorityKind::JustChecked => self
                .waiting
                .iter()
                .take_while(|(waiting_priority, _)| *waiting_priority == GradingPriorityKind::JustChecked)
                .count(),
            GradingPriorityKind::Saved => self.waiting.len(),
        };

        self.waiting.insert(insert_position, (priority, grading));
    }

    /// Moves the first waiting grading to running.
    pub fn take_next(&mut self) -> Option<QueuedGrading> {
        if self.running.is_some() {
            return None;
        }

        let (_, next_grading): (GradingPriorityKind, QueuedGrading) = self.waiting.pop_front()?;

        self.running = Some(next_grading.clone());

        Some(next_grading)
    }

    pub fn finish_running(&mut self) {
        self.running = None;
    }

    pub fn abandon_waiting(&mut self) {
        self.waiting.clear();
    }

    pub fn is_idle(&self) -> bool {
        self.running.is_none() && self.waiting.is_empty()
    }

    /// The next grading counts as in progress while the model loads for it.
    pub fn task_status(&self, course_id: &str, task_key: &str, answered_milliseconds: i64, grader_state: GraderStateKind) -> TaskGradingStatusKind {
        let is_running: bool = self
            .running
            .as_ref()
            .is_some_and(|running_grading| running_grading.is_for_answer(course_id, task_key, answered_milliseconds));
        let waiting_position: Option<usize> = self
            .waiting
            .iter()
            .position(|(_, waiting_grading)| waiting_grading.is_for_answer(course_id, task_key, answered_milliseconds));
        let model_loads_for_the_next: bool =
            self.running.is_none() && matches!(grader_state, GraderStateKind::Downloaded | GraderStateKind::Loading);

        match waiting_position {
            _ if is_running => TaskGradingStatusKind::InProgress,
            Some(0) if model_loads_for_the_next => TaskGradingStatusKind::InProgress,
            Some(_) => TaskGradingStatusKind::Pending,
            None => TaskGradingStatusKind::NotQueued,
        }
    }

    /// The model stays in memory while a session is open or a grading is waiting or running.
    pub fn next_step(&self, grader_state: GraderStateKind, session_presence: OpenResponseSessionPresenceKind) -> GradingQueueStepKind {
        if self.running.is_some() {
            return GradingQueueStepKind::Nothing;
        }

        if self.waiting.is_empty() {
            let holds_model: bool = matches!(grader_state, GraderStateKind::Loading | GraderStateKind::Ready);

            return match session_presence {
                OpenResponseSessionPresenceKind::NoneOpen if holds_model => GradingQueueStepKind::UnloadModel,
                OpenResponseSessionPresenceKind::NoneOpen | OpenResponseSessionPresenceKind::Open => GradingQueueStepKind::Nothing,
            };
        }

        match grader_state {
            GraderStateKind::Ready => GradingQueueStepKind::StartGeneration,
            GraderStateKind::Downloaded => GradingQueueStepKind::LoadModel,
            GraderStateKind::Unchecked | GraderStateKind::Loading => GradingQueueStepKind::Nothing,
            GraderStateKind::Unsupported
            | GraderStateKind::NotDownloaded
            | GraderStateKind::Downloading(_)
            | GraderStateKind::InUseInAnotherPage
            | GraderStateKind::Failed => GradingQueueStepKind::AbandonWaiting,
        }
    }
}

#[cfg(test)]
mod tests {
    use shared::grading::GraderPromptKind;

    use super::*;

    const COURSE_ID: &str = "pt_br.en.foundations";

    fn grading(task_key: &str, answered_milliseconds: i64) -> QueuedGrading {
        QueuedGrading {
            course_id: COURSE_ID.to_string(),
            task_key: task_key.to_string(),
            answered_milliseconds,
            submission: OpenResponseSubmission {
                grader_prompt: GraderPromptKind::BrazilianPortugueseTaughtInEnglish,
                level: "a1".to_string(),
                prompt_source: "Introduce yourself.".to_string(),
                context_source: None,
                answer: "Eu chamo Pedro.".to_string(),
            },
        }
    }

    fn taken_task_keys(queue: &mut GradingQueue) -> Vec<String> {
        let mut task_keys: Vec<String> = Vec::new();

        while let Some(next_grading) = queue.take_next() {
            task_keys.push(next_grading.task_key);
            queue.finish_running();
        }

        task_keys
    }

    #[test]
    fn take_next_grades_just_checked_answers_in_order_before_saved_ones() {
        let mut queue: GradingQueue = GradingQueue::default();

        queue.enqueue(grading("saved-1", 1), GradingPriorityKind::Saved);
        queue.enqueue(grading("checked-1", 10), GradingPriorityKind::JustChecked);
        queue.enqueue(grading("saved-2", 2), GradingPriorityKind::Saved);
        queue.enqueue(grading("checked-2", 11), GradingPriorityKind::JustChecked);

        assert_eq!(taken_task_keys(&mut queue), vec!["checked-1", "checked-2", "saved-1", "saved-2"]);
    }

    #[test]
    fn enqueue_replaces_the_waiting_answer_of_the_same_task_and_ignores_one_already_queued() {
        let mut queue: GradingQueue = GradingQueue::default();
        queue.enqueue(grading("running", 1), GradingPriorityKind::Saved);
        queue.take_next();
        queue.enqueue(grading("task", 2), GradingPriorityKind::Saved);
        queue.enqueue(grading("other", 3), GradingPriorityKind::Saved);

        queue.enqueue(grading("task", 4), GradingPriorityKind::JustChecked);
        queue.enqueue(grading("other", 3), GradingPriorityKind::Saved);
        queue.enqueue(grading("running", 1), GradingPriorityKind::Saved);

        assert_eq!(queue.task_status(COURSE_ID, "task", 2, GraderStateKind::Ready), TaskGradingStatusKind::NotQueued);

        queue.finish_running();

        assert_eq!(taken_task_keys(&mut queue), vec!["task", "other"]);
    }

    #[test]
    fn task_status_tells_the_running_answer_from_a_newer_answer_to_the_same_task() {
        let mut queue: GradingQueue = GradingQueue::default();
        queue.enqueue(grading("task", 1), GradingPriorityKind::Saved);
        queue.take_next();

        queue.enqueue(grading("task", 2), GradingPriorityKind::JustChecked);

        assert_eq!(queue.task_status(COURSE_ID, "task", 1, GraderStateKind::Ready), TaskGradingStatusKind::InProgress);
        assert_eq!(queue.task_status(COURSE_ID, "task", 2, GraderStateKind::Ready), TaskGradingStatusKind::Pending);
        assert_eq!(queue.task_status("es.en.foundations", "task", 2, GraderStateKind::Ready), TaskGradingStatusKind::NotQueued);
    }

    #[test]
    fn next_step_loads_the_model_for_a_waiting_grading_and_generates_once_it_is_ready() {
        let mut queue: GradingQueue = GradingQueue::default();
        queue.enqueue(grading("task", 1), GradingPriorityKind::JustChecked);

        assert_eq!(queue.next_step(GraderStateKind::Downloaded, OpenResponseSessionPresenceKind::NoneOpen), GradingQueueStepKind::LoadModel);
        assert_eq!(queue.next_step(GraderStateKind::Loading, OpenResponseSessionPresenceKind::NoneOpen), GradingQueueStepKind::Nothing);
        assert_eq!(queue.next_step(GraderStateKind::Ready, OpenResponseSessionPresenceKind::NoneOpen), GradingQueueStepKind::StartGeneration);

        queue.take_next();

        assert_eq!(queue.next_step(GraderStateKind::Ready, OpenResponseSessionPresenceKind::NoneOpen), GradingQueueStepKind::Nothing);
    }

    #[test]
    fn next_step_abandons_the_waiting_gradings_when_this_page_cannot_load_the_model() {
        let mut queue: GradingQueue = GradingQueue::default();
        queue.enqueue(grading("task", 1), GradingPriorityKind::JustChecked);

        for grader_state in [GraderStateKind::InUseInAnotherPage, GraderStateKind::Failed, GraderStateKind::Unsupported] {
            assert_eq!(queue.next_step(grader_state, OpenResponseSessionPresenceKind::Open), GradingQueueStepKind::AbandonWaiting);
        }

        queue.abandon_waiting();

        assert!(queue.is_idle());
    }

    #[test]
    fn next_step_unloads_the_model_only_once_no_session_is_open_and_the_queue_is_idle() {
        let mut queue: GradingQueue = GradingQueue::default();

        assert_eq!(queue.next_step(GraderStateKind::Ready, OpenResponseSessionPresenceKind::Open), GradingQueueStepKind::Nothing);
        assert_eq!(queue.next_step(GraderStateKind::Ready, OpenResponseSessionPresenceKind::NoneOpen), GradingQueueStepKind::UnloadModel);
        assert_eq!(queue.next_step(GraderStateKind::Loading, OpenResponseSessionPresenceKind::NoneOpen), GradingQueueStepKind::UnloadModel);
        assert_eq!(queue.next_step(GraderStateKind::Downloaded, OpenResponseSessionPresenceKind::NoneOpen), GradingQueueStepKind::Nothing);

        queue.enqueue(grading("task", 1), GradingPriorityKind::Saved);
        queue.take_next();

        assert_eq!(queue.next_step(GraderStateKind::Ready, OpenResponseSessionPresenceKind::NoneOpen), GradingQueueStepKind::Nothing);

        queue.finish_running();

        assert_eq!(queue.next_step(GraderStateKind::Ready, OpenResponseSessionPresenceKind::NoneOpen), GradingQueueStepKind::UnloadModel);
    }

    #[test]
    fn task_status_counts_the_next_grading_in_progress_while_the_model_loads_for_it() {
        let mut queue: GradingQueue = GradingQueue::default();
        queue.enqueue(grading("first", 1), GradingPriorityKind::Saved);
        queue.enqueue(grading("second", 2), GradingPriorityKind::Saved);

        assert_eq!(queue.task_status(COURSE_ID, "first", 1, GraderStateKind::Loading), TaskGradingStatusKind::InProgress);
        assert_eq!(queue.task_status(COURSE_ID, "second", 2, GraderStateKind::Loading), TaskGradingStatusKind::Pending);
        assert_eq!(queue.task_status(COURSE_ID, "first", 1, GraderStateKind::Ready), TaskGradingStatusKind::Pending);
    }
}
