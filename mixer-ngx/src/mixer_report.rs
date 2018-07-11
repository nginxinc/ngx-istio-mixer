// use std::str;
use std::sync::mpsc::{channel};
use std::sync::Mutex;

use grpc::RequestOptions;
use ngx_mixer_transport::mixer_grpc::service_grpc::MixerClient;
use ngx_mixer_transport::mixer_grpc::report::ReportRequest;
use ngx_mixer_transport::mixer_grpc::attributes::CompressedAttributes;
use ngx_mixer_transport::mixer_grpc::service_grpc::Mixer;


use protobuf::RepeatedField;
use ngx_rust::bindings:: { ngx_array_t };
use ngx_rust::bindings::ngx_http_request_s;
use ngx_rust::bindings::ngx_http_upstream_state_t;

use ngx_mixer_transport::attribute::attr_wrapper::AttributeWrapper;
use ngx_mixer_transport::attribute::global_dict::GlobalDictionary;
use ngx_mixer_transport::attribute::message_dict::MessageDictionary;
use ngx_mixer_transport::attribute::global_dict::{ RESPONSE_DURATION, CONTEXT_PROTOCOL };


use super::message::Channels;
use super::message::MixerInfo;

use ngx::main_config::ngx_http_mixer_main_conf_t;
use ngx::server_config::ngx_http_mixer_srv_conf_t;
use ngx::config::MixerConfig;


// initialize channel that can be shared
lazy_static! {
    static ref CHANNELS: Channels<MixerInfo> = {
        let (tx, rx) = channel();

        Channels {
            tx: Mutex::new(tx),
            rx: Mutex::new(rx),
        }
    };
}

// background activy for report.
// receives report attributes and send out to mixer
// Spawned by nginx module init function, spawns new
// thread to listen for mixer attributes
pub fn mixer_report_background()  {

    let rx = CHANNELS.rx.lock().unwrap();

    loop {
        ngx_event_debug!("mixer report thread waiting");
        // Receive attributes from background thread
        let info = rx.recv().unwrap();
        ngx_event_debug!("mixer report thread woke up");

        ngx_event_debug!("New Mixer Request, server: {} port: {}", info.server_name, info.server_port);
        let client = MixerClient::new_plain( &info.server_name, info.server_port , Default::default()).expect("init");

        let mut req = ReportRequest::new();
        let mut rf = RepeatedField::default();
        rf.push(info.attributes);
        req.set_attributes(rf);

        let resp = client.report(RequestOptions::new(), req);

        let result = resp.wait();

        ngx_event_debug!("mixer report thread: finished sending to mixer, {:?}",result);
    }
}


// send to background thread using channels
#[allow(unused_must_use)]
fn send_dispatcher(request: &ngx_http_request_s,main_config: &ngx_http_mixer_main_conf_t, attr: CompressedAttributes)  {

    // Get mixer server and port from main config (Defined in c module)
    let server_name = main_config.mixer_server.to_str();
    let server_port = main_config.mixer_port as u16;


    let tx = CHANNELS.tx.lock().unwrap().clone();
    ngx_event_debug!("Attributes to be sent: {:?}", attr);
    let info = MixerInfo { server_name: String::from(server_name), server_port: server_port, attributes: attr};


    // Send information to background thread
    tx.send(info.clone());

    // NGINX debug logging
    ngx_http_debug!(request,"send attribute to mixer report background task");

}


// Total Upstream response Time Calculation Function Start

fn upstream_response_time_calculation( upstream_states: *const ngx_array_t ) -> i64 {

    unsafe {
        // Get request information array
        let upstream_value = *upstream_states;
        // Hold array of elements
        let upstream_response_time_list = upstream_value.elts;
        // Number of elements
        let upstream_response_time_n = upstream_value.nelts as isize;
        // Size of a single element. Used to index array
        let upstream_response_time_size = upstream_value.size as isize;
        // Variable going to be used to hold the extracted total response time from array
        let mut upstream_response_time_total:i64 = 0;

        // For loop to iterate through number of elements in upstream_response and count total response time
        for i in 0..upstream_response_time_n as isize {

            // Response time pointer gets index i * size of each element.
            let upstream_response_time_ptr = upstream_response_time_list.offset(i*upstream_response_time_size) as *mut ngx_http_upstream_state_t;
            // Get response time of upstream
            let upstream_response_time_value = (*upstream_response_time_ptr).response_time as i64;
            // Tally total response time
            upstream_response_time_total = upstream_response_time_total + upstream_response_time_value;

        }

        return upstream_response_time_total;
    }
}


#[no_mangle]
pub extern fn nginmesh_mixer_report_handler(request: &ngx_http_request_s,main_config: &ngx_http_mixer_main_conf_t,
    srv_conf: &ngx_http_mixer_srv_conf_t)  {

    // Calls debug logging through NGINX - macro set up in ngx-rust/log.rs
    ngx_http_debug!(request,"invoking nginx report");

    // Create new HashMap - mixer-transport/attribute/attr_wrapper.rs
    let mut attr = AttributeWrapper::new();

    // Ngx_http_mixer_srv_conf_t made compatible through struct definition in ./ngx/server_config.rs
    srv_conf.process_istio_attr(&mut attr);

    // Send request attribute to mixer
    request.process_istio_attr(&mut attr);

    // Get total response time of all upstreams
    attr.insert_int64_attribute(RESPONSE_DURATION, upstream_response_time_calculation(request.upstream_states));

    // Access response headers
    let headers_out =  &request.headers_out;
    // Process output headers
    headers_out.process_istio_attr(&mut attr);

    // Create new message dictionary
    let mut message_dict = MessageDictionary::new(GlobalDictionary::new());

    ngx_http_debug!(request,"setting mixer attributes");

    attr.insert_string_attribute( CONTEXT_PROTOCOL, "http");

    // Send attributes from config to background thread
    send_dispatcher(request,main_config, attr.as_attributes(&mut message_dict));

}



