use crate::model::{ModelController, Ticket, TicketForCreate};
use crate::Result;
use axum::extract::{Path, State};
use axum::routing::delete;
use axum::Json;
use axum::{routing::post, Router};

pub async fn create_ticket(
    State(model_controller): State<ModelController>,
    Json(ticket_for_create): Json<TicketForCreate>
) -> Result<Json<Ticket>> {
    println!(">> {:<12?} - create_ticket", "HANDLER");

    let ticket = model_controller.create(ticket_for_create).await?;

    Ok(Json(ticket))
}

pub async fn list_tickets(
    State(model_controller): State<ModelController>
) -> Result<Json<Vec<Ticket>>> {
    println!(">> {:<12?} - list_tickets", "HANDLER");

    let tickets = model_controller.list_tickets().await?;

    Ok(Json(tickets))
}

pub async fn delete_ticket(
    State(model_controller): State<ModelController>,
    Path(id): Path<u64>
) -> Result<Json<Ticket>> {
    println!(">> {:<12?} - delete_ticket", "HANDLER");

    let ticket = model_controller.delete(id).await?;

    Ok(Json(ticket))    
}

pub fn routes(model_controller: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(model_controller)
}