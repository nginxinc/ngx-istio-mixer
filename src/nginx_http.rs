
extern crate libc;


use std::str;
use std::slice;
use std::ffi::CString;

use bindings::ngx_http_request_s;
use bindings::ngx_list_part_t;
use bindings::ngx_table_elt_t;
use bindings::ngx_uint_t;
use bindings::ngx_str_t;
use bindings::ngx_log_error_core;
use bindings::NGX_LOG_ERR;
use bindings::ngx_cycle;



impl ngx_str_t  {
    // convert nginx string to str slice
    fn to_str(&self) -> &str  {

        unsafe {
            let slice = slice::from_raw_parts(self.data,self.len) ;
            return str::from_utf8(slice).unwrap();
        }            
   
    }
}


// extract request.header from nginx request
// 
pub fn extract_request_header_from_nginx(request: *const ngx_http_request_s)  -> String {

    let mut out_header = String::from("");

      // extract 
    unsafe  {
        let mut part: *const ngx_list_part_t  = &(*request).headers_in.headers.part ;
        let mut h: *const ngx_table_elt_t =  (*part).elts as *const ngx_table_elt_t;

        let mut i: ngx_uint_t = 0;
        let mut done = false;

        while !done  {
            if i >= (*part).nelts  {
                if (*part).next.is_null() {
                    done = true;
                    break;
                }

                part = (*part).next;
                h = (*part).elts as *mut ngx_table_elt_t;
                i = 0;
            }

            let header: *const ngx_table_elt_t = h.offset(i as isize);

            let header_name: ngx_str_t = (*header).key;   
            ngx_log_error_core(NGX_LOG_ERR as usize, (*ngx_cycle).log, 0, CString::new("request header: %*s").unwrap().as_ptr(),
                header_name.len,header_name.data);         
            out_header.push_str(header_name.to_str());
           
           
            out_header.push_str(":");

            let header_value: ngx_str_t = (*header).value;
            ngx_log_error_core(NGX_LOG_ERR as usize, (*ngx_cycle).log, 0, CString::new("request value: %*s").unwrap().as_ptr(),
                header_value.len,header_value.data);  

            out_header.push_str(header_value.to_str());
  
                  
            i = i + 1;

        }
    }

    return out_header;

}
