extern crate grpc;
extern crate futures;



use std::env;
use std::mem;
use std::ptr;
use std::str;
use std::slice;
use std::collections::HashMap;
use service_grpc::MixerClient;
use report::ReportRequest;
use attributes::Attributes;
use attributes::StringMap;
use service_grpc::Mixer;

use protobuf::well_known_types::Timestamp;
use bindings::ngx_http_request_s;
use bindings::ngx_http_headers_in_t;
use bindings::ngx_http_headers_out_t;
use nginx_http::list_iterator;
use nginx_http::log;
use bindings::ngx_str_t;


static REQUEST_HEADER: i32 = 0;

static SOURCE_IP: i32 = 2;
static SOURCE_PORT: i32 = 3;
static SOURCE_NAME: i32 = 4;
static SOURCE_UID: i32 = 5;
static SOURCE_NAMESPACE: i32 = 6;
static SOURCE_LABLES: i32 = 7;
static SOURCE_USER: i32 = 8;
static TARGET_IP: i32 = 9;
static TARGET_PORT: i32 = 10;
static TARGET_SERVICE: i32 = 11;
static TARGET_NAME: i32 = 12;
static TARGET_UID: i32 = 13;
static TARGET_NAMESPACE: i32 = 14;
static TARGET_LABELS: i32 = 15;
static TARGET_URSER: i32 = 16;

static REQUEST_PATH: i32 = 17;
static REQUEST_HOST: i32 = 18;
static REQUEST_METHOD: i32 = 19;
static REQUEST_REASON: i32 = 20;
static REQUEST_REFER: i32 = 21;
static REQUEST_SCHEME: i32 = 22;
static REQUEST_SIZE: i32 = 23;
static REQUEST_TIME: i32 = 24;
static REQUEST_USERAGENT: i32 = 25;
static RESPONSE_CODE: i32 = 29;
static RESPONSE_DURATION: i32 = 28;
static RESPONSE_SIZE: i32 = 30;
static RESPONSE_HEADERS: i32 = 31;
static RESPONSE_TIME: i32 = 32;



/**
  filter nginx
 */
/*
#[no_mangle]
public extern fn ngx_int_t ngx_http_istio_mixer_filter(request: *const ngx_http_request_s) -> ngx_int_t {
{

    log(&format!("start invoking istio mixer filter");

    ngx_http_mixer_main_conf_t *conf = *rngx_http_istio_mixer_module);

    ngx_log_error(NGX_LOG_ERR, ngx_cycle->log, 0, "using server: %*s",conf->mixer_server.len,conf->mixer_server.data);

    // invoke mix client
    mixer_client(r);

    ngx_log_error(NGX_LOG_ERR, ngx_cycle->log, 0, "finish calling istio filter");

    return ngx_http_next_header_filter(r);

} */




#[no_mangle] 
pub extern fn mixer_client(request: & ngx_http_request_s,ng_server: & ngx_str_t,port: u32) -> *const u8 {

    let server_name = ng_server.to_str()  ;

    log(&format!("server port {}",port));

 
    let client = MixerClient::new_plain(server_name, port as u16, Default::default()).expect("init");

    let mut requests = Vec::new();
    let mut req = ReportRequest::new();
    let mut attr = Attributes::new();
    //attr.set_string_attributes("")
    req.set_request_index(0);


    // set up attribute dictionary
    let mut dictValues: HashMap<i32,String> = HashMap::new();
    dictValues.insert(REQUEST_HEADER,String::from("request.headers"));
    dictValues.insert(TARGET_SERVICE,String::from("target.service"));
    dictValues.insert(REQUEST_HOST,String::from("request.host"));
    dictValues.insert( REQUEST_METHOD, String::from("request.method"));
    dictValues.insert( REQUEST_PATH, String::from("request.path"));
    dictValues.insert( REQUEST_REFER, String::from("request.referer"));
    dictValues.insert( REQUEST_SCHEME, String::from("request.scheme"));
    dictValues.insert( REQUEST_SIZE, String::from("request.size"));
    dictValues.insert( REQUEST_TIME, String::from("request.time"));
    dictValues.insert( REQUEST_USERAGENT, String::from("request.useragent"));
    dictValues.insert( RESPONSE_CODE, String::from("response.code"));
    dictValues.insert( RESPONSE_DURATION, String::from("response.duration"));
    dictValues.insert(RESPONSE_SIZE,String::from("response.size"));
    dictValues.insert(RESPONSE_HEADERS,String::from("response.headers"));
    dictValues.insert(RESPONSE_SIZE,String::from("response.size"));


    // populate
    attr.set_dictionary(dictValues);

    process_request_attribute(request, &mut attr);
    process_response_attribute(request, &mut attr);

    req.set_attribute_update(attr);

    requests.push(req);


    let resp = client.report(grpc::RequestOptions::new(), grpc::StreamingRequest::iter(requests));

    resp.wait_drop_metadata().count();

    "Hello, world!\0".as_ptr()
}





// find string index from dictionary
fn string_index(value: &str, dictValues: &HashMap<i32,String>) -> Option<i32> {

    log(&format!("checking if value: {} exists in dictionary",value));
    for( index ,dictValue) in dictValues  {
      //  log(&format!("comparing existing dictionary: {}",&dictValue));
        if value == dictValue {
       //     log(&format!("matched existing value"));
            return Some(index.abs());
        }
    }
   // log(&format!("did not match"));
    return None;
}



// process request attribute,
// loop over request header and add to dictionary
// then use that to build new string map
fn process_request_attribute(request: & ngx_http_request_s, attr: &mut Attributes, )  {

    let headers_in = request.headers_in;

    // fill in the string values
    let host = headers_in.host_str();
    log(&format!("request host {}",host));
    attr.mut_string_attributes().insert(REQUEST_HOST,String::from(host));

    let method = request.method_name.to_str();
    log(&format!("request method {}",method));
    attr.mut_string_attributes().insert(REQUEST_METHOD,String::from(method));

    let path = request.uri.to_str();
    log(&format!("request path {}",path));
    attr.mut_string_attributes().insert(REQUEST_PATH,String::from(path));

    let referer = headers_in.referer_str();
    if let Some(refererStr) = referer {
        log(&format!("request referrer {}",refererStr));
        attr.mut_string_attributes().insert(REQUEST_REFER,String::from(refererStr));
    }

    let scheme = request.http_protocol.to_str();
    log(&format!("request scheme {}",scheme));
    attr.mut_string_attributes().insert(REQUEST_SCHEME,String::from("http")); // hard code now

    let request_size = request.request_length;
    log(&format!("request size {}",request_size));
    attr.mut_int64_attributes().insert(REQUEST_SIZE,request_size);

    let mut request_time = Timestamp::new();
    request_time.set_seconds(request.start_sec);
    request_time.set_nanos(request.start_msec as i32);
    attr.mut_timestamp_attributes().insert( REQUEST_TIME, request_time);

    let user_agent = headers_in.user_agent_str();
    log(&format!("request user agent {}",user_agent));
    attr.mut_string_attributes().insert(REQUEST_USERAGENT,String::from(user_agent));



    // fill in the string value
    let mut map: HashMap<i32,String> = HashMap::new();
    {
        // create new scope so we can borrow dictionary
        // otherwise we can't borrow 2 things
        let dictValues = attr.mut_dictionary();
        for (name,value) in headers_in.headers_iterator()   {
            log(&format!("header name: {}, value: {}",&name,&value));

            let result = string_index(&value,dictValues);

            match result  {
                Some(index) =>  {
                    map.insert(index,value);
                    log(&format!("match existing index: {}",index));
                },
                None =>  {
                    let newIndex = dictValues.len() as i32  + 1;
                    dictValues.insert(newIndex,value.clone());
                    map.insert(newIndex,value.clone());
                    log(&format!("adding to string map index: {}, value: {}",newIndex,&value));
                },

            }

        }
    }


    let mut requestValueMap = StringMap::new();
    requestValueMap.set_map(map);
    attr.mut_stringMap_attributes().insert(REQUEST_HEADER,requestValueMap);

}


static RESPONSE_HEADER_STATUS: i32 = 100;
static RESPONSE_HEADER_CONTENT_LENGTH: i32 = 101;

/**
 * proces response headers
 */
fn process_response_attribute(request: &ngx_http_request_s, attr: &mut Attributes, )  {


    let headers_out =  request.headers_out;


    let response_code = headers_out.status;
    log(&format!("response code {}",response_code));
    attr.mut_int64_attributes().insert(RESPONSE_CODE,response_code as i64);

    let content_length = headers_out.content_length_n;
    log(&format!("content length {}",content_length));
    attr.mut_int64_attributes().insert(RESPONSE_SIZE,content_length);

    let duration = headers_out.date_time - request.start_sec;
    log(&format!("response duration {}",duration));
    attr.mut_int64_attributes().insert(RESPONSE_DURATION,5000);

    // fill in the string value
    let mut map: HashMap<i32,String> = HashMap::new();
    {
        // create new scope so we can borrow dictionary
        // otherwise we can't borrow 2 things
        let dictValues = attr.mut_dictionary();
        for (name,value) in headers_out.headers_iterator()   {
            log(&format!("out header name: {}, value: {}",&name,&value));

            let result = string_index(&value,dictValues);

            match result  {
                Some(index) =>  {
                    map.insert(index,value);
                    log(&format!("match existing index: {}",index));
                },
                None =>  {
                    let newIndex = dictValues.len() as i32  + 1;
                    dictValues.insert(newIndex,value.clone());
                    map.insert(newIndex,value.clone());
                    log(&format!("adding to string map index: {}, value: {}",newIndex,&value));
                },

            }

        }
    }


    let mut requestValueMap = StringMap::new();
    requestValueMap.set_map(map);
    attr.mut_stringMap_attributes().insert(RESPONSE_HEADERS,requestValueMap);



}
