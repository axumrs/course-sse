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

    pub async fn create(&self, e: impl PgExecutor<'_>) -> sqlx::Result<Self> {
        let mut q = sqlx::QueryBuilder::new(
            r#"INSERT INTO "notifications" ("id", "title", "content", "is_read", "kind", "created_at") "#,
        );
        q.push_values(&[self], |mut b, m| {
            b.push_bind(&m.id)
                .push_bind(&m.title)
                .push_bind(&m.content)
                .push_bind(&m.is_read)
                .push_bind(&m.kind)
                .push_bind(&m.created_at);
        });
        q.push(r#" RETURNING *"#);

        q.build_query_as().fetch_one(e).await
    }

    pub async fn find(e: impl PgExecutor<'_>, id: &str) -> sqlx::Result<Option<Self>> {
        sqlx::query_as(r#"SELECT  "id", "title", "content", "is_read", "kind", "created_at" FROM "notifications" WHERE "id"=$1"#).bind(id).fetch_optional(e).await
    }

    pub async fn list<'a>(
        e: impl PgExecutor<'a>,
        opts: &'a NotificationListOption,
    ) -> sqlx::Result<Vec<Self>> {
        let q = sqlx::QueryBuilder::new(
            r#"SELECT  "id", "title", "content", "is_read", "kind", "created_at" FROM "notifications"  WHERE 1=1"#,
        );
        let mut q = Self::build_list_query(q, opts);

        q.push(r#" ORDER BY "id" DESC "#)
            .push(" LIMIT ")
            .push_bind(opts.page_size)
            .push(" OFFSET ")
            .push_bind(opts.page * opts.page_size);

        q.build_query_as().fetch_all(e).await
    }

    pub async fn list_count<'a>(
        e: impl PgExecutor<'a>,
        opts: &'a NotificationListOption,
    ) -> sqlx::Result<i64> {
        let q = sqlx::QueryBuilder::new(r#"SELECT COUNT(*) FROM "notifications" WHERE 1=1"#);
        let mut q = Self::build_list_query(q, opts);
        q.build_query_scalar().fetch_one(e).await
    }
    fn build_list_query<'a>(
        mut q: sqlx::QueryBuilder<'a, sqlx::Postgres>,
        opts: &'a NotificationListOption,
    ) -> sqlx::QueryBuilder<'a, sqlx::Postgres> {
        if let Some(kind) = &opts.kind {
            q.push(r#" AND "kind" = "#).push_bind(kind);
        }
        if let Some(is_read) = &opts.is_read {
            q.push(r#" AND "is_read" = "#).push_bind(is_read);
        }
        if let Some(title) = &opts.title {
            q.push(r#" AND "title" ILIKE "#)
                .push_bind(format!("%{title}%"));
        }
        q
    }

    pub async fn make_read(e: impl PgExecutor<'_>, id: &str) -> sqlx::Result<u64> {
        let aff = sqlx::query(r#"UPDATE "notifications" SET "is_read" = true WHERE "id" = $1"#)
            .bind(id)
            .execute(e)
            .await?
            .rows_affected();
        Ok(aff)
    }

    pub async fn make_all_read(e: impl PgExecutor<'_>) -> sqlx::Result<u64> {
        let aff = sqlx::query(r#"UPDATE "notifications" SET "is_read" = true"#)
            .execute(e)
            .await?
            .rows_affected();
        Ok(aff)
    }

    pub async fn del(e: impl PgExecutor<'_>, id: &str) -> sqlx::Result<u64> {
        let aff = sqlx::query(r#"DELETE FROM "notifications" WHERE "id" = $1"#)
            .bind(id)
            .execute(e)
            .await?
            .rows_affected();
        Ok(aff)
    }

    pub async fn turncate(e: impl PgExecutor<'_>) -> sqlx::Result<()> {
        let _ = sqlx::query(r#"TRUNCATE TABLE "notifications""#)
            .execute(e)
            .await?;
        Ok(())
    }
}

pub struct NotificationListOption {
    pub page: i32,
    pub page_size: i32,
    pub kind: Option<NotificationKind>,
    pub is_read: Option<bool>,
    pub title: Option<String>,
}

impl Default for NotificationListOption {
    fn default() -> Self {
        Self {
            page: 0,
            page_size: DEFAULT_PAGE_SIZE,
            kind: Default::default(),
            is_read: Default::default(),
            title: Default::default(),
        }
    }
}

pub const DEFAULT_PAGE_SIZE: i32 = 30;

#[derive(Debug, Serialize)]
pub struct Pagination<T> {
    pub page: i32,
    pub page_size: i32,
    pub total: i64,
    pub total_page: i64,
    pub data: Vec<T>,
}
impl<T: Serialize> Pagination<T> {
    pub fn with_page_size(page: i32, page_size: i32, total: i64, data: Vec<T>) -> Self {
        let total_page = f64::ceil(total as f64 / page_size as f64) as i64;
        Self {
            page,
            page_size,
            total,
            total_page,
            data,
        }
    }
    pub fn new(total: i64, page: i32, data: Vec<T>) -> Self {
        Self::with_page_size(page, DEFAULT_PAGE_SIZE, total, data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    async fn get_pool() -> sqlx::PgPool {
        let cfg = Config::from_env().unwrap();
        sqlx::postgres::PgPool::connect(&cfg.database_url)
            .await
            .unwrap()
    }
    #[tokio::test]
    async fn test_create_notify() {
        let pool = get_pool().await;
        let m = Notification::new("通知-test", "内容-test", NotificationKind::Instance);
        let m = m.create(&pool).await.unwrap();
        println!("{:?}", m);
    }

    #[tokio::test]
    async fn test_list_notify() {
        let pool = get_pool().await;
        let notification_list = Notification::list(&pool, &NotificationListOption::default()).await;
        println!("{:?}", notification_list);
    }

    #[tokio::test]
    async fn test_find_notify() {
        let pool = get_pool().await;
        let notification = Notification::find(&pool, "d7o0bpghmvaps2e9of5g").await;
        println!("{:?}", notification);
    }

    #[tokio::test]
    async fn test_count_unread_notify() {
        let pool = get_pool().await;
        let c = Notification::count_unread(&pool).await;
        println!("{:?}", c);
    }
}
