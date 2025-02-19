use crate::dto::{GeneralReadResponseDTO, GeneralTransactionResponseDTO, TransactionType};
use lambda_http::{Body, Response};

pub fn get_success_response(data: String) -> Response<Body> {
    let general_response = GeneralReadResponseDTO {
        status: 200,
        message: "success".to_string(),
        data: serde_json::from_str(&data).unwrap(),
    };

    Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Body::Text(
            serde_json::to_string(&general_response).unwrap(),
        ))
        .unwrap()
}

pub fn get_failed_response(data: String, message: &str) -> Response<Body> {
    let general_response = GeneralReadResponseDTO {
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
pub fn process_success_response(data: String, tx_type: TransactionType) -> Response<Body> {
    //build general response
    let general_response = GeneralTransactionResponseDTO {
        status: 200,
        tx_type: TransactionType::map_tx_type(&tx_type),
        message: "success".to_string(),
        data: serde_json::from_str(&data).unwrap(),
    };

    //submit the reponse into celestia node

    //store it in database for faster retrieving

    Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
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
    let general_response = GeneralTransactionResponseDTO {
        status: 400,
        tx_type: TransactionType::map_tx_type(&tx_type),
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
