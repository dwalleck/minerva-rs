use std::fs;

use regex::Regex;
use self::models::*;
use diesel::prelude::*;
use minerva::*;
use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;
use urlencoding::encode;

fn main() {
    use self::schema::test_results::dsl::*;

    let connection = &mut establish_connection();

    let raw_results = fs::read_to_string("C:\\Users\\dwall\\repos\\minerva\\nightly_results.xml")
        .expect("The file could not be found.");
    let mut results = raw_results.replace("<>", "").replace("&", "");
    //let re = Regex::new(r"<failure message="(.*)">(.*)</failure>").unwrap();
    let re = Regex::new(r"<failure message=(?P<msg>.*)>(?P<content>.*)</failure>").unwrap();
    // let binding = results.to_string();
    // let results = re.replace_all(
    //     binding.as_str(),
    //     "<failure message=\"\">Some error</failure>",
    // );
    if let Some(captures) = re.captures(results.as_str()) {
        let message = encode(captures.get(1).unwrap().as_str());
        let content = encode(captures.get(2).unwrap().as_str());
        results = re.replace(results.as_str(), format!("<failure message=\"{}\">{}</failure>", message.to_string(), content.to_string()).as_str()).to_string();
    }

    println!("{}", encode("selenium.common.exceptions.TimeoutException: Message: Stacktrace: #0 0x55804c5f9b13 <unknown> #1 0x55804c400688 <unknown> #2 0x55804c437cc7 <unknown> #3 0x55804c437e91 <unknown> #4 0x55804c46ae34 <unknown> #5 0x55804c4558dd <unknown> #6 0x55804c468b94 <unknown> #7 0x55804c4557a3 <unknown> #8 0x55804c42b0ea <unknown> #9 0x55804c42c225 <unknown> #10 0x55804c6412dd <unknown> #11 0x55804c6452c7 <unknown> #12 0x55804c62b22e <unknown> #13 0x55804c6460a8 <unknown> #14 0x55804c61fbc0 <unknown> #15 0x55804c6626c8 <unknown> #16 0x55804c662848 <unknown> #17 0x55804c67cc0d <unknown> #18 0x7f1b224a5609 <unknown>"));
    println!("{}", results);

    let ts: TestSuites = from_str(&results.to_string()).unwrap();
    for suite in ts.test_suites {
        for case in suite.test_cases {
            let new_result = NewTestResult {
                duration: case.time.parse().unwrap(),
                run_at: suite.timestamp.parse().unwrap(),
                name: &case.name,
                status: &case.failure.map_or("Passed".to_string(), |_| "Failed".to_string()),
            };

            diesel::insert_into(test_results)
                .values(&new_result)
                .execute(connection)
                .expect("Error saving new post");
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
