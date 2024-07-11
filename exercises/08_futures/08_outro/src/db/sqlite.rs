use super::Db;
use crate::data::{Ticket, TicketDto, TicketId};
use anyhow::anyhow;
use anyhow::Result;
use async_trait::async_trait;
use shaku::Component;
use sqlx::sqlite::SqlitePool;

#[derive(Component)]
#[shaku(interface = Db)]
pub struct SqliteImpl {
    pool: SqlitePool,
}

#[async_trait]
impl Db for SqliteImpl {
    async fn insert(&self, ticket: Ticket) -> Result<()> {
        let TicketDto {
            id,
            title,
            description,
            status,
            updated_at,
        } = ticket.into();

        sqlx::query!(
            "INSERT INTO tickets VALUES(?, ?, ?, ?, ?)",
            id,
            title,
            description,
            status,
            updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| anyhow!("[@{} in {}] {:?}", line!(), file!(), e))?;

        Ok(())
    }

    async fn select(&self, id: TicketId) -> Result<Option<Ticket>> {
        let id = id.0.to_string();
        let ticket_dto: Option<TicketDto> =
            sqlx::query_as!(TicketDto, "SELECT * FROM tickets WHERE id = ?", id)
                .fetch_optional(&self.pool)
                .await?;

        let Some(ticket_dto) = ticket_dto else {
            return Ok(None);
        };

        let ticket = ticket_dto.try_into()?;

        Ok(Some(ticket))
    }

    async fn select_all(&self) -> Result<Vec<Ticket>> {
        let ticket_dtos: Vec<TicketDto> = sqlx::query_as!(TicketDto, "SELECT * FROM tickets")
            .fetch_all(&self.pool)
            .await?;

        let tickets = ticket_dtos
            .into_iter()
            .map(|tdto| tdto.try_into())
            .collect::<Result<Vec<Ticket>>>()?;

        Ok(tickets)
    }

    async fn update(&self, ticket: Ticket) -> Result<()> {
        let TicketDto {
            id,
            title,
            description,
            status,
            updated_at,
        } = ticket.into();

        sqlx::query!(
            "UPDATE tickets SET title = ?, description = ?, status = ?, updated_at = ? WHERE id = ?",
            title,
            description,
            status,
            updated_at,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| anyhow!("[@{} in {}] {:?}", line!(), file!(), e))?;

        Ok(())
    }
}
