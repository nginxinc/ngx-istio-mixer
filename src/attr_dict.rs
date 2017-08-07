
use std::collections::HashMap;

use protobuf::well_known_types::Timestamp;
use attributes::Attributes;
use attributes::StringMap;


// TODO: use defaults

pub struct AttributeWrapper {

    pub attributes: Attributes,

    string_map: HashMap<String,i32>      // map of string to integer
}


impl AttributeWrapper  {

    pub fn new() -> AttributeWrapper {
        AttributeWrapper { attributes: Attributes::new(), string_map: HashMap::new() }
    }


    /**
     *  get index of the string key in the dictionary if not founded, then add
     */
    pub fn string_index(&mut self, key: &str) -> i32  {


        // have to declare scope, so multiple borrowing doesn't occur
        {
            let result = self.string_map.get_mut(key);
            if let Some(index) = result {
                return index.clone();
            }
        }


        let index = self.attributes.get_dictionary().len() as i32 + 1;
        self.attributes.mut_dictionary().insert( index , String::from(key));
        self.string_map.insert(String::from(key),index);

        index
    }

    // insert string attributes
    pub fn insert_string_attribute(&mut self, key: &str, value: &str) {
        let index = self.string_index(key).clone();
        self.attributes.mut_string_attributes().insert(index,String::from(value));
    }

    pub fn insert_int64_attribute(&mut self, key: &str, value: i64) {
        let index = self.string_index(key).clone();
        self.attributes.mut_int64_attributes().insert(index,value);
    }

    pub fn insert_time_stamp_attribute(&mut self, key: &str, value: Timestamp) {
        let index = self.string_index(key).clone();
        self.attributes.mut_timestamp_attributes().insert( index, value);
    }

    pub fn insert_string_map(&mut self, key: &str, value: HashMap<i32,String>) {
        let index = self.string_index(key).clone();
        let mut request_value_map = StringMap::new();
        request_value_map.set_map(value);
        self.attributes.mut_stringMap_attributes().insert(index,request_value_map);
    }
}
