use crate::runtime::task::{self, Task};

/// `task::Schedule` implementation that does nothing. This is unique to the
/// blocking scheduler as tasks scheduled are not really futures but blocking
/// operations.
///
/// We avoid storing the task by forgetting it in `bind` and re-materializing it
/// in `release.
pub(super) struct NoopSchedule;

impl task::Schedule for NoopSchedule {
    fn bind(task: Task<Self>) -> NoopSchedule {
        // Forget the task here, it will be re-materialized in `release`.
        std::mem::forget(task);
        NoopSchedule
    }

    fn release(&self, task: &Task<Self>) -> Option<Task<Self>> {
        // safety: the task is forgotten in `bind`.
        Some(unsafe { Task::from_raw(task.header().into()) })
    }

    fn schedule(&self, _task: task::Notified<Self>) {
        unreachable!();
    }
}

impl task::ScheduleSendOnly for NoopSchedule {}
