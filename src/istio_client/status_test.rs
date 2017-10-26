
use mixer::status::Status;
use super::status;
use super::status::StatusCodeEnum;

#[test]
fn test_code_ok() {

    let status = Status::new();
     assert_eq!(status::from_int(status.code), StatusCodeEnum::OK);
}

#[test]
fn test_code_ok_invalid_arg() {

    let mut status = Status::new();
    status.set_code(3);
     assert_eq!(status::from_int(status.code), StatusCodeEnum::INVALID_ARGUMENT);
}