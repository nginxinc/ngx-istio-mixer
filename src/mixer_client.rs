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

use bindings::ngx_http_request_s;
use bindings::ngx_http_headers_in_t;
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
static REQUEST_DURATION: i32 = 28;
static REQUEST_CODE: i32 = 29;
static RESPONSE_SIZE: i32 = 30;
static RESPONSE_HEADERS: i32 = 31;
static RESPONSE_TIME: i32 = 32;
static RESPONSE_DURATIOn: i32 = 33;
static RESPONSE_CODE: i32 = 34;


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

    // define attribute value types
    let mut stringAttributes: HashMap<i32,String> = HashMap::new();
    let mut stringMapAttributes: HashMap<i32,StringMap> = HashMap::new();


    // set up attribute dictionary 
    let mut dictValues: HashMap<i32,String> = HashMap::new();
    dictValues.insert(REQUEST_HEADER,String::from("request.headers"));
    dictValues.insert(TARGET_SERVICE,String::from("target.service"));
    dictValues.insert(REQUEST_HOST,String::from("request.host"));
    dictValues.insert(RESPONSE_SIZE,String::from("response.size"));
    dictValues.insert(RESPONSE_HEADERS,String::from("response.headers"));
    dictValues.insert(RESPONSE_SIZE,String::from("response.size"));



    // process request map
    let requestValueMap = process_request_attribute(
        request.headers_in,
        &mut dictValues,
        &mut stringAttributes,
        &mut stringMapAttributes
    );

    let reponseValueMap = process_response_attribute(&mut dictValues);
    stringMapAttributes.insert(RESPONSE_HEADERS,reponseValueMap);
    
    attr.set_dictionary(dictValues);
    attr.set_stringMap_attributes(stringMapAttributes);
    

    let mut int64Values: HashMap<i32,i64> = HashMap::new();
    int64Values.insert(RESPONSE_SIZE,100);      // fake to test
    int64Values.insert(RESPONSE_SIZE,500);
    attr.set_int64_attributes(int64Values);

  
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
        log(&format!("comparing existing dictionary: {}",&dictValue));
        if value == dictValue {
            log(&format!("matched existing value"));
            return Some(index.abs());
        }
    }
    log(&format!("did not match"));
    return None;
}



// process request attribute,
// loop over request header and add to dictionary
// then use that to build new string map
fn process_request_attribute(
         headers_in:  ngx_http_headers_in_t,
         dictValues: &mut HashMap<i32,String>,
         stringAttributes: &mut HashMap<i32,String>,
         stringMapValues: &mut HashMap<i32,StringMap>)  {

    // fill in the string values
    let host = headers_in.host_str();
    log(&format!("request host {}",host));
    stringAttributes.insert(REQUEST_HOST,String::from(host));


    // fill in the string value
    let mut map: HashMap<i32,String> = HashMap::new();
  
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

    let mut requestValueMap = StringMap::new();
    requestValueMap.set_map(map);
    
    stringMapValues.insert(REQUEST_HEADER,requestValueMap);
}


static RESPONSE_HEADER_STATUS: i32 = 100;
static RESPONSE_HEADER_CONTENT_LENGTH: i32 = 101;

/**
 * proces response headers
 */
fn process_response_attribute(dictValues: &mut HashMap<i32,String>) -> StringMap {

     let mut map: HashMap<i32,String> = HashMap::new();

    
    dictValues.insert(RESPONSE_HEADER_STATUS,String::from(":status"));
    dictValues.insert(RESPONSE_HEADER_CONTENT_LENGTH,String::from("content-length"));

    map.insert(RESPONSE_HEADER_STATUS,String::from("200"));
    map.insert(RESPONSE_HEADER_CONTENT_LENGTH,String::from("213"));
    
    let mut stringMap = StringMap::new();
    stringMap.set_map(map);
    return stringMap;

}
