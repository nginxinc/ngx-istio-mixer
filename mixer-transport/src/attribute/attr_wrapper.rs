use std::collections::HashMap;

use protobuf::well_known_types::Timestamp;
use mixer_grpc::attributes::CompressedAttributes;
use mixer_grpc::attributes::StringMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{ Hash };
use std::net::Ipv4Addr;

use super::message_dict::MessageDictionary;

#[allow(dead_code)]
#[derive(Debug)]
enum AttrValue  {
    StrValue(String),
    I64(i64),
    Double(f64),
    Bool(bool),
    Timestamp(Timestamp),
    StringMap(HashMap<String,String>),
    Ip(Ipv4Addr)
}

#[derive(Debug)]
pub struct AttributeWrapper {

    values: HashMap<String,AttrValue>,       // map of value

}


impl AttributeWrapper  {

    pub fn new() -> AttributeWrapper {
        AttributeWrapper {
            values: HashMap::new()
        }
    }

    #[allow(dead_code)]
    pub fn key_exists(&self, key: &str) -> bool  {

        self.values.contains_key(key)
    }

    // hash the value found
    #[allow(dead_code)]
    pub fn hash(&self, key: &str,hashing: &mut DefaultHasher)  {

        if let Some(value) = self.values.get(key)  {

            match value  {
                &AttrValue::StrValue(ref str_value) => {
                    str_value.to_string().hash(hashing)
                },
                &AttrValue::I64(int_value) => {
                    int_value.hash(hashing)
                },

                &AttrValue::Double(d_value) => {
                    d_value.to_string().hash(hashing)
                },

                &AttrValue::Bool(b_value) => {
                    b_value.hash(hashing)
                },
                &AttrValue::Timestamp(ref t_value) => {
                    t_value.get_seconds().hash(hashing)
                },
                &AttrValue::StringMap(ref str_value) => {
                    for (_key, value) in str_value.iter() {
                        value.hash(hashing);
                    }
                },
                &AttrValue::Ip(ref ip_value) => {
                    ip_value.hash(hashing);
                }
            }

        }

    }


    // insert string attributes
    fn insert_value(&mut self, key: &str, value: AttrValue) {
        self.values.insert(String::from(key),value);

    }

    pub fn insert_string_attribute(&mut self, key: &str, value: &str) {
        if value.len() > 0 {
            self.insert_value(key, AttrValue::StrValue(String::from(value)));
        }
    }

    pub fn insert_int64_attribute(&mut self, key: &str, value: i64) {
        self.insert_value(key,AttrValue::I64(value));
    }

    #[allow(dead_code)]
    pub fn insert_f64_attribute(&mut self, key: &str, value: f64) {
        self.insert_value(key,AttrValue::Double(value));
    }

    #[allow(dead_code)]
    pub fn insert_bool_attribute(&mut self, key: &str, value: bool) {
        self.insert_value(key,AttrValue::Bool(value));
    }


    pub fn insert_time_stamp_attribute(&mut self, key: &str, value: Timestamp) {
        self.insert_value(key,AttrValue::Timestamp(value));
    }

    pub fn insert_string_map(&mut self, key: &str, value: HashMap<String,String>) {
        self.insert_value(key,AttrValue::StringMap(value));
    }

    pub fn insert_ip_attribute(&mut self, key:&str, value: Ipv4Addr) {
        self.insert_value(key,AttrValue::Ip(value));
    }

        // generate mixer attributes
    pub fn as_attributes(&self, dict: &mut MessageDictionary) -> CompressedAttributes  {

        let mut attrs = CompressedAttributes::new();

        for (key,value) in &self.values {

            let index = dict.index_of(key);
            match value  {
                &AttrValue::StrValue(ref str_value) =>  {
                    attrs.mut_strings().insert(index,dict.index_of(str_value.as_str()));
                },
                &AttrValue::I64(int_value) => {
                    attrs.mut_int64s().insert(index,int_value);
                },
                &AttrValue::Double(d_value) => {
                    attrs.mut_doubles().insert(index,d_value);
                },
                &AttrValue::Bool(b_value) => {
                    attrs.mut_bools().insert(index,b_value);
                },
                &AttrValue::Timestamp(ref t_value) => {
                   attrs.mut_timestamps().insert(index,t_value.clone());
                },
                &AttrValue::StringMap(ref str_value) => {
                    attrs.mut_string_maps().insert(index, map_string_map(str_value,  dict));
                }
                &AttrValue::Ip(ref ip_value) => {
                    attrs.mut_bytes().insert(index, ip_value.octets().to_vec());
                }

            }

        }

        for word in dict.get_words() {
            attrs.mut_words().push(word.clone());
        }

        return attrs;
    }
}

// convert rust hashmap of string to stringmap
fn map_string_map(string_map: &HashMap<String,String> , dict: &mut MessageDictionary) -> StringMap {

    let mut msg = StringMap::new();
    for (key, value) in string_map.iter()  {
        msg.mut_entries().insert(dict.index_of(key),dict.index_of(value));
    }

    return msg;

}
