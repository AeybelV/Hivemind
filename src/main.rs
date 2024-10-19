mod scheduler;

use scheduler::job::{Job, TaskGroup};

fn main() {
    let mut task_group = TaskGroup::new("ExampleGroup");

    let mut job1 = Job::new();
    let mut job2 = Job::new();
    let mut job3 = Job::new();

    job1.set_alias("Task 1");
    job3.set_alias("Task 3");

    job2.add_dependency(job1.get_id());
    job3.add_dependency(job2.get_id());

    // Add jobs to the task group
    task_group.add_job(job1);
    task_group.add_job(job2);
    task_group.add_job(job3);

    // Generate the Graphviz diagram
    task_group
        .create_graphviz_diagram("task_group.dot")
        .expect("Failed to create Graphviz diagram");
}
