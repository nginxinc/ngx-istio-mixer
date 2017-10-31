
extern crate ngx_rust;
extern crate ngx_mixer_transport;

use std::collections::HashMap;


use protobuf::well_known_types::Timestamp;
use ngx_rust::bindings::ngx_http_request_s;
use ngx_rust::nginx_http::log;

use ngx_mixer_transport::attribute::attr_wrapper::AttributeWrapper;


use ngx_mixer_transport::attribute::global_dict::REQUEST_HEADER;
use ngx_mixer_transport::attribute::global_dict::REQUEST_HOST;
use ngx_mixer_transport::attribute::global_dict::REQUEST_METHOD;
use ngx_mixer_transport::attribute::global_dict::REQUEST_PATH;
use ngx_mixer_transport::attribute::global_dict::REQUEST_REFER;
use ngx_mixer_transport::attribute::global_dict::REQUEST_SCHEME;
use ngx_mixer_transport::attribute::global_dict::REQUEST_SIZE;
use ngx_mixer_transport::attribute::global_dict::REQUEST_TIME;
use ngx_mixer_transport::attribute::global_dict::REQUEST_USERAGENT;
use ngx_mixer_transport::attribute::global_dict::SOURCE_IP;
use ngx_mixer_transport::attribute::global_dict::SOURCE_UID;
use ngx_mixer_transport::attribute::global_dict::SRC_IP_HEADER;
use ngx_mixer_transport::attribute::global_dict::SRC_UID_HEADER;





// process request attribute,
// loop over request header and add to dictionary
// then use that to build new string map
pub fn process_request_attribute(request: & ngx_http_request_s, attr: &mut AttributeWrapper )  {

    let headers_in = request.headers_in;


    attr.insert_string_attribute(REQUEST_HOST,  headers_in.host_str());
    attr.insert_string_attribute(REQUEST_METHOD, request.method_name.to_str());
    attr.insert_string_attribute(REQUEST_PATH, request.uri.to_str());

    let referer = headers_in.referer_str();
    if let Some(ref_str) = referer {
        attr.insert_string_attribute(REQUEST_REFER, ref_str);
    }

    //let scheme = request.http_protocol.to_str();
    attr.insert_string_attribute(REQUEST_SCHEME, "http"); // hard code now


    attr.insert_int64_attribute(REQUEST_SIZE, request.request_length);

    let mut request_time = Timestamp::new();
    request_time.set_seconds(request.start_sec);
    request_time.set_nanos(request.start_msec as i32);
    attr.insert_time_stamp_attribute(REQUEST_TIME, request_time);

    attr.insert_string_attribute(REQUEST_USERAGENT, headers_in.user_agent_str());


   // fill in the string value
    let mut map: HashMap<String,String> = HashMap::new();
    {
        for (name,value) in headers_in.headers_iterator()   {
            log(&format!("in header name: {}, value: {}",&name,&value));

            // TODO: remove header
            match name.as_ref()  {

                SRC_IP_HEADER  => {
                    log(&format!("source IP received {}",&value));
                    attr.insert_string_attribute( SOURCE_IP,&value);
                },

                SRC_UID_HEADER => {
                    log(&format!("source UID received {}",&value));
                    attr.insert_string_attribute( SOURCE_UID,&value);
                },
                _ => {
                    log(&format!("other source header {}",&name));
                    map.insert(name,value);
                }
            }


        }
    }

    attr.insert_string_map(REQUEST_HEADER, map);

}