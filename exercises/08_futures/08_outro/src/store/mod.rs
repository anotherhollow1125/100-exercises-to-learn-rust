use crate::data::{Ticket, TicketDraft, TicketDto, TicketId, TicketPatch, TicketPatchDto};
use async_trait::async_trait;
use axum::response::{IntoResponse, Response};
use http::status::StatusCode;
use serde::{ser::SerializeStruct, Serialize};
use shaku::Interface;

pub mod with_db;
pub use with_db::StoreImpl;

pub enum UpdateResult<P, T> {
    Conflict { yours: P, now: T },
    Accept(T),
}

#[derive(Debug, thiserror::Error)]
#[error("Store Error!: {0}")]
pub struct StoreError(#[from] anyhow::Error);

impl<P, T> Serialize for UpdateResult<P, T>
where
    P: Serialize,
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            UpdateResult::Conflict { yours, now } => {
                let mut s = serializer.serialize_struct("Conflict", 2)?;
                s.serialize_field("yours", &yours)?;
                s.serialize_field("now", &now)?;
                s.end()
            }
            UpdateResult::Accept(t) => serializer.serialize_newtype_struct("Accept", &t),
        }
    }
}

#[async_trait]
pub trait Store: Interface {
    async fn add_ticket(&self, ticket: TicketDraft) -> Result<TicketId, StoreError>;

    async fn get(&self, id: TicketId) -> Result<Option<Ticket>, StoreError>;

    async fn get_all(&self) -> Result<Vec<Ticket>, StoreError>;

    async fn update(
        &self,
        patch: TicketPatch,
    ) -> Result<UpdateResult<TicketPatch, Ticket>, StoreError>;
}

impl IntoResponse for StoreError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}

impl UpdateResult<TicketPatch, Ticket> {
    pub fn into_dto(self) -> UpdateResult<TicketPatchDto, TicketDto> {
        match self {
            UpdateResult::Conflict { yours, now } => UpdateResult::Conflict {
                yours: yours.into(),
                now: now.into(),
            },
            UpdateResult::Accept(t) => UpdateResult::Accept(t.into()),
        }
    }
}
