use crate::dto::{TranfersRequestDTO, TransfersResponseDTO};
use app::{self, usecase::payment_service::PaymentService};
use infra::{self, circle_repository::CircleRepository};
use rocket::serde::json::Json;
use std::sync::Arc;

#[post(
    "/transfers",
    format = "application/json",
    data = "<transfers_request>"
)]
pub async fn transfers(transfers_request: Json<TranfersRequestDTO>) -> Json<TransfersResponseDTO> {
    let repository = Arc::new(CircleRepository::new());
    let payment_service = PaymentService::new(repository);

    match payment_service
        .transfers(
            &transfers_request.amount,
            &transfers_request.chain,
            &transfers_request.destination_address,
        )
        .await
    {
        Ok(response) => Json(TransfersResponseDTO {
            status: "200".to_string(),
            message: "success".to_string(),
            data: response,
        }),
        Err(error) => Json(TransfersResponseDTO {
            status: "400".to_string(),
            message: error,
            data: "".to_string(),
        }),
    }
}
