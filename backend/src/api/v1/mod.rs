use axum::Router;
use chrono::{NaiveDateTime, Utc, Days};

use crate::ApiContext;

pub mod users;
pub mod projects;
pub mod task_groups;
pub mod tasks;
pub mod sub_tasks;

pub fn configure() -> Router<ApiContext> {
    Router::new()
        .merge(users::configure())
        .merge(projects::configure())
        .merge(task_groups::configure())
        .merge(tasks::configure())
        .merge(sub_tasks::configure())
}

#[derive(Deserialize)]
pub struct FetchQuery {
    // If filled only results made after this timestamp will be
    // fetched
    pub from: Option<NaiveDateTime>,
    // If filled only results made before this timestamp will be
    // fetched
    pub until: Option<NaiveDateTime>,
    // The maximum number of results to fetch, by default 20
    pub limit: Option<u32>,
    // The offset from which to start fetching, by default no
    // offset is applied. The actual offset is calculated from the
    // limit value (limit * page)
    pub page: Option<u32>,
}

pub struct FetchOptions {
    pub from: NaiveDateTime,
    pub until: NaiveDateTime,
    pub limit: u32,
    pub page: u32,
}

impl From<FetchQuery> for FetchOptions {
    fn from(query: FetchQuery) -> Self {
        let until = query.until.unwrap_or(Utc::now().naive_utc());
        let from = query.from.unwrap_or(until
            .checked_sub_days(Days::new(7))
            .unwrap_or_default());

        Self {
            from,
            until,
            limit: query.limit.unwrap_or(20),
            page: query.page.unwrap_or(0)
        }
    }
}
