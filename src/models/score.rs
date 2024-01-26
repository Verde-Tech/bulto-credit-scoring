use super::*;

fn calculate_comprehensive_credit_score(comprehensive_score: &ComprehensiveCreditScore) -> f64 {
    let weight_traditional = 0.6;
    let weight_alternative = 0.4;

    let traditional_score = calculate_traditional_score(&comprehensive_score.traditional_credit_score);
    let alternative_score = calculate_alternative_score(&comprehensive_score.alternative_credit_score);

    let combined_score = (traditional_score * weight_traditional) + (alternative_score * weight_alternative);

    normalize_scale_score(combined_score)
}

fn calculate_traditional_score(credit_report: &TraditionalCreditReport) -> f64 {
    let weight_payment_history = 0.35;
    let weight_credit_utilization = 0.30;
    let weight_credit_history = 0.15;
    let weight_credit_types = 0.10;
    let weight_recent_inquiries = 0.05;
    let weight_total_debts = 0.05;

    let score_payment_history = calculate_payment_history_score(&credit_report.payment_history);
    let score_credit_utilization = calculate_credit_utilization_score(&credit_report.credit_utilization);
    let score_credit_history = calculate_credit_history_score(&credit_report.credit_history);
    let score_credit_types = calculate_credit_types_score(&credit_report.credit_types);
    let score_recent_inquiries = calculate_recent_inquiries_score(credit_report.recent_inquiries);
    let score_total_debts = calculate_total_debts_score(credit_report.total_debts);

    (score_payment_history * weight_payment_history) +
    (score_credit_utilization * weight_credit_utilization) +
    (score_credit_history * weight_credit_history) +
    (score_credit_types * weight_credit_types) +
    (score_recent_inquiries * weight_recent_inquiries) +
    (score_total_debts * weight_total_debts)
}

fn calculate_alternative_score(alternative_score: &AlternativeCreditReport) -> f64 {
    let weight_utility_payments = 0.20;
    let weight_rent_payments = 0.15;
    let weight_telecom_payments = 0.15;
    let weight_employment_history = 0.10;
    let weight_educational_background = 0.10;
    let weight_social_media = 0.10;
    let weight_e_commerce = 0.10;
    let weight_bank_info = 0.10;

    let score_utility_payments = calculate_utility_payments_score(&alternative_score.utility_payments);
    let score_rent_payments = calculate_rent_payments_score(&alternative_score.rent_payments);
    let score_telecom_payments = calculate_telecom_payments_score(&alternative_score.telecommunication_payments);
    let score_employment_history = calculate_employment_history_score(&alternative_score.employment_history);
    let score_educational_background = calculate_educational_background_score(&alternative_score.educational_background);
    let score_social_media = calculate_social_media_activity_score(&alternative_score.social_media_activity);
    let score_e_commerce = calculate_e_commerce_transactions_score(&alternative_score.e_commerce_transactions);
    let score_bank_info = calculate_bank_account_info_score(&alternative_score.bank_account_info);

    (score_utility_payments * weight_utility_payments) +
    (score_rent_payments * weight_rent_payments) +
    (score_telecom_payments * weight_telecom_payments) +
    (score_employment_history * weight_employment_history) +
    (score_educational_background * weight_educational_background) +
    (score_social_media * weight_social_media) +
    (score_e_commerce * weight_e_commerce) +
    (score_bank_info * weight_bank_info)
}

fn normalize_scale_score(score: f64) -> f64 {
    // Placeholder logic for normalization and scaling
    // For example, scale a score from 0-1 range to 300-850
    300.0 + (score * 550.0)
}
