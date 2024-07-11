use std::str::FromStr;

use anyhow::anyhow;
use chrono::{DateTime, FixedOffset};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use ticket_fields::{TicketDescription, TicketTitle};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TicketId(pub Uuid);

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
    pub updated_at: DateTime<FixedOffset>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

#[derive(Clone, Display, Debug, Copy, PartialEq, Eq)]
pub enum Status {
    #[display(fmt = "to_do")]
    ToDo,
    #[display(fmt = "in_progress")]
    InProgress,
    #[display(fmt = "done")]
    Done,
}

impl FromStr for Status {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "to_do" => Ok(Status::ToDo),
            "in_progress" => Ok(Status::InProgress),
            "done" => Ok(Status::Done),
            _ => Err(anyhow!("Invalid Status: {}", s)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TicketPatch {
    pub id: TicketId,
    pub version: DateTime<FixedOffset>,
    pub title: Option<TicketTitle>,
    pub description: Option<TicketDescription>,
    pub status: Option<Status>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TicketDto {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: String,
    pub updated_at: String,
}

impl TryFrom<TicketDto> for Ticket {
    type Error = anyhow::Error;

    fn try_from(value: TicketDto) -> Result<Self, Self::Error> {
        let TicketDto {
            id,
            title,
            description,
            status,
            updated_at,
        } = value;

        Ok(Self {
            id: TicketId(Uuid::from_str(&id)?),
            title: title.try_into()?,
            description: description.try_into()?,
            status: Status::from_str(&status)?,
            updated_at: DateTime::<FixedOffset>::from_str(&updated_at)?,
        })
    }
}

impl From<Ticket> for TicketDto {
    fn from(value: Ticket) -> Self {
        let Ticket {
            id,
            title,
            description,
            status,
            updated_at,
        } = value;

        Self {
            id: id.0.to_string(),
            title: title.into(),
            description: description.into(),
            status: status.to_string(),
            updated_at: updated_at.to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TicketDraftDto {
    pub title: String,
    pub description: String,
}

impl TryFrom<TicketDraftDto> for TicketDraft {
    type Error = anyhow::Error;

    fn try_from(value: TicketDraftDto) -> Result<Self, Self::Error> {
        let TicketDraftDto { title, description } = value;

        Ok(Self {
            title: title.try_into()?,
            description: description.try_into()?,
        })
    }
}

impl From<TicketDraft> for TicketDraftDto {
    fn from(value: TicketDraft) -> Self {
        let TicketDraft { title, description } = value;

        Self {
            title: title.into(),
            description: description.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TicketPatchDto {
    pub id: String,
    pub version: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}

impl TryFrom<TicketPatchDto> for TicketPatch {
    type Error = anyhow::Error;

    fn try_from(value: TicketPatchDto) -> Result<Self, Self::Error> {
        let TicketPatchDto {
            id,
            version,
            title,
            description,
            status,
        } = value;

        Ok(Self {
            id: TicketId(Uuid::from_str(&id)?),
            version: DateTime::<FixedOffset>::from_str(&version)?,
            title: title.map(|t| t.try_into()).transpose()?,
            description: description.map(|d| d.try_into()).transpose()?,
            status: status.map(|s| Status::from_str(&s)).transpose()?,
        })
    }
}

impl From<TicketPatch> for TicketPatchDto {
    fn from(value: TicketPatch) -> Self {
        let TicketPatch {
            id,
            version,
            title,
            description,
            status,
        } = value;

        Self {
            id: id.0.to_string(),
            version: version.to_string(),
            title: title.map(|t| t.into()),
            description: description.map(|d| d.into()),
            status: status.map(|s| s.to_string()),
        }
    }
}
