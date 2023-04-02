use std::fs;

use self::models::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use minerva::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;
use urlencoding::{decode, encode};

fn main() {
    use self::schema::test_results::dsl::*;

    let connection = &mut establish_connection();

    let path = "C:\\Users\\dwalleck\\repos\\junitxml-result-scraper";
    for entry_o in fs::read_dir(path).unwrap() {
        let entry = entry_o.unwrap();
        println!("Processing {:?}", entry.path().display());
        let path = entry.path();
        let raw_results = fs::read_to_string(path).unwrap();

        let results = raw_results.replace("<>", "").replace("&", "");
        let re = Regex::new(r"<failure message=(?P<msg>.*)>(?P<content>.*)</failure>").unwrap();
        let mut other_results = results.clone();
        let captures = re.captures_iter(results.as_str());
        for cap in captures {
            let message = encode(cap.get(1).unwrap().as_str()).into_owned();
            let content = encode(cap.get(2).unwrap().as_str()).into_owned();

            other_results = re
                .replace_all(
                    other_results.as_str(),
                    format!(
                        "<failure message=\"{}\">{}</failure>",
                        message.to_string(),
                        content.to_string()
                    )
                    .as_str(),
                )
                .to_string();
        }
        let ts: TestSuites = from_str(&other_results.to_string()).unwrap();

        for suite in ts.test_suites {
            let tcs: Vec<TestCase> = suite.test_cases;
            let mut all_results: Vec<NewTestResult> = Vec::new();
            let time_run = suite.timestamp.parse().unwrap();
            for i in 0..tcs.len() {
                let msg = tcs[i].failure.as_ref().map_or(None, |f| {
                    Some(decode(&f.message.as_str()).unwrap().into_owned())
                });
                let mut statusz = tcs[i].failure.as_ref().map_or("Passed", |_| "Failed");
                let v: Vec<Skipped> = Vec::new();
                let skipped_length = tcs[i].skipped.as_ref().map_or(&v, |x| x).len();
                if skipped_length > 0 {
                    statusz = "Skipped"
                }

                let new_result = NewTestResult {
                    duration: tcs[i].time.parse().unwrap(),
                    run_at: time_run,
                    name: &tcs[i].name,
                    status: &statusz,
                    error_message: msg,
                    job_name: &suite.hostname,
                };
                all_results.push(new_result);
            }

            diesel::insert_into(test_results)
                .values(&all_results)
                .execute(connection)
                .expect("Error saving test result");
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TestSuites {
    #[serde(rename = "$value")]
    test_suites: Vec<TestSuite>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TestSuite {
    name: String,
    tests: u32,
    errors: u32,
    failures: u32,
    skipped: u32,
    hostname: String,
    time: String,
    timestamp: String,
    #[serde(rename = "$value")]
    test_cases: Vec<TestCase>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TestCase {
    classname: String,
    name: String,
    time: String,
    skipped: Option<Vec<Skipped>>,
    failure: Option<Failure>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Skipped {
    #[serde(rename = "type", default)]
    skip_type: String,
    message: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Failure {
    message: String,
}
