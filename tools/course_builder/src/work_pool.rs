use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

use shared::AppError;

const PROGRESS_LOG_INTERVAL: usize = 200;
const REPORTED_FAILURE_LIMIT: usize = 10;

/// Runs `process` on every item across `worker_count` threads, each with state from `create_state`.
pub fn run_work_pool<Item, State>(
    label: &str,
    items: &[Item],
    worker_count: usize,
    describe: impl Fn(&Item) -> String + Sync,
    create_state: impl Fn() -> Result<State, AppError> + Sync,
    process: impl Fn(&mut State, &Item) -> Result<(), AppError> + Sync,
) -> Result<(), AppError>
where
    Item: Sync,
{
    let next_item_index: AtomicUsize = AtomicUsize::new(0);
    let completed_count: AtomicUsize = AtomicUsize::new(0);
    let failure_messages: Mutex<Vec<String>> = Mutex::new(Vec::new());

    thread::scope(|scope| {
        for _ in 0..worker_count {
            scope.spawn(|| {
                let state_result: Result<State, AppError> = create_state();
                let mut state: State = match state_result {
                    Ok(state) => state,
                    Err(error) => {
                        record_failure(&failure_messages, format!("worker could not start: {error}"));
                        return;
                    },
                };

                loop {
                    let item_index: usize = next_item_index.fetch_add(1, Ordering::SeqCst);
                    let Some(item) = items.get(item_index) else {
                        return;
                    };

                    if let Err(error) = process(&mut state, item) {
                        record_failure(&failure_messages, format!("{}: {error}", describe(item)));
                    }

                    let completed: usize = completed_count.fetch_add(1, Ordering::SeqCst) + 1;

                    if completed % PROGRESS_LOG_INTERVAL == 0 {
                        log::info!("progress; [work={label} completed={completed} total={}]", items.len());
                    }
                }
            });
        }
    });

    let failures: Vec<String> = failure_messages.into_inner().unwrap_or_default();

    if failures.is_empty() {
        return Ok(());
    }

    for failure in failures.iter().take(REPORTED_FAILURE_LIMIT) {
        log::error!("work item failed; [work={label} detail={failure}]");
    }

    Err(AppError::from(format!("work items failed; [work={label} failures={}]", failures.len())))
}

fn record_failure(failure_messages: &Mutex<Vec<String>>, message: String) {
    if let Ok(mut failures) = failure_messages.lock() {
        failures.push(message);
    }
}
