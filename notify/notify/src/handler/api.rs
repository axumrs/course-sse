use axum::{
    Json,
    extract::{Path, State},
};

use crate::{ArcAppState, Error, Result, model};

pub async fn find(
    State(state): State<ArcAppState>,
    Path(id): Path<String>,
) -> Result<Json<model::Notification>> {
    let m = model::Notification::find(&state.pool, &id).await?;
    let m = match m {
        Some(v) => v,
        None => return Err(Error::not_found("不存在的通知")),
    };

    Ok(Json(m))
}

#[derive(serde::Deserialize)]
pub struct ListPayload {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
    pub kind: Option<model::NotificationKind>,
    pub is_read: Option<bool>,
    pub title: Option<String>,
}

impl ListPayload {
    pub fn page(&self) -> i32 {
        self.page.unwrap_or(0)
    }
    pub fn page_size(&self) -> i32 {
        self.page_size.unwrap_or(model::DEFAULT_PAGE_SIZE)
    }
}

pub async fn list(
    State(state): State<ArcAppState>,
    Json(payload): Json<ListPayload>,
) -> Result<Json<model::Pagination<model::Notification>>> {
    let opts = model::NotificationListOption {
        page: payload.page(),
        page_size: payload.page_size(),
        kind: payload.kind,
        is_read: payload.is_read,
        title: payload.title,
    };
    let list = model::Notification::list(&state.pool, &opts).await?;
    let total = model::Notification::list_count(&state.pool, &opts).await?;
    Ok(Json(model::Pagination::new(total, opts.page, list)))
}

pub async fn make_read(
    State(state): State<ArcAppState>,
    Path(id): Path<String>,
) -> Result<Json<u64>> {
    let aff = model::Notification::make_read(&state.pool, &id).await?;
    Ok(Json(aff))
}

pub async fn make_all_read(State(state): State<ArcAppState>) -> Result<Json<u64>> {
    let aff = model::Notification::make_all_read(&state.pool).await?;
    Ok(Json(aff))
}

pub async fn del(State(state): State<ArcAppState>, Path(id): Path<String>) -> Result<Json<u64>> {
    let aff = model::Notification::del(&state.pool, &id).await?;
    Ok(Json(aff))
}
