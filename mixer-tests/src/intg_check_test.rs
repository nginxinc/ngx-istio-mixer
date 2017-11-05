extern crate reqwest;

use util::make;
use std::io::Read;


#[test]
fn nginx_check_test()  {

    let _result = make("test-nginx-only");

    let mut response = reqwest::get("http://localhost:8000/report").unwrap();
    assert!(response.status().is_success(),"nginx test check succedd");

    let mut content = String::new();
    response.read_to_string(&mut content);

    println!("response: {}",content);
    assert_eq!(content,"9100","should return local services");
}