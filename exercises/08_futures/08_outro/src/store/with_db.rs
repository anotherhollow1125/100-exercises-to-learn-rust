use super::{Store, StoreError, UpdateResult};
use crate::data::{Status, Ticket, TicketDraft, TicketId, TicketPatch};
use crate::db::Db;
use anyhow::anyhow;
use async_trait::async_trait;
use chrono::Local;
use shaku::Component;
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Component)]
#[shaku(interface = Store)]
pub struct StoreImpl {
    #[shaku(inject)]
    db: Arc<dyn Db>,
    #[shaku(default)]
    cache: Mutex<BTreeMap<TicketId, Ticket>>,
}

#[async_trait]
impl Store for StoreImpl {
    async fn add_ticket(&self, ticket: TicketDraft) -> Result<TicketId, StoreError> {
        let id = TicketId(Uuid::now_v7());

        let ticket = Ticket {
            id,
            title: ticket.title,
            description: ticket.description,
            status: Status::ToDo,
            updated_at: Local::now().fixed_offset(),
        };

        self.db.insert(ticket.clone()).await?;
        self.cache.lock().await.insert(id, ticket);

        Ok(id)
    }

    async fn get(&self, id: TicketId) -> Result<Option<Ticket>, StoreError> {
        if let Some(ticket) = self.cache.lock().await.get(&id) {
            return Ok(Some(ticket.clone()));
        }

        if let Some(ticket) = self.db.select(id).await? {
            self.cache.lock().await.insert(id, ticket.clone());

            return Ok(Some(ticket));
        }

        Ok(None)
    }

    async fn get_all(&self) -> Result<Vec<Ticket>, StoreError> {
        let tickets = self.db.select_all().await?;

        // Sync cache
        let mut cache = self.cache.lock().await;
        *cache = tickets
            .iter()
            .cloned()
            .map(|t| (t.id, t))
            .collect::<BTreeMap<TicketId, Ticket>>();

        Ok(tickets)
    }

    async fn update(
        &self,
        patch: TicketPatch,
    ) -> Result<UpdateResult<TicketPatch, Ticket>, StoreError> {
        let TicketPatch {
            id,
            version,
            title,
            description,
            status,
        } = patch.clone();

        let mut ticket = match self.get(id).await {
            Ok(Some(ticket)) => ticket,
            Ok(None) => {
                return Err(anyhow!("[@ {} in {}] Ticket Not Found !", file!(), line!()).into())
            }
            Err(e) => return Err(e),
        };

        if version < ticket.updated_at {
            return Ok(UpdateResult::Conflict {
                yours: patch,
                now: ticket,
            });
        }

        ticket.updated_at = Local::now().fixed_offset();

        if let Some(title) = title {
            ticket.title = title;
        }

        if let Some(description) = description {
            ticket.description = description;
        }

        if let Some(status) = status {
            ticket.status = status;
        }

        self.db.update(ticket.clone()).await?;
        self.cache.lock().await.insert(id, ticket.clone());

        Ok(UpdateResult::Accept(ticket))
    }
}
