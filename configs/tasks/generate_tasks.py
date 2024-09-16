import random
import argparse
from typing import List, Dict

def generate_random_task(task_id: int) -> Dict:
    return {
        "id": task_id,
        "name": f"Task {task_id}",
        "period": random.randint(1, 10) * 100,  # Periods are now multiples of 100
        "execution_time": random.randint(10, 100),
        "deadline": random.randint(50, 1000),
        "priority": random.randint(1, 100)
    }

def generate_tasks(n: int) -> List[Dict]:
    return [generate_random_task(i) for i in range(1, n + 1)]

def save_tasks_to_toml(tasks: List[Dict], filename: str):
    with open(filename, 'w') as f:
        f.write("tasks = [\n")
        for task in tasks:
            f.write(f"\t{str(task).replace(': ', '= ')},\n")
        f.write("]\n")

def main():
    parser = argparse.ArgumentParser(description="Generate random tasks and save to TOML file.")
    parser.add_argument("-n", "--num_tasks", type=int, default=100, help="Number of tasks to generate (default: 100)")
    parser.add_argument("-o", "--output", type=str, default="tasks.toml", help="Output file name (default: tasks.toml)")
    args = parser.parse_args()

    tasks = generate_tasks(args.num_tasks)
    save_tasks_to_toml(tasks, args.output)
    print(f"{args.num_tasks} tasks have been generated and saved to {args.output}")

if __name__ == "__main__":
    main()
