
use ngx_rust::bindings::ngx_int_t;
use ngx_rust::bindings::ngx_str_t;




#[repr(C)]
pub struct ngx_http_mixer_main_conf_t {
 
    pub mixer_server: ngx_str_t,
    pub mixer_port: ngx_int_t,
    pub target_ip: ngx_str_t,
    pub target_uid: ngx_str_t,
    pub target_service: ngx_str_t

}

/*
#[repr(C)]
pub struct ngx_http_mixer_loc_conf_t {
    pub enable_report: ngx_flag_t,              // for every location, we need flag to enable/disable mixer
    pub enable_check: ngx_flag_t
}
*/

