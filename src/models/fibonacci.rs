use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct FibonacciRequest {
    pub n: u32,
}

#[derive(Serialize)]
pub struct FibonacciResponse {
    pub n: u32,
    pub result: u64,
}

#[derive(Deserialize)]
pub struct FibonacciQuery {
    pub n: Option<u32>,
}