use crate::task::Task;
use crate::processor::Processor;
use crate::scheduler::get_scheduler;
use crate::scheduler::Scheduler;


pub type CoreSchedule = Vec<ScheduleArtifact>;


/// The artifact of a task in the schedule
/// 
/// The task is identified by the id, and the start and end time are the time
/// the task is scheduled to run.
/// 
/// This exists so the tasks can be preempted and resumed.
#[derive(Debug)]
pub struct TaskArtifact<'a> {
    pub task: &'a Task,
    pub duration: u32,
}

#[derive(Debug)]
pub struct ScheduleArtifact {
    pub task_id: u32,
    pub start_time: u32,
    pub end_time: u32,
}

impl ScheduleArtifact {
    pub fn duration(&self) -> u32 {
        self.end_time - self.start_time
    }
}


#[derive(Debug)]
pub struct Schedule {
    // The schedule for each processor
    processors: Vec<CoreSchedule>,
    // The tasks that are scheduled
    tasks: Vec<Task>,
}

impl Schedule {
    fn gcd(a: u32, b: u32) -> u32 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }

    pub fn display(&self) {
        for (i, processor) in self.processors.iter().enumerate() {
            println!("Processor {}:", i);
            for task in processor {
                println!("Task {:?}: {} - {}", task.task_id, task.start_time, task.end_time);
            }
        }
    }

    /// Get the least common multiple of the periods of the tasks
    pub fn get_hyperperiod(&self) -> u32 {
        let mut lcm = 1;
        for task in &self.tasks {
            lcm = lcm * task.period / Self::gcd(lcm, task.period);
        }
        lcm
    }

    /// Get the next free time for a processor
    /// 
    /// Returns a tuple with the next free time and the processor id
    fn next_free_time(&self, current_time: u32) -> (u32, u32) {
        let mut min_free_time = u32::MAX;
        let mut processor_id = 0;
        for (i, processor) in self.processors.iter().enumerate() {
            if processor.is_empty() {
                return (0, i as u32);
            }
            let free_time = processor.last().unwrap().end_time;
            if free_time < min_free_time {
                min_free_time = free_time;
                processor_id = i as u32;
            }
        }

        // If the next free time is less than the current time, then the current time is the next free time
        if min_free_time < current_time {
            (current_time, processor_id)
        } else {
            (min_free_time, processor_id)
        }
    }

    pub fn generate(tasks: Vec<Task>, processor: Processor) -> Self {
        // TODO: Implement the schedule generation algorithm
        // The algorithm should take into account the tasks, the processors and the scheduler algorithm
        // The algorithm should return a schedule for each processor
        // The schedule should contain the task id, the start time and the end time
        // The schedule should be a list of ScheduleArtifact

        let mut schedule = Schedule {
            processors: Vec::new(),
            tasks: tasks,
        };

        for _ in 0..processor.cores {
            schedule.processors.push(Vec::new());
        }

        let mut remaining_tasks: Vec<TaskArtifact> = Vec::new();

        let scheduler: Box<dyn Scheduler> = get_scheduler(processor.scheduler_algorithm);

        let mut current_time = 0;

        // Schedule for one hyperperiod
        while current_time < schedule.get_hyperperiod() {
            // Get newly released tasks
            for task in &schedule.tasks {
                if current_time % task.period == 0 {  // New release
                    println!("New release at time {} for task {}", current_time, task.id);
                    remaining_tasks.push(TaskArtifact {
                        task: task,
                        duration: task.execution_time,
                    });
                }
            }

            let (_next_free_time, processor_id) = schedule.next_free_time(current_time);
            if !remaining_tasks.is_empty() {
                let task: ScheduleArtifact = scheduler.schedule(
                    &remaining_tasks,
                    &schedule.processors[processor_id as usize],
                    processor.preemption_enabled,
                    current_time,
                );

                // Update the remaining tasks
                remaining_tasks.iter_mut().for_each(|task_artifact| {
                    if task_artifact.task.id == task.task_id {
                        task_artifact.duration -= task.duration();
                    }
                });

                // Remove the task from the remaining tasks if the duration is 0
                remaining_tasks.retain(|task_artifact| task_artifact.duration > 0);
                schedule.processors[processor_id as usize].push(task);
            }

            current_time += 1;
        }

        schedule
    }
}