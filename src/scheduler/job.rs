use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use uuid::Uuid;

enum JobState {
    Queued,    // Task is qeueud and waiting to be assigned to a worker
    Waiting,   // Waiting for dependencies or conditions to be met before starting
    Running,   // Job is currently executing
    Timeout,   // Job timed-ut
    Success,   // Job executed without errors
    Failed,    // Job failed execution
    Skipped,   // Job was skipped due to conditions
    Removed,   // Task was removed from queue
    Cancelled, //Task was cancelled by user or system
}

pub struct Job {
    id: Uuid,
    alias: Option<String>,
    state: JobState,
    dependencies: Vec<Uuid>,
}

impl Job {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            alias: None,
            state: JobState::Queued,
            dependencies: vec![],
        }
    }

    pub fn set_alias(&mut self, alias: &str) {
        self.alias = Some(alias.to_string())
    }

    pub fn add_dependency(&mut self, task: Uuid) {
        self.dependencies.push(task);
    }

    pub fn get_id(&mut self) -> Uuid {
        return self.id;
    }
}

pub struct TaskGroup {
    pub jobs: HashMap<Uuid, Job>,
    pub name: String,
}

impl TaskGroup {
    pub fn new(name: &str) -> Self {
        Self {
            jobs: HashMap::new(),
            name: name.to_string(),
        }
    }
    pub fn add_job(&mut self, job: Job) {
        self.jobs.insert(job.id, job);
    }

    pub fn create_graphviz_diagram(&self, output_path: &str) -> io::Result<()> {
        let mut file = File::create(output_path)?;

        writeln!(file, "digraph {} {{", self.name)?;
        writeln!(file, "    node [shape=box];")?;

        for (job_id, job) in &self.jobs {
            // Node label: use alias if available, otherwise use the job ID.
            let job_label = job.alias.clone().unwrap_or_else(|| job.id.to_string());

            // Write the node definition
            writeln!(file, r#"    "{}" [label="{}"];"#, job_id, job_label)?;

            // Write edges for each dependency
            for dependency_id in &job.dependencies {
                writeln!(file, r#"    "{}" -> "{}";"#, dependency_id, job_id)?;
            }
        }

        writeln!(file, "}}")?;
        Ok(())
    }
}
