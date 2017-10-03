use protobuf::well_known_types::Timestamp;
use std::collections::HashMap;

use global_dict::GlobalDictionary;
use message_dict::MessageDictionary;
use attr_wrapper::AttributeWrapper;


#[test]
fn simple_string_mapping() {

    let global_dict = GlobalDictionary::new();
    let mut dict = MessageDictionary::new(global_dict);
    let mut attr_wrapper = AttributeWrapper::new();

    attr_wrapper.insert_string_attribute("source.ip","10.0.0.0");
    attr_wrapper.insert_string_attribute("destination.ip","10.0.0.0");

    let attributes = attr_wrapper.as_attributes(&mut dict);
    let index = attributes.get_strings().get(&0).unwrap();
    assert_eq!(*index,-1);

    let destination_ip_index = dict.index_of("destination.ip");
    let index = attributes.get_strings().get(&destination_ip_index).unwrap();
    assert_eq!(*index,-1);

}

#[test]
fn simple_int64_mapping() {

    let global_dict = GlobalDictionary::new();
    let mut dict = MessageDictionary::new(global_dict);
    let mut attr_wrapper = AttributeWrapper::new();

    attr_wrapper.insert_int64_attribute("response.duration",50);

    let attributes = attr_wrapper.as_attributes(&mut dict);
    let response_dur_index = dict.index_of("response.duration");
    let duration = attributes.get_int64s().get(&response_dur_index).unwrap();
    assert_eq!(*duration,50);
}

#[test]
fn simple_double_mapping() {

    let global_dict = GlobalDictionary::new();
    let mut dict = MessageDictionary::new(global_dict);
    let mut attr_wrapper = AttributeWrapper::new();

    attr_wrapper.insert_f64_attribute("response.duration", 0.5_f64);

    let attributes = attr_wrapper.as_attributes(&mut dict);
    let response_dur_index = dict.index_of("response.duration");
    let duration = attributes.get_doubles().get(&response_dur_index).unwrap();
    assert_eq!(*duration,0.5_f64);
}

#[test]
fn simple_bool_mapping() {

    let global_dict = GlobalDictionary::new();
    let mut dict = MessageDictionary::new(global_dict);
    let mut attr_wrapper = AttributeWrapper::new();

    attr_wrapper.insert_bool_attribute("true", true);

    let attributes = attr_wrapper.as_attributes(&mut dict);
    let response_dur_index = dict.index_of("true");
    let duration = attributes.get_bools().get(&response_dur_index).unwrap();
    assert_eq!(*duration,true);
}

#[test]
fn simple_time_stamp() {

    let global_dict = GlobalDictionary::new();
    let mut dict = MessageDictionary::new(global_dict);
    let mut attr_wrapper = AttributeWrapper::new();

    let mut request_time = Timestamp::new();
    request_time.set_seconds(1000);
    attr_wrapper.insert_time_stamp_attribute("request.time", request_time);

    let attributes = attr_wrapper.as_attributes(&mut dict);
    let response_dur_index = dict.index_of("request.time");
    let duration = attributes.get_timestamps().get(&response_dur_index).unwrap();

    assert_eq!(duration.get_seconds(),1000);
}


#[test]
fn simple_stringmap_mapping() {
    let global_dict = GlobalDictionary::new();
    let mut dict = MessageDictionary::new(global_dict);
    let mut attr_wrapper = AttributeWrapper::new();

    let mut string_map: HashMap<String,String> = HashMap::new();
    string_map.insert(String::from("request.scheme"), String::from("http"));
    string_map.insert(String::from("request.useragent"), String::from("mac"));
    attr_wrapper.insert_string_map("request.headers", string_map);

    let attributes = attr_wrapper.as_attributes(&mut dict);


    let scheme_index = dict.index_of("request.scheme");
    let user_agent_index = dict.index_of("request.useragent");
    let http_index = dict.index_of("http");
    let header_index = dict.index_of("request.headers");

    let str_map = attributes.get_string_maps().get(&header_index).unwrap();
    let str_http_index = str_map.get_entries().get(&scheme_index).unwrap();
    assert_eq!(*str_http_index, http_index);
}
