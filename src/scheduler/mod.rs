mod priority;
mod round_robin;

pub use priority::PriorityScheduler;
pub use round_robin::RoundRobinScheduler;

use crate::schedule::{CoreSchedule, ScheduleArtifact, TaskArtifact};

/// Scheduler trait
/// 
/// This trait is used to implement different scheduling algorithms.
/// 
/// # Example
/// 
/// ```
/// use scheduler::{Scheduler, PriorityScheduler};
/// 
/// let scheduler = PriorityScheduler::default();
/// let tasks = vec![Task::new("Task 1", 1), Task::new("Task 2", 2)];
/// let scheduled_tasks = scheduler.schedule(&tasks);
/// ```
pub trait Scheduler: std::fmt::Debug {
    /// Schedule the tasks by producing the next task artifact for a core
    fn schedule(&self, tasks: &Vec<TaskArtifact>, core: &CoreSchedule, preemptable: bool, current_time: u32) -> ScheduleArtifact;
}

/// Get the scheduler from the algorithm name
pub fn get_scheduler<S: AsRef<str>>(algorithm: S) -> Box<dyn Scheduler> {
    match algorithm.as_ref() {
        "round_robin" => Box::new(RoundRobinScheduler::default()),
        "priority" => Box::new(PriorityScheduler::default()),
        _ => panic!("Unknown scheduler algorithm"),
    }
}
