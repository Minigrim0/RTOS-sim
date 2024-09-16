# RTOS Simulator

## Current State

This project is a Real-Time Operating System (RTOS) simulator implemented in Rust. It's currently in development and provides basic functionality for simulating task scheduling on a multi-core processor.

## Goals

1. Simulate various RTOS scheduling algorithms
2. Support multi-core processors
3. Allow for custom task and processor configurations
4. Provide visual output of task schedules
5. Implement different scheduling algorithms (e.g., Round Robin, Priority-based)

## Global Architecture

The simulator is structured into several key components:

1. **Main Application** (`src/main.rs`): Handles command-line arguments and orchestrates the simulation process.
2. **Task Management** (`src/task.rs`): Defines the structure for tasks and their properties.
3. **Processor Configuration** (`src/processor.rs`): Manages processor settings such as core count and scheduling algorithm.
4. **Scheduler** (`src/scheduler/mod.rs`, `src/scheduler/round_robin.rs`, `src/scheduler/priority.rs`): Implements different scheduling algorithms.
5. **Schedule Generation** (`src/schedule.rs`): Creates and manages the task schedule based on the chosen algorithm and processor configuration.

## Usage

To run the simulator:
```
cargo run -- -t <tasks_file> -p <processor_file> -o <output_file>
```
To generate default configuration files:
```
cargo run -- -g tasks
cargo run -- -g processor
```

## Current Limitations

- The schedule generation algorithm is not fully implemented.
- Only Round Robin and Priority scheduling are partially implemented.
- Visualization of the schedule is limited to console output.

## Next Steps

1. Complete the implementation of scheduling algorithms
2. Enhance the schedule generation logic
3. Improve schedule visualization
4. Add more unit tests and integration tests
5. Implement additional RTOS features (e.g., inter-task communication, resource management)

For more details on the implementation, refer to the source code in the `src` directory.
