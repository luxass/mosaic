use std::str::FromStr;

use apalis::{
  prelude::{Monitor, WorkerBuilder, WorkerFactoryFn},
  utils::TokioExecutor,
};
use apalis_cron::{CronStream, Schedule};
use mosaic_utils::{AppError, AppState};

mod jobs;
mod tasks;

pub async fn run(state: AppState) -> Result<(), AppError> {
  let update_projects_schedule = Schedule::from_str("0 */60 * * * *").unwrap();
  let update_projects_worker = WorkerBuilder::new("mosaic_cron::jobs::update_projects")
    .data(state.clone())
    .backend(CronStream::new(update_projects_schedule))
    .build_fn(jobs::update_projects);

  Monitor::<TokioExecutor>::new()
    .register(update_projects_worker)
    .run_with_signal(mosaic_utils::shutdown_signal())
    .await?;
  Ok(())
}
