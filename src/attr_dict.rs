
use std::collections::HashMap;

use protobuf::well_known_types::Timestamp;
use attributes::Attributes;
use attributes::StringMap;

use ngx_rust::nginx_http::log;

use message_dict::MessageDictionary;


enum AttrValue  {
    StrValue(String),
    I64(i64),
    Double(f64),
    Bool(bool),
    Timestamp(Timestamp)
}

// TODO: use defaults

pub struct AttributeWrapper {

    values: HashMap<String,AttrValue>,       // map of value

    string_map: HashMap<String,String>      // map of string to integer
}


impl AttributeWrapper  {

    pub fn new() -> AttributeWrapper {
        AttributeWrapper {
            values: HashMap::new(),
            string_map: HashMap::new()
        }
    }



    // insert string attributes
    fn insert_value(&mut self, key: &str, value: AttrValue) {
        self.values.insert(String::from(key),value);

    }

    pub fn insert_string_attribute(&mut self, key: &str, value: &str) {
        self.insert_value(key, AttrValue::StrValue(String::from(value)));
    }

    pub fn insert_int64_attribute(&mut self, key: &str, value: i64) {
        self.insert_value(key,AttrValue::I64(value));
    }

    pub fn insert_time_stamp_attribute(&mut self, key: &str, value: Timestamp) {
        self.insert_value(key,AttrValue::Timestamp(value));
    }

    pub fn insert_string_map(&mut self, key: &str, value: HashMap<String,String>) {

    }

        // generate mixer attributes
    pub fn as_attributes(&mut self,mut dict:  MessageDictionary) -> Attributes  {

        let mut attrs = Attributes::new();

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
                   // attrs.mut_timestamps().insert(index,tValue);
                }

            }

        }

        return attrs;
    }
}
