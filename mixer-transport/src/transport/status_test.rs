
use super::status::{ Status, StatusCodeEnum };


#[test]
fn test_code_ok() {

    let status = Status::new();
     assert_eq!(status.get_error_code(), StatusCodeEnum::OK);
}

#[test]
fn test_code_ok_invalid_arg() {

    let status = Status::with_code(StatusCodeEnum::INVALID_ARGUMENT);
     assert_eq!(status.get_error_code(), StatusCodeEnum::INVALID_ARGUMENT);
}