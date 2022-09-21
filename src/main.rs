use serde::{Deserialize, Serialize};
use serde_xml_rs::{from_str, to_string};

fn main() {
    let src = r#"<?xml version="1.0" encoding="UTF-8"?>
    <testsuites disabled="0" 
                errors="100"   
                failures="200" 
                name="MyTests"
                tests=""    
                time="20.22"
            >
      <testsuite name="Things"      
             tests="0"     
             disabled="0"  
                 errors="0"    
                 failures="0" 
                 hostname="0"  
             id="0"       
             package="0"   
             skipped="0"   
             time="0"      
             timestamp="0"
             >
    
        <properties>
          <property name="" value=""/>
        </properties>
    
        <testcase name=""   
              assertions="" 
              classname=""  
              status=""    
              time=""      
              >
    
    
          <skipped message=""   
          />
    
          <error message="" 
             type="" 
             >error description</error>
    
          <failure message=""
               type=""    
               >failure description</failure>
    
          <system-out>STDOUT text</system-out>
    
          <system-err>STDERR text</system-err>
        </testcase>
    
        <system-out>STDOUT text</system-out>
        <system-err>STDERR text</system-err>
      </testsuite>
    </testsuites>"#;
    let ts: TestSuites = from_str(src).unwrap();
    println!("{}", ts.test_suites[0].errors);
    
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TestSuites {
    disabled: String,
    errors: u32,
    failures: u32,
    name: String,
    tests: String,
    time: String,
    #[serde(rename = "$value")]
    test_suites: Vec<TestSuite>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TestSuite {
    name: String,
    tests: u32,
    disabled: u32,
    errors: u32,
    failures: u32,
    hostname: String,
    id: String,
    package: String,
    skipped: u32,
    time: String,
    timestamp: String
}

