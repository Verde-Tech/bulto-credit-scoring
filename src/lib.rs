use log::info;
use stubs::{stub_comprehensive_credit_score, stub_alternative_credit_report, stub_traditional_credit_report};
use tonic::{transport::Server, Request, Response, Status};
use crate::models::{ComprehensiveCreditScore, TraditionalCreditReport, AlternativeCreditReport};


use credit_scoring::{credit_scoring_service_server::{CreditScoringService, CreditScoringServiceServer}, CalculateCreditScoreRequest, CalculateCreditScoreResponse, ScoreType};
// use credit_scoring::::{credit_scoring_server::{CreditScoring, CreditScoringServer}, CalculateCreditScoreRequest, CalculateCreditScoreResponse, ScoreType};
use score::{calculate_comprehensive_score, calculate_traditional_score, calculate_alternative_score};

mod stubs;
mod score;
mod models;
pub mod credit_scoring {
    tonic::include_proto!("credit_scoring"); // The string specified here must match the proto package name
}

#[derive(Default)]
pub struct CreditScoring {}

#[tonic::async_trait]
impl CreditScoringService for CreditScoring {
    async fn calculate_credit_score(
        &self,
        request: Request<CalculateCreditScoreRequest>,
    ) -> Result<Response<CalculateCreditScoreResponse>, Status> {
        let request = request.into_inner();

        let score = match request.score_type() {
            ScoreType::Traditional => {
                let report = stub_traditional_credit_report(); // Replace with actual data retrieval and calculation
                score::calculate_traditional_score(&report)
            },
            ScoreType::Comprehensive => {
                let report = stub_comprehensive_credit_score(); // Replace with actual data retrieval and calculation
                score::calculate_comprehensive_score(&report)
            },
            ScoreType::Alternative => {
                let report = stub_alternative_credit_report(); // Replace with actual data retrieval and calculation
                score::calculate_alternative_score(&report)
            },
        };

        let response = CalculateCreditScoreResponse {
            score: score as i32,
        };

        Ok(Response::new(response))
    }
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = CreditScoring::default();

    Server::builder()
        .add_service(CreditScoringServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
