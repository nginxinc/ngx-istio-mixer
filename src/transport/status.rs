
use std::mem;

// https://github.com/google/protobuf/blob/master/src/google/protobuf/stubs/status.h

#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum StatusCodeEnum {

    OK =  0,
    CANCELLED =   1,
    UNKNOWN = 2,
    INVALID_ARGUMENT = 3,
    DEADLINE_EXCEEDED = 4,
    NOT_FOUND = 5,
    ALREADY_EXISTS = 6,
    PERMISSION_DENIED = 7,
    UNAUTHENTICATED = 16,
    RESOURCE_EXHAUSTED = 8,
    FAILED_PRECONDITION = 9,
    ABORTED = 10,
    OUT_OF_RANGE = 11,
    UNIMPLEMENTED = 12,
    INTERNAL = 13,
    UNAVAILABLE = 14,
    DATA_LOSS = 15,
}


// convert integer status to enum status
pub fn from_int(code: i32) -> StatusCodeEnum  {

    //return code as StatusCodeEnum;
    return unsafe { 
        mem::transmute(code as u8) 
    };
}

#[derive(Clone)]
pub struct Status {

    error_code:  StatusCodeEnum,
    error_message: Option<String>
}

impl Status  {

    pub fn new() -> Status  {
        Status  {
            error_code: StatusCodeEnum::OK,
            error_message: None
        }
    }


    pub fn with_code( code: StatusCodeEnum) -> Status {
        Status {
            error_code: code,
            error_message: None
        }
    }

    pub fn with(error_code: StatusCodeEnum, error_message: String) -> Status {
        Status {
            error_code,
            error_message: Some(error_message)
        }
    }

    pub fn ok(&self) -> bool  {
        self.error_code == StatusCodeEnum::OK
    }

    pub fn get_error_code(&self) -> StatusCodeEnum  {
        self.error_code
    }
}

