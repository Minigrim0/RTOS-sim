use crate::scheduler::Scheduler;
use crate::schedule::{TaskArtifact, CoreSchedule, ScheduleArtifact};
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct PriorityScheduler {
    priority_levels: u32,
    time_slice: u32,
}

impl Default for PriorityScheduler {
    fn default() -> Self {
        Self {
            priority_levels: 10,
            time_slice: 10,
        }
    }
}

impl Scheduler for PriorityScheduler {
    fn schedule(&self, tasks: &Vec<TaskArtifact>, _core: &CoreSchedule, _preemptable: bool, _current_time: u32) -> ScheduleArtifact {
        ScheduleArtifact {
            task_id: tasks[0].task.id,
            start_time: 0,
            end_time: 0,
        }
    }
}
