use config::Config;
use inquire::{CustomType, MultiSelect, Text};
use std::process::Command;
pub mod config;
pub mod gpu_type;

fn main() {
    let config = config::Config::read();
    let default_account = config.as_ref().map(|c| c.account.as_str());
    let default_queue = config.as_ref().map(|c| c.queue.as_str());

    let account = match default_account {
        Some(a) => Text::new("Account:").with_default(&a),
        None => Text::new("Account:"),
    }
    .prompt()
    .expect("failed to get account");

    let queue = match default_queue {
        Some(q) => Text::new("Queue:").with_default(&q),
        None => Text::new("Queue:"),
    }
    .prompt()
    .expect("failed to get queue");

    Config::new(account.clone(), queue.clone()).write();

    let gpus = CustomType::new("GPUs:")
        .with_default(1)
        .with_formatter(&|i: u8| format!("{i}"))
        .prompt()
        .expect("failed to get GPUs");

    let gpu_types = MultiSelect::new("GPU types:", gpu_type::GPU_TYPES.to_vec())
        .prompt()
        .expect("failed to get GPU types");

    let hours = CustomType::new("Hours:")
        .with_default(12)
        .with_formatter(&|i: u8| format!("{i}"))
        .prompt()
        .expect("failed to get hours");

    let command = Text::new("Command:")
        .prompt()
        .expect("failed to get command");

    let name = Text::new("Name:").prompt().expect("failed to get name");

    let job_name = name;
    let output = format!("{}.o", job_name);

    let days = hours / 24;
    let hours = hours % 24;

    let cpus_per_task = gpu_types
        .iter()
        .map(|gpu| gpu.cpus_per_gpu())
        .min()
        .expect("failed to get CPUs per task");

    let memory = gpu_types
        .iter()
        .map(|gpu| gpu.system_memory_per_gpu())
        .min()
        .expect("failed to get memory");

    let gpus = gpu_types
        .into_iter()
        .map(|gpu_type| format!("{}:{}", gpu_type.name(), gpus))
        .collect::<Vec<String>>()
        .join(",");

    let script = format!(
        "#!/bin/sh
#SBATCH --job-name={job_name}
#SBATCH --output={output}
cd $SLURM_SUBMIT_DIR
{command}",
    );

    let cmd = format!(
        "--time={days}-{hours}:00:00 \
--cpus-per-task={cpus_per_task} \
--mem={memory}G \
--account={account} \
--partition={queue} \
--gpus={{{gpus}}} ",
    );

    Command::new("sh")
        .arg("-c")
        .arg(format!("{script} | sbatch {cmd}"))
        .spawn()
        .expect("failed to submit job");
}
