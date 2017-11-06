extern crate reqwest;

use util::make;
use std::io::Read;


// call check, this should succeed since this doesn't have any
#[test]
fn nginx_check_basic_test()  {

  //  let _result = make("test-nginx-only");

    let mut response = reqwest::get("http://localhost:8000/check").unwrap();
    assert!(response.status().is_success(),"nginx test check succedd");

    let mut content = String::new();
    response.read_to_string(&mut content);

    println!("response: {}",content);
    assert_eq!(content,"9100","should return local services");
}


// force deny

#[test]
fn nginx_check_deny_test()  {

    let _result = make("test-nginx-only");

    let mut response = reqwest::get("http://localhost:8000/check").unwrap();
    assert!(response.status().is_success(),"nginx test check succedd");

    let mut content = String::new();
    response.read_to_string(&mut content);

    println!("response: {}",content);
    assert_eq!(content,"9100","should return local services");
}
