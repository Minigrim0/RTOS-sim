import random
import argparse

def generate_cpu_config(num_cores, scheduler):
    config = {
        "processor": {
            "cores": num_cores,
            "scheduler_algorithm": scheduler,
            "preemption_enabled": True,
            "clock_speed": 1000
        }
    }
    return config

def generate_random_cpu_config():
    num_cores = random.randint(1, 16)
    scheduler = random.choice(["round_robin", "priority"])
    return generate_cpu_config(num_cores, scheduler)

def save_config_to_toml(config, filename):
    with open(filename, 'w') as f:
        f.write(f"cores = {config['processor']['cores']}\n")
        f.write(f"scheduler_algorithm = \"{config['processor']['scheduler_algorithm']}\"\n")
        f.write(f"preemption_enabled = {config['processor']['preemption_enabled']}\n")
        f.write(f"clock_speed = {config['processor']['clock_speed']}\n")

def main():
    parser = argparse.ArgumentParser(description="Generate CPU configuration and save to TOML file.")
    parser.add_argument("-c", "--cores", type=int, default=4, help="Number of CPU cores (default: 4)")
    parser.add_argument("-s", "--scheduler", type=str, default="round_robin", choices=["round_robin", "priority"], help="Scheduler type (default: round_robin)")
    parser.add_argument("-o", "--output", type=str, default="cpu_config.toml", help="Output file name (default: cpu_config.toml)")
    parser.add_argument("-r", "--random", action="store_true", help="Generate a random CPU configuration")
    args = parser.parse_args()

    if args.random:
        config = generate_random_cpu_config()
    else:
        config = generate_cpu_config(args.cores, args.scheduler)

    save_config_to_toml(config, args.output)
    print(f"CPU configuration with {config['processor']['cores']} cores and {config['processor']['scheduler_algorithm']} scheduler has been generated and saved to {args.output}")

if __name__ == "__main__":
    main()
