extern crate grpc;
extern crate futures;
extern crate ngx_rust;
extern crate ngx_mixer_transport;

use std::str;
use std::sync::mpsc::{channel};
use std::sync::Mutex;


use ngx_mixer_transport::mixer_grpc::service_grpc::MixerClient;
use ngx_mixer_transport::mixer_grpc::report::ReportRequest;
use ngx_mixer_transport::mixer_grpc::attributes::Attributes;
use ngx_mixer_transport::mixer_grpc::service_grpc::Mixer;


use protobuf::RepeatedField;
use ngx_rust::bindings::ngx_http_request_s;

use ngx_mixer_transport::attribute::attr_wrapper::AttributeWrapper;
use ngx_mixer_transport::attribute::global_dict::GlobalDictionary;
use ngx_mixer_transport::attribute::message_dict::MessageDictionary;

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
pub fn mixer_report_background()  {

    let rx = CHANNELS.rx.lock().unwrap();

    loop {
        ngx_log!("mixer report  thread waiting");
        let info = rx.recv().unwrap();
        ngx_log!("mixer report thread woke up");

        let client = MixerClient::new_plain( &info.server_name, info.server_port , Default::default()).expect("init");

        let mut req = ReportRequest::new();
        let mut rf = RepeatedField::default();
        rf.push(info.attributes);
        req.set_attributes(rf);

        let resp = client.report(grpc::RequestOptions::new(), req);

        let result = resp.wait();

        ngx_log!("mixer report thread: finished sending to mixer, {:?}",result);
    }
}


// send to background thread using channels
#[allow(unused_must_use)]
fn send_dispatcher(main_config: &ngx_http_mixer_main_conf_t, attr: Attributes)  {

    let server_name = main_config.mixer_server.to_str();
    let server_port = main_config.mixer_port as u16;

    let tx = CHANNELS.tx.lock().unwrap().clone();
    let info = MixerInfo { server_name: String::from(server_name), server_port: server_port, attributes: attr};
    tx.send(info.clone());


   // log(&format!("server: {}, port {}",server_name, server_port));

    ngx_log!("send attribute to mixer report background task");

}



#[no_mangle]
pub extern fn nginmesh_mixer_report_handler(request: &ngx_http_request_s,main_config: &ngx_http_mixer_main_conf_t,
    srv_conf: &ngx_http_mixer_srv_conf_t)  {


    let mut attr = AttributeWrapper::new();

    srv_conf.process_istio_attr(&mut attr);

    request.process_istio_attr(&mut attr);

    let headers_out =  &request.headers_out;
    headers_out.process_istio_attr(&mut attr);


    let mut message_dict = MessageDictionary::new(GlobalDictionary::new());
    send_dispatcher(main_config, attr.as_attributes(&mut message_dict));

}



