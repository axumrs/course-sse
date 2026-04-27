use serde::{Deserialize, Serialize};
use sqlx::PgExecutor;

#[derive(Debug, Serialize, Deserialize, Default, sqlx::Type)]
#[sqlx(type_name = "notification_kind")]
pub enum NotificationKind {
    #[default]
    Uncategorized,
    Instance,
    Snapshot,
    Transfer,
    Security,
    Ticket,
    System,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Default)]
pub struct Notification {
    pub id: String,
    pub title: String,
    pub content: String,
    pub is_read: bool,
    pub kind: NotificationKind,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Notification {
    pub fn new(
        title: impl Into<String>,
        content: impl Into<String>,
        kind: NotificationKind,
    ) -> Self {
        Self {
            id: xid::new().to_string(),
            title: title.into(),
            content: content.into(),
            is_read: false,
            kind,
            created_at: chrono::Utc::now(),
        }
    }

    pub async fn count_unread(e: impl PgExecutor<'_>) -> sqlx::Result<i64> {
        sqlx::query_scalar(r#"SELECT COUNT(*) FROM "notifications" WHERE "is_read" = false"#)
            .fetch_one(e)
            .await
    }
}
