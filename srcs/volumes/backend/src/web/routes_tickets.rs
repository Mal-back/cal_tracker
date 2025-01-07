//use axum::extract::Path;
//use axum::routing::{delete, post};
//use axum::Router;
//use axum::{extract::State, Json};
//use tracing::debug;
//
//use crate::ctx::Ctx;
////use crate::model::{ModelManager, Ticket, TicketForCreate};
//use crate::Result;

//pub fn routes(mc: ModelManager) -> Router {
//    Router::new()
//        .route("/tickets/", post(create_tickets).get(list_tickets))
//        .route("/tickets/:id", delete(delete_ticket))
//        .with_state(mc)
//}
//
//async fn create_tickets(
//    State(mc): State<ModelManager>,
//    ctx: Ctx,
//    Json(ticket_fc): Json<TicketForCreate>,
//) -> Result<Json<Ticket>> {
//    debug!("{:<12} - Create ticket", "HANDLER",);
//    let created = mc.create_ticket(ticket_fc, ctx).await?;
//
//    Ok(Json(created))
//}
//
//async fn list_tickets(
//    State(mc): State<ModelManager>,
//    ctx: Ctx,
//    ) -> Result<Json<Vec<Ticket>>> {
//    debug!("{:<12} - List tickets", "HANDLER",);
//    let list = mc.list_tickets(ctx).await?;
//
//    Ok(Json(list))
//}
//
//async fn delete_ticket(
//    State(mc): State<ModelManager>,
//    ctx: Ctx,
//    Path(id): Path<u32>,
//) -> Result<Json<Ticket>> {
//    debug!("{:<12} - Delete ticket", "HANDLER",);
//    let deleted = mc.delete_ticket(id, ctx).await?;
//
//    Ok(Json(deleted))
//}
