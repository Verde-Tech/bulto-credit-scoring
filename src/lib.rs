use log::info;

pub mod models;
pub use score::{calculate_comprehensive_credit_score, calculate_traditional_score, calculate_alternative_score};

mod stubs;
mod score;

pub fn run() {
    info!("Library code goes here");
    // Rest of the library code
}
