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

        let dest_ip = self.destination_ip.to_str();
        let source_ip = self.source_ip.to_str();
        if dest_ip.len() != 0 {
            attr.insert_ip_attribute( DESTINATION_IP, dest_ip.parse::<Ipv4Addr>().unwrap());
        }
        if source_ip.len() != 0 {
            attr.insert_ip_attribute( SOURCE_IP, source_ip.parse::<Ipv4Addr>().unwrap());
        }

        attr.insert_string_attribute( DESTINATION_SERVICE, self.destination_service.to_str());
        attr.insert_string_attribute( DESTINATION_UID, self.destination_uid.to_str());
        attr.insert_string_map( DESTINATION_LABELS, self.destination_labels.to_map());
        attr.insert_string_attribute( SOURCE_UID,self.source_uid.to_str());
        attr.insert_string_attribute( SOURCE_SERVICE,self.source_service.to_str());
        attr.insert_int64_attribute( SOURCE_PORT,self.source_port as i64);
        attr.insert_string_map( SOURCE_LABELS, self.source_labels.to_map());
    }
}

impl service_labels {

    fn to_map(&self) -> HashMap<String,String> {

        let mut out_map: HashMap<String,String> = HashMap::new();

        if (self.key as *const u8).is_null() {
            return out_map;
        } else {
            for (key,value) in self.iter() {
                out_map.insert(key, value);
            }
        }

        return out_map;
    }

    pub fn iter(&self) -> LabelsIterator {

        unsafe {
            LabelsIterator {
                key: (*self.key).elts as *const ngx_str_t,
                value: (*self.value).elts as *const ngx_str_t,
                nelts: (*self.key).nelts,
                i: 0
            }
        }

    }
}

pub struct LabelsIterator {
    key: *const ngx_str_t,
    value: *const ngx_str_t,
    nelts: ngx_uint_t,
    i: isize
}

impl Iterator for LabelsIterator {

    type Item = (String,String);

    fn next(&mut self) -> Option<Self::Item> {

        unsafe {
            if (self.key as *const u8).is_null()
                    || self.i >= self.nelts as isize {
                return None;
            } else {
                self.i = self.i + 1;

                return Some ( ((*self.key.offset(self.i - 1)).to_string(), (*self.value.offset(self.i - 1)).to_string()) );
            }

        }
    }

}


