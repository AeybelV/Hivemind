# Hivemind

Hivemind is a customizable job scheduler for clusters written in Rust. It allows
you to schedule and manage jobs with flexible scheduling, dependencies, priortization,
distributed workloads, and many more.

## Features

- **Job Scheduling**: Schedule one-time or recurring jobs with ease.
- **Job Dependencies**: Define jobs that depend on the completion of other jobs.
- **Conditional Branching**: Create conditional flows where different tasks are
  executed based on the outcome of another.
- **Event Driven**: Define event driven jobs.
- **Job Prioritization**: Assign priority levels to jobs (high, medium, low) to
  control their execution order.
- **Concurrency Control**: Limit the number of jobs running simultaneously to
  avoid resource overload.
- **Retry Mechanism**: Automatically retry jobs on failure with customizable retry
  policies.
- **Job Status Tracking**: Monitor job status in real-time (pending, running,
  completed, failed).
- **Logging and Auditing**: Log job execution history, output, and errors for
  auditing and debugging.
- **Cluster Support**: Assign jobs to worker nodes in a cluster, allowing for a
  scalable job execution.
- **Circuit Breaker for External Services (Future)**: In containerized jobs,
  detect patterns that rely on external services such as API's databased. Prevent
  overwhelming external services during outages.
- **Access Control**: Role based access control for creating, deleting,
  and managing jobs.

## Core Concepts

- **Routines:** Reusable snippets that are pieces of logics or commands that can
  be used in multiple tasks. They can be thought of as function or macros
  that perform a certain action, and they can accept parameters to customize
  their behaviour.
- **Tasks:** The core units of work that the scheduler will execute. Each task
  has a unique identifier and can define parameters, a set of commands, and
  references to routines.
- **Task Groups:** Collections of tasks that can be run together. A group can
  define dependencies or run a sequence of tasks. Allowing you to organize
  complex workflows.
