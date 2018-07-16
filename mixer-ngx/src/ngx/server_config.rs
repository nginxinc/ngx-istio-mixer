extern crate ngx_rust;

use std::collections::HashMap;
use ngx_rust::bindings:: { ngx_uint_t, ngx_str_t, ngx_array_t } ;

use ngx_mixer_transport::attribute::attr_wrapper::AttributeWrapper;
use ngx_mixer_transport::attribute::global_dict::{ SOURCE_IP, SOURCE_UID, SOURCE_SERVICE, SOURCE_PORT,
                    SOURCE_LABELS, DESTINATION_SERVICE, DESTINATION_IP, DESTINATION_UID, DESTINATION_LABELS
};

use super::config::MixerConfig;
use std::net::Ipv4Addr;

#[repr(C)]
pub struct service_labels {
    pub key:                 *const ngx_array_t,
    pub value:               *const ngx_array_t

}

// ngx_http_mixer_srv_conf_t struct made compatible with data type in c module using #[repr(C)]
#[repr(C)]
pub struct ngx_http_mixer_srv_conf_t {

    pub destination_service:    ngx_str_t,
    pub destination_uid:        ngx_str_t,
    pub destination_ip:         ngx_str_t,
    pub destination_labels:     service_labels,
    pub source_ip:              ngx_str_t,
    pub source_uid:             ngx_str_t,
    pub source_service:         ngx_str_t,
    pub source_port:            ngx_uint_t,
    pub source_labels:          service_labels
}

impl MixerConfig for  ngx_http_mixer_srv_conf_t  {

    fn process_istio_attr(&self, attr: &mut AttributeWrapper) {
        ngx_event_debug!("Processing server attributes");

        ngx_event_debug!("{}: {}",DESTINATION_SERVICE, self.destination_service.to_str());
        attr.insert_string_attribute( DESTINATION_SERVICE, self.destination_service.to_str());
        ngx_event_debug!("{}: {}", DESTINATION_UID, self.destination_uid.to_str());
        attr.insert_string_attribute( DESTINATION_UID, self.destination_uid.to_str());
        ngx_event_debug!("{}: {}",DESTINATION_IP, self.destination_ip.to_str());
        attr.insert_string_attribute( DESTINATION_IP, self.destination_ip.to_str());
        ngx_event_debug!("{}: {:?}", DESTINATION_LABELS, labels_to_map(&self.destination_labels));
        attr.insert_string_map( DESTINATION_LABELS, labels_to_map(&self.destination_labels));
        let ip = self.source_ip.to_str();
        if ip.len() != 0 {
            ngx_event_debug!("{}: {:?}", SOURCE_IP, ip);
            attr.insert_ip_attribute( SOURCE_IP, ip.parse::<Ipv4Addr>().unwrap());
        }
        ngx_event_debug!("{}: {}", SOURCE_UID, self.source_uid.to_str());
        attr.insert_string_attribute( SOURCE_UID,self.source_uid.to_str());
        ngx_event_debug!("{}: {}", SOURCE_SERVICE, self.source_service.to_str());
        attr.insert_string_attribute( SOURCE_SERVICE,self.source_service.to_str());
        ngx_event_debug!("{}: {}", SOURCE_PORT, self.source_port as i64);
        attr.insert_int64_attribute( SOURCE_PORT,self.source_port as i64);
        ngx_event_debug!("{}: {:?}", SOURCE_LABELS, labels_to_map(&self.source_labels));
        attr.insert_string_map( SOURCE_LABELS, labels_to_map(&self.source_labels));

    }
}

fn labels_to_map(labels: *const service_labels) -> HashMap<String,String> {

    let mut out_map: HashMap<String,String> = HashMap::new();

    unsafe {
        if ((*labels).key as *const u8).is_null() {
            return out_map;
        }

        let key_arr: *const ngx_str_t = (*(*labels).key).elts as *const ngx_str_t;
        let value_arr: *const ngx_str_t = (*(*labels).value).elts as *const ngx_str_t;

        for i in 0..(*(*labels).key).nelts {

            let key = (*key_arr.offset(i as isize)).to_string();
            let value = (*value_arr.offset(i as isize)).to_string();

            out_map.insert(String::from(key), value.to_string());
        }
    }

    return out_map;
}


