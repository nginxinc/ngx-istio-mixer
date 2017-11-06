
macro_rules! ngx_log  {

    ($($arg:tt)*) => {
        ngx_rust::nginx_http::log(&format!($($arg)*))
    }
}