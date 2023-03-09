use diesel::prelude::*;
use chrono::{ NaiveDateTime };
use crate::schema::test_results;
use bigdecimal::{ BigDecimal };

#[derive(Queryable)]
#[diesel(table_name = test_results)]
pub struct TestResult {
    pub id: i32,
    pub duration: BigDecimal,
    pub run_at: NaiveDateTime,
    pub name: String,
    pub status: String,
}

#[derive(Insertable)]
#[diesel(table_name = test_results)]
pub struct NewTestResult<'a> {
    pub duration: BigDecimal,
    pub run_at: NaiveDateTime,
    pub name: &'a str,
    pub status: &'a str,
}