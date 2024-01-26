use crate::models::*;

pub fn stub_traditional_credit_report() -> TraditionalCreditReport {
    TraditionalCreditReport {
        payment_history: PaymentHistory {
            late_payments: 0,
            on_time_payments: 0,
            missed_payments: 0,
        },
        credit_utilization: CreditUtilization {
            total_balance: 0.0,
            available_credit: 0.0,
        },
        credit_history: CreditHistory {
            oldest_account_age: 0,
            average_account_age: 0.0,
        },
        credit_types: vec![],
        recent_inquiries: 0,
        total_debts: 0.0,
        public_records: PublicRecords {
            bankruptcies: 0,
            foreclosures: 0,
            legal_judgments: 0,
            tax_liens: 0,
        },
    }
}


pub fn stub_alternative_credit_report() -> AlternativeCreditReport {
    AlternativeCreditReport {
        utility_payments: UtilityPayments {
            electric: PaymentHistory {
                late_payments: 0,
                on_time_payments: 0,
                missed_payments: 0,
            },
            water: PaymentHistory {
                late_payments: 0,
                on_time_payments: 0,
                missed_payments: 0,
            },
            internet: PaymentHistory {
                late_payments: 0,
                on_time_payments: 0,
                missed_payments: 0,
            },
        },
        rent_payments: PaymentHistory {
            late_payments: 0,
            on_time_payments: 0,
            missed_payments: 0,
        },
        telecommunication_payments: TelecomPayments {
            mobile: PaymentHistory {
                late_payments: 0,
                on_time_payments: 0,
                missed_payments: 0,
            },
            landline: PaymentHistory {
                late_payments: 0,
                on_time_payments: 0,
                missed_payments: 0,
            },
        },
        employment_history: EmploymentHistory {
            current_employer: EmploymentStatus::Other,
            employment_duration: 0,
        },
        educational_background: EducationalBackground {
            highest_level: EducationLevel::Other,
            field_of_study: EmploymentStatus::Other,
        },
        social_media_activity: SocialMediaActivity {
            activity_level: 0,
        },
        e_commerce_transactions: ECommerceTransactions {
            frequency: 0,
            diversity: 0,
        },
        bank_account_info: BankAccountInfo {
            average_balance: 0.0,
            overdraft_history: 0,
        },
        mobile_money_data: MobileMoneyData {
            transaction_history: vec![],
            account_balance: 0.0,
            account_duration: 0,
            transaction_frequency: TransactionFrequency::VeryLow,
            transaction_partners: vec![],
        },
    }
}


pub fn stub_comprehensive_credit_score() -> ComprehensiveCreditScore {
    ComprehensiveCreditScore {
        traditional_credit_score: stub_traditional_credit_report(),
        alternative_credit_score: stub_alternative_credit_report(),
    }
}

