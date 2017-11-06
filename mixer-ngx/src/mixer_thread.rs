extern crate ngx_rust;

use std::{thread };


use ngx_rust::bindings::ngx_int_t;
use ngx_rust::bindings::NGX_OK;
use ngx_rust::nginx_http::log;

use mixer_report::mixer_report_background;
//use ngx::mixer_check::mixer_check_background;





// start background activities
#[no_mangle]
pub extern fn nginmesh_mixer_init() -> ngx_int_t {

    log(&format!("init mixer start "));
    thread::spawn(|| {
        log(&format!("starting mixer report background task"));
        mixer_report_background();
    });

    
    log(&format!("init mixer end "));
    return NGX_OK as ngx_int_t;
}

#[no_mangle]
pub extern fn nginmesh_mixer_exit() {
    log(&format!("mixer exit "));
}

