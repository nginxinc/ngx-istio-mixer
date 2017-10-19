
use mixer::status::Status;
use std::mem;

// https://github.com/google/protobuf/blob/master/src/google/protobuf/stubs/status.h

#[derive(Debug, PartialEq)]
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

