pub struct TraditionalCreditReport {
    pub payment_history: PaymentHistory,
    pub credit_utilization: CreditUtilization,
    pub credit_history: CreditHistory,
    pub credit_types: Vec<CreditAccountType>,
    pub recent_inquiries: u32,
    pub total_debts: f64,
    pub public_records: PublicRecords,
}

pub struct AlternativeCreditReport {
    pub utility_payments: UtilityPayments,
    pub rent_payments: PaymentHistory,
    pub telecommunication_payments: TelecomPayments,
    pub employment_history: EmploymentHistory,
    pub educational_background: EducationalBackground,
    pub social_media_activity: SocialMediaActivity,
    pub e_commerce_transactions: ECommerceTransactions,
    pub bank_account_info: BankAccountInfo,
    pub mobile_money_data: MobileMoneyData,
}

pub struct ComprehensiveCreditScore {
    pub traditional_credit_score: TraditionalCreditReport,
    pub alternative_credit_score: AlternativeCreditReport,
}

pub struct PaymentHistory {
    pub late_payments: u32,
    pub on_time_payments: u32,
    pub missed_payments: u32, // Number of missed payments
}

pub struct CreditUtilization {
    pub total_balance: f64, // Total balance on credit accounts
    pub available_credit: f64, // Total available credit across accounts
}

pub struct CreditHistory {
    pub oldest_account_age: u32, // Age of the oldest account
    pub average_account_age: f64, // Average age of all accounts
}

pub struct CreditAccountType {
    pub account_type: String, // e.g., 'Credit Card', 'Mortgage'
    pub account_balance: f64, // Balance of this account type
}

pub struct PublicRecords {
    pub bankruptcies: u32,
    pub foreclosures: u32,
    pub legal_judgments: u32,
    pub tax_liens: u32, // Number of tax liens
}

pub struct UtilityPayments {
    pub electric: PaymentHistory,
    pub water: PaymentHistory,
    pub internet: PaymentHistory,
}

pub struct TelecomPayments {
    pub mobile: PaymentHistory,
    pub landline: PaymentHistory,
}

pub struct EmploymentHistory {
    pub current_employer: EmploymentStatus,
    pub employment_duration: u32, // in months
}

pub struct EducationalBackground {
    pub highest_level: EducationLevel,
    pub field_of_study: EmploymentStatus,
}

// Enum for Educational Levels
pub enum EducationLevel {
    HighSchool,
    Associate,
    Bachelor,
    Master,
    Doctorate,
    Other,
}

// Enum for Employment Status
pub enum EmploymentStatus {
    Employed,
    Unemployed,
    SelfEmployed,
    Student,
    Retired,
    Other,
}

pub struct SocialMediaActivity {
    pub activity_level: u32,
}

pub struct ECommerceTransactions {
    pub frequency: u32,
    pub diversity: u32,
}

pub struct BankAccountInfo {
    pub average_balance: f64,
    pub overdraft_history: u32,
}

pub struct MobileMoneyData {
    pub transaction_history: Vec<MobileMoneyTransaction>,
    pub account_balance: f64,
    pub account_duration: u32, // in months
    pub transaction_frequency: TransactionFrequency,
    pub transaction_partners: Vec<TransactionPartner>, // could be anonymized IDs
}

pub struct MobileMoneyTransaction {
    pub transaction_type: TransactionType,
    pub amount: f64,
    pub date: String, // ISO 8601 format or use a date-time library
}

pub enum TransactionType {
    Deposit,
    Withdrawal,
    Payment,
    Other,
}

pub enum TransactionFrequency {
    VeryHigh,
    High,
    Moderate,
    Low,
    VeryLow,
}

pub enum TransactionPartner {
    Business,
    Individual,
    Government,
    NGO,
    Other,
}