use crate::schema::test_results;
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
