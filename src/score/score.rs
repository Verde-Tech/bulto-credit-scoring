use crate::models::reports::*;

pub fn calculate_comprehensive_credit_score(comprehensive_score: &ComprehensiveCreditScore) -> f64 {
    let weight_traditional = 0.6;
    let weight_alternative = 0.4;

    let traditional_score = calculate_traditional_score(&comprehensive_score.traditional_credit_score);
    let alternative_score = calculate_alternative_score(&comprehensive_score.alternative_credit_score);

    let combined_score = (traditional_score * weight_traditional) + (alternative_score * weight_alternative);

    normalize_scale_score(combined_score)
}

pub fn calculate_traditional_score(credit_report: &TraditionalCreditReport) -> f64 {
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

pub fn calculate_alternative_score(alternative_score: &AlternativeCreditReport) -> f64 {
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
fn calculate_payment_history_score(payment_history: &PaymentHistory) -> f64 {
    // Simple implementation based on the ratio of on-time payments to total payments
    let total_payments = payment_history.late_payments + payment_history.on_time_payments + payment_history.missed_payments;
    if total_payments == 0 {
        return 0.0;
    }
    (payment_history.on_time_payments as f64) / (total_payments as f64)
}

fn calculate_credit_utilization_score(credit_utilization: &CreditUtilization) -> f64 {
    // Simple implementation based on the ratio of available credit to total balance
    if credit_utilization.total_balance == 0.0 {
        return 1.0;
    }
    credit_utilization.available_credit / credit_utilization.total_balance
}

fn calculate_credit_history_score(credit_history: &CreditHistory) -> f64 {
    // Simple implementation based on the age of accounts
    let age_score = credit_history.oldest_account_age as f64 / 100.0; // Assuming 100 is the max age
    let average_age_score = credit_history.average_account_age / 100.0; // Assuming 100 is the max average age
    (age_score + average_age_score) / 2.0
}

fn calculate_credit_types_score(credit_types: &[CreditAccountType]) -> f64 {
    // Simple implementation based on the diversity of credit types
    let types_count = credit_types.len();
    types_count as f64 / 10.0 // Assuming 10 is the max number of account types
}

fn calculate_recent_inquiries_score(recent_inquiries: u32) -> f64 {
    // Simple implementation based on the number of recent inquiries
    1.0 / (1.0 + recent_inquiries as f64) // More inquiries, lower the score
}

fn calculate_total_debts_score(total_debts: f64) -> f64 {
    // Simple implementation based on the total amount of debts
    1.0 / (1.0 + total_debts / 10000.0) // Assuming 10000 is a significant debt amount
}

fn calculate_utility_payments_score(utility_payments: &UtilityPayments) -> f64 {
    // Simple implementation based on the payment history of utilities
    let electric_score = calculate_payment_history_score(&utility_payments.electric);
    let water_score = calculate_payment_history_score(&utility_payments.water);
    let internet_score = calculate_payment_history_score(&utility_payments.internet);
    (electric_score + water_score + internet_score) / 3.0
}

fn calculate_rent_payments_score(rent_payments: &PaymentHistory) -> f64 {
    // Simple implementation based on the payment history of rent
    calculate_payment_history_score(rent_payments)
}

fn calculate_telecom_payments_score(telecom_payments: &TelecomPayments) -> f64 {
    // Simple implementation based on the payment history of telecom services
    let mobile_score = calculate_payment_history_score(&telecom_payments.mobile);
    let landline_score = calculate_payment_history_score(&telecom_payments.landline);
    (mobile_score + landline_score) / 2.0
}

fn calculate_employment_history_score(employment_history: &EmploymentHistory) -> f64 {
    // Simple implementation based on the duration of employment
    employment_history.employment_duration as f64 / 120.0 // Assuming 120 months (10 years) is a significant duration
}

fn calculate_educational_background_score(educational_background: &EducationalBackground) -> f64 {
    // Simple implementation based on the level of education
    match educational_background.highest_level {
        EducationLevel::HighSchool => 0.2,
        EducationLevel::Associate => 0.4,
        EducationLevel::Bachelor => 0.6,
        EducationLevel::Master => 0.8,
        EducationLevel::Doctorate => 1.0,
        EducationLevel::Other => 0.5,
    }
}

fn calculate_social_media_activity_score(social_media_activity: &SocialMediaActivity) -> f64 {
    // Simple implementation based on the level of activity
    social_media_activity.activity_level as f64 / 100.0 // Assuming 100 is the max activity level
}

fn calculate_e_commerce_transactions_score(e_commerce_transactions: &ECommerceTransactions) -> f64 {
    // Simple implementation based on the frequency and diversity of transactions
    let frequency_score = e_commerce_transactions.frequency as f64 / 100.0; // Assuming 100 is the max frequency
    let diversity_score = e_commerce_transactions.diversity as f64 / 100.0; // Assuming 100 is the max diversity
    (frequency_score + diversity_score) / 2.0
}

fn calculate_bank_account_info_score(bank_account_info: &BankAccountInfo) -> f64 {
    // Simple implementation based on the average balance and overdraft history
    let balance_score = bank_account_info.average_balance / 10000.0; // Assuming 10000 is a significant balance
    let overdraft_penalty = bank_account_info.overdraft_history as f64 * 0.1; // Each overdraft reduces the score
    balance_score - overdraft_penalty
}
