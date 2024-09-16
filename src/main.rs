use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use std::fs;

mod processor;
mod task;
mod scheduler;
mod schedule;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the tasks description file
    #[arg(short, long)]
    tasks_file: Option<PathBuf>,

    /// Path to the processor description file
    #[arg(short, long)]
    processor_file: Option<PathBuf>,

    /// Output file path (default: stdout)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Generate default configuration files
    #[arg(short, long, value_enum)]
    generate: Option<ConfigType>,
}

#[derive(Debug, Clone, ValueEnum)]
enum ConfigType {
    Tasks,
    Processor,
}

fn main() {
    let args = Args::parse();

    if let Some(config_type) = args.generate {
        match config_type {
            ConfigType::Tasks => println!("{}", toml::to_string_pretty(&task::TasksConfig::default()).unwrap()),
            ConfigType::Processor => println!("{}", toml::to_string_pretty(&processor::Processor::default()).unwrap()),
        }
        return;
    }

    // Read task description file
    let tasks_config = task::TasksConfig::new(args.tasks_file.as_ref());
    println!("Scheduling {} tasks", tasks_config.tasks.len());

    // Read processor description file
    let processor: processor::Processor = processor::Processor::new(args.processor_file.as_ref());
    println!("Using processor with {} cores", processor.cores);
    
    // Generate schedule
    let schedule: schedule::Schedule = schedule::Schedule::generate(tasks_config.tasks, processor);
    
    // Output results
    schedule.display();
    println!("Schedule hyperperiod: {:?}", schedule.get_hyperperiod());
    let result = "";
    match args.output {
        Some(path) => fs::write(path, result).expect("Failed to write output file"),
        None => println!("{}", result),
    }
}
