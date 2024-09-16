use crate::scheduler::Scheduler;
use crate::schedule::TaskArtifact;
use serde::{Serialize, Deserialize};
use std::cmp::max;

use crate::schedule::{CoreSchedule, ScheduleArtifact};


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RoundRobinScheduler;


impl Scheduler for RoundRobinScheduler {
    fn schedule(&self, remaining_tasks: &Vec<TaskArtifact>, core: &CoreSchedule, _preemptable: bool, current_time: u32) -> ScheduleArtifact {
        // Round robin cycles through tasks. The tasks are not preempted.
        let last_task_end_time = core.last().and_then(|last_task| Some(last_task.end_time)).unwrap_or(0);
        let task_start_time = max(last_task_end_time, current_time);

        ScheduleArtifact {
            task_id: remaining_tasks[0].task.id,
            start_time: task_start_time,
            end_time: task_start_time + remaining_tasks[0].duration,
        }
    }
}
