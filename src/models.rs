use crate::schema::test_results;
use crate::schema::test_summaries;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable)]
#[diesel(table_name = test_results)]
pub struct TestResult {
    pub id: i32,
    pub duration: BigDecimal,
    pub run_at: NaiveDateTime,
    pub name: String,
    pub status: String,
    pub error_message: Option<String>,
    pub job_name: String,
}

#[derive(Insertable, Clone)]
#[diesel(table_name = test_results)]
pub struct NewTestResult<'a> {
    pub duration: BigDecimal,
    pub run_at: NaiveDateTime,
    pub name: &'a str,
    pub status: &'a str,
    pub error_message: Option<String>,
    pub job_name: &'a str,
}

#[derive(Queryable)]
#[diesel(table_name = test_summaries)]
pub struct TestSummary {
    pub id: i32,
    pub name: String,
    pub errors: i32,
    pub failures: i32,
    pub skipped: i32,
    pub tests: i32,
    pub time: BigDecimal,
    pub timestamp: NaiveDateTime,
}

#[derive(Insertable, Clone)]
#[diesel(table_name = test_summaries)]
pub struct NewTestSummary<'a> {
    pub name: &'a str,
    pub errors: &'a i32,
    pub failures: &'a i32,
    pub skipped: &'a i32,
    pub tests: &'a i32,
    pub time: BigDecimal,
    pub timestamp: NaiveDateTime,
}
