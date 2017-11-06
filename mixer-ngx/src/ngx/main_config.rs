extern crate ngx_rust;

use ngx_rust::bindings:: { ngx_int_t, ngx_str_t } ;

use ngx_mixer_transport::attribute::attr_wrapper::AttributeWrapper;
use ngx_mixer_transport::attribute::global_dict::{ SOURCE_IP, SOURCE_UID, SOURCE_SERVICE, SOURCE_PORT };

use super::config::MixerConfig;

#[repr(C)]
pub struct ngx_http_mixer_main_conf_t {

    pub mixer_server: ngx_str_t,
    pub mixer_port: ngx_int_t,
    pub source_ip: ngx_str_t,
    pub source_uid: ngx_str_t,
    pub source_service: ngx_str_t,
    pub source_port: ngx_int_t
}


impl MixerConfig for  ngx_http_mixer_main_conf_t  {

    fn process_istio_attr(&self, attr: &mut AttributeWrapper) {


        attr.insert_string_attribute( SOURCE_IP,self.source_ip.to_str());
        attr.insert_string_attribute(SOURCE_UID,self.source_uid.to_str());
        attr.insert_string_attribute(SOURCE_SERVICE,self.source_service.to_str());
        attr.insert_int64_attribute(SOURCE_PORT,self.source_port as i64);

    }


}



