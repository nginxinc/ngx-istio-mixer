extern crate ngx_rust;

use std::collections::HashMap;
use ngx_rust::bindings:: { ngx_uint_t, ngx_str_t } ;

use ngx_mixer_transport::attribute::attr_wrapper::AttributeWrapper;
use ngx_mixer_transport::attribute::global_dict::{ SOURCE_IP, SOURCE_UID, SOURCE_SERVICE, SOURCE_PORT,
                    SOURCE_LABELS, DESTINATION_SERVICE, DESTINATION_IP, DESTINATION_UID, DESTINATION_LABELS
};

use super::config::MixerConfig;
use std::net::Ipv4Addr;

#[repr(C)]
pub struct service_labels {
    pub app:                 ngx_str_t,
    pub pod_template_hash:   ngx_str_t,
    pub version:             ngx_str_t

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



        let mut out_destination_labels: HashMap<String,String> = HashMap::new();

        out_destination_labels.insert(String::from("app"), self.destination_labels.app.to_string());
        out_destination_labels.insert(String::from("pod-template-hash"), self.destination_labels.pod_template_hash.to_string());
        out_destination_labels.insert(String::from("version"), self.destination_labels.version.to_string());

        // let mut out_destination_labels = labels_to_map(self.destination_labels);
        ngx_event_debug!("{}: {:?}", DESTINATION_LABELS, out_destination_labels);
        attr.insert_string_map( DESTINATION_LABELS, out_destination_labels);

        ngx_event_debug!("{}: {:?}", SOURCE_IP, self.source_ip.to_str().parse::<Ipv4Addr>().unwrap());
        attr.insert_ip_attribute( SOURCE_IP,self.source_ip.to_str().parse::<Ipv4Addr>().unwrap());

        ngx_event_debug!("{}: {}", SOURCE_UID, self.source_uid.to_str());
        attr.insert_string_attribute( SOURCE_UID,self.source_uid.to_str());
        ngx_event_debug!("{}: {}", SOURCE_SERVICE, self.source_service.to_str());
        attr.insert_string_attribute( SOURCE_SERVICE,self.source_service.to_str());
        ngx_event_debug!("{}: {}", SOURCE_PORT, self.source_port as i64);
        attr.insert_int64_attribute( SOURCE_PORT,self.source_port as i64);

        let mut out_source_labels: HashMap<String,String> = HashMap::new();

        out_source_labels.insert(String::from("app"), self.source_labels.app.to_string());
        out_source_labels.insert(String::from("pod-template-hash"), self.source_labels.pod_template_hash.to_string());
        out_source_labels.insert(String::from("version"), self.source_labels.version.to_string());
        ngx_event_debug!("Destination Labels: {:?}", out_source_labels);
        attr.insert_string_map( SOURCE_LABELS, out_source_labels);

    }
}

fn labels_to_map(labels: service_labels) -> HashMap<String,String> {

        let mut out_map: HashMap<String,String> = HashMap::new();

        out_map.insert(String::from("app"), labels.app.to_string());
        out_map.insert(String::from("pod-template-hash"), labels.pod_template_hash.to_string());
        out_map.insert(String::from("version"), labels.version.to_string());

        return out_map;
    }


