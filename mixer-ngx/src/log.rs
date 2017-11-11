
macro_rules! ngx_log  {

    ($($arg:tt)*) => {
        //ngx_rust::nginx_http::log(&format!($($arg)*))
    }
}


macro_rules! ngx_debug  {

    ($level:expr,$log:expr,$($arg:tt)*) => {
        if (*$log).log_level & $level as usize > 0{
            let c_message = ::std::ffi::CString::new(format!($($arg)*)).unwrap();
            ngx_rust::bindings::ngx_log_error_core(ngx_rust::bindings::NGX_LOG_DEBUG as usize, $log, 0, c_message.as_ptr());
        }
    }
}

macro_rules! ngx_http_debug  {

    ($request:expr,$($arg:tt)*) => {
        unsafe  {
            ngx_debug!(NGX_LOG_DEBUG_HTTP,(*($request).connection).log,$($arg)*);
        }

    }
}

