use tonic::{transport::Server, Request, Response, Status};
use crate::models::score::{self, ComprehensiveCreditScore, TraditionalCreditReport, AlternativeCreditReport};
use crate::credit_scoring::{credit_scoring_server::{CreditScoring, CreditScoringServer}, CalculateCreditScoreRequest, CalculateCreditScoreResponse, ScoreType};

pub mod credit_scoring {
    tonic::include_proto!("credit_scoring"); // The string specified here must match the proto package name
}

#[derive(Default)]
pub struct CreditScoringService {}

#[tonic::async_trait]
impl CreditScoring for CreditScoringService {
    async fn calculate_credit_score(
        &self,
        request: Request<CalculateCreditScoreRequest>,
    ) -> Result<Response<CalculateCreditScoreResponse>, Status> {
        let request = request.into_inner();

        let score = match request.score_type() {
            ScoreType::Traditional => {
                let report = TraditionalCreditReport::default(); // Replace with actual data retrieval and calculation
                score::calculate_traditional_score(&report)
            },
            ScoreType::Comprehensive => {
                let report = ComprehensiveCreditScore::default(); // Replace with actual data retrieval and calculation
                score::calculate_comprehensive_score(&report)
            },
            ScoreType::Alternative => {
                let report = AlternativeCreditReport::default(); // Replace with actual data retrieval and calculation
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
    let service = CreditScoringService::default();

    Server::builder()
        .add_service(CreditScoringServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
