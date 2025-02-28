use app::usecase::{chain_service::ChainService, database_service::DatabaseService};
use domain::shared::dtos::{CelestiaSubmitModel, GeneralResponseDTO, TransactionType};
use infra::{celestia_repository::CelestiaRepository, postgres_repository::PostgresRepository};
use lambda_http::{Body, Response};
use serde_json::Value;
use std::sync::Arc;

pub fn get_success_response(data: String) -> Response<Body> {
    let general_response = GeneralResponseDTO {
        status: 200,
        message: "success".to_string(),
        data: serde_json::from_str(&data).unwrap(),
    };

    Response::builder()
        .status(200)
        .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .header("Access-Control-Allow-Headers", "Content-Type")
        .body(Body::Text(
            serde_json::to_string(&general_response).unwrap(),
        ))
        .unwrap()
}

pub fn get_failed_response(data: String, message: &str) -> Response<Body> {
    let general_response = GeneralResponseDTO {
        status: 400,
        message: message.to_string(),
        data: serde_json::from_str(&data).unwrap(),
    };

    Response::builder()
        .status(400)
        .body(Body::Text(
            serde_json::to_string(&general_response).unwrap(),
        ))
        .unwrap()
}

//process if reponse is belong to transaction
pub async fn process_success_response(
    data: Value,
    tx_type: TransactionType,
    user_address: &str,
) -> Response<Body> {
    //build general response
    let general_response = GeneralResponseDTO {
        status: 200,
        message: "success".to_string(),
        data: data.clone(),
    };

    //submit the response into celestia node
    let celestia_repository = Arc::new(CelestiaRepository::new().await);
    let chain_service = ChainService::new(celestia_repository.clone());
    let celestia_submit_model = CelestiaSubmitModel {
        tx_type: TransactionType::map_tx_type(&tx_type),
        data: data.clone(),
    };
    let blob = chain_service
        .build_blob(
            user_address.to_lowercase().as_str(),
            serde_json::to_value(celestia_submit_model).unwrap(),
        )
        .await
        .unwrap();
    let height = chain_service.submit(&[blob]).await.unwrap();
    //get again to check
    //store it in database for faster retrieving
    let db_repository = Arc::new(PostgresRepository::new().await);
    let db_service = DatabaseService::new(db_repository.clone());
    let rs = db_service
        .add_new_transaction(height.to_string().as_str(), user_address)
        .await
        .unwrap();

    Response::builder()
        .status(200)
        .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .header("Access-Control-Allow-Headers", "Content-Type")
        .body(Body::Text(
            serde_json::to_string(&general_response).unwrap(),
        ))
        .unwrap()
}

pub fn process_failed_response(
    data: String,
    message: &str,
    tx_type: TransactionType,
) -> Response<Body> {
    let general_response = GeneralResponseDTO {
        status: 400,
        message: message.to_string(),
        data: serde_json::from_str(&data).unwrap(),
    };

    Response::builder()
        .status(400)
        .body(Body::Text(
            serde_json::to_string(&general_response).unwrap(),
        ))
        .unwrap()
}
