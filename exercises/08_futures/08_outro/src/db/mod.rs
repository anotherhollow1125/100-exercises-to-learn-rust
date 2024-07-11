use crate::data::{Ticket, TicketId};
use anyhow::Result;
use async_trait::async_trait;
use shaku::Interface;

pub mod sqlite;
pub use sqlite::{SqliteImpl, SqliteImplParameters};

// async Traitは最近(1.75から)の機能！
// しかしobject safeにならず！

// upsertとはせず、それぞれで事前にチェックをしてから挿入

#[async_trait]
pub trait Db: Interface {
    async fn insert(&self, ticket: Ticket) -> Result<()>;

    async fn select(&self, id: TicketId) -> Result<Option<Ticket>>;

    async fn select_all(&self) -> Result<Vec<Ticket>>;

    async fn update(&self, ticket: Ticket) -> Result<()>;
}
