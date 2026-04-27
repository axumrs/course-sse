CREATE TYPE "notification_kind" AS ENUM ('Uncategorized', 'Instance', 'Snapshot', 'Transfer', 'Security', 'Ticket', 'System');

CREATE TABLE IF NOT EXISTS "notifications"(
    "id" TEXT CHECK(LENGTH("id") = 20) PRIMARY KEY,
    "title" TEXT NOT NULL,
    "content" TEXT NOT NULL,
    "is_read" BOOLEAN NOT NULL DEFAULT FALSE,
    "kind" notification_kind NOT NULL DEFAULT 'Uncategorized',
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);