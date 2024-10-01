use apalis::prelude::Data;
use chrono::{DateTime, Utc};
use mosaic_utils::AppState;
use serde::{Deserialize, Serialize};

use crate::tasks;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectsUpdate(DateTime<Utc>);

impl From<DateTime<Utc>> for ProjectsUpdate {
    fn from(t: DateTime<Utc>) -> Self {
        Self(t)
    }
}

pub async fn update_projects(_job: ProjectsUpdate, data: Data<AppState>) -> bool {
    tasks::update_projects(&data).await.is_ok()
}
