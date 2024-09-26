use config::Config;
use gpu_type::GPU_TYPES;
use inquire::{CustomType, MultiSelect, Text};
use std::process::Command;
pub mod config;
pub mod gpu_type;

fn main() {
    let config = config::Config::read();

    let project = Text::new("Project:")
        .with_default(
            config
                .as_ref()
                .map(|c| c.project.clone())
                .flatten()
                .unwrap_or_default()
                .as_str(),
        )
        .prompt()
        .expect("failed to get project");

    let queue = Text::new("Queue:")
        .with_default(
            config
                .as_ref()
                .map(|c| c.queue.clone())
                .flatten()
                .unwrap_or_default()
                .as_str(),
        )
        .prompt()
        .expect("failed to get queue");

    let gpus = CustomType::new("GPUs:")
        .with_default(config.as_ref().map(|c| c.gpus).flatten().unwrap_or(1))
        .with_formatter(&|i: u8| format!("{i}"))
        .prompt()
        .expect("failed to get GPUs");

    let mut gpu_types = MultiSelect::new("GPU types:", gpu_type::GPU_TYPES.to_vec())
        .with_default(
            config
                .as_ref()
                .map(|c| c.gpu_types.clone())
                .flatten()
                .unwrap_or_default()
                .iter()
                .filter_map(|gpu_type| GPU_TYPES.iter().position(|t| t == gpu_type))
                .collect::<Vec<_>>()
                .as_slice(),
        )
        .prompt()
        .expect("failed to get GPU types");

    gpu_types.sort();

    let hours = CustomType::new("Hours:")
        .with_default(config.as_ref().map(|c| c.hours).flatten().unwrap_or(12))
        .with_formatter(&|i: u8| format!("{i}"))
        .prompt()
        .expect("failed to get hours");

    Config::new(
        Some(project.clone()),
        Some(queue.clone()),
        Some(gpus),
        Some(gpu_types.clone()),
        Some(hours),
    )
    .write();

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

    let gpus = match gpu_types.len() {
        1 => format!("{}:{}", gpu_types[0].name(), gpus),
        _ => format!(
            "{{{}}}",
            gpu_types
                .into_iter()
                .map(|gpu_type| format!("{}:{}", gpu_type.name(), gpus))
                .collect::<Vec<String>>()
                .join(",")
        ),
    };

    let script = format!(
        "#!/bin/sh
#SBATCH --job-name={job_name}
#SBATCH --output={output}

{command}

exit 0",
    );

    Command::new("bash")
        .arg("-c")
        .arg(format!(
            "sbatch --time={days}-{hours}:00:00 \
             --cpus-per-task={cpus_per_task} \
             --mem={memory}G \
             --account={project} \
             --partition={queue} \
             --gpus={gpus} \
             <(echo '{}')",
            script.replace("'", "'\\''")
        ))
        .output()
        .expect("failed to submit job");
}
