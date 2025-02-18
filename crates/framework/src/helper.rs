use crate::dto::GeneralResponseDTO;
use lambda_http::{Body, Response};

pub fn get_success_response(data: String) -> Response<Body> {
    let general_response = GeneralResponseDTO {
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
