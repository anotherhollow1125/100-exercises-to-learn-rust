use super::StoreModule;
use crate::data::{TicketDraftDto, TicketDto, TicketId, TicketPatchDto};
use crate::store::Store;
use crate::store::UpdateResult;
use axum::extract::{Json, Path, State};
use axum::response::{IntoResponse, Result};
use http::status::StatusCode;
use shaku::HasComponent;
use std::sync::Arc;
use uuid::Uuid;

#[allow(unused)]
pub async fn get_ticket_by_id(
    State(module): State<Arc<StoreModule>>,
    Path(ticket_id): Path<Uuid>,
) -> Result<Json<TicketDto>> {
    let store: &dyn Store = module.resolve_ref();

    let ticket = store.get(TicketId(ticket_id)).await?;

    match ticket {
        Some(t) => Ok(Json(t.into())),
        None => Err(StatusCode::NOT_FOUND.into()),
    }
}

#[allow(unused)]
pub async fn get_all_ticket(
    State(module): State<Arc<StoreModule>>,
) -> Result<Json<Vec<TicketDto>>> {
    let store: &dyn Store = module.resolve_ref();

    let tickets = store.get_all().await?;

    Ok(Json(tickets.into_iter().map(TicketDto::from).collect()))
}

#[allow(unused)]
pub async fn create_ticket(
    State(module): State<Arc<StoreModule>>,
    Json(draft): Json<TicketDraftDto>,
) -> Result<Json<Uuid>> {
    let store: &dyn Store = module.resolve_ref();

    let draft = draft.try_into().map_err(|e| {
        (StatusCode::BAD_REQUEST, format!("Invalid Format: {:?}", e)).into_response()
    })?;

    let id = store.add_ticket(draft).await?;

    Ok(Json(id.0))
}

#[allow(unused)]
pub async fn update_ticket(
    State(module): State<Arc<StoreModule>>,
    Json(patch): Json<TicketPatchDto>,
) -> Result<Json<UpdateResult<TicketPatchDto, TicketDto>>> {
    let store: &dyn Store = module.resolve_ref();

    let patch = patch.try_into().map_err(|e| {
        (StatusCode::BAD_REQUEST, format!("Invalid Format: {:?}", e)).into_response()
    })?;

    let update_result = store.update(patch).await?.into_dto();

    Ok(Json(update_result))
}
