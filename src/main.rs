use std::fs;

use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;

fn main() {
    let raw_results = fs::read_to_string("C:\\Users\\dwall\\repos\\minerva\\example.xml")
        .expect("The file could not be found.");
    let results = raw_results.replace("<>", "").replace("&", "");
    let ts: TestSuites = from_str(&results.to_string()).unwrap();
    println!("{}", ts.test_suites[0].test_cases[0].classname);
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
    skipped: Option<Skipped>,
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
