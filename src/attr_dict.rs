
use std::collections::HashMap;
use attributes::Attributes;


// TODO: use defaults

pub struct AttributeWrapper {

    pub attributes: Attributes,

    string_map: HashMap<String,u32>      // map of string to integer
}


impl AttributeWrapper  {

    pub fn new() -> AttributeWrapper {
        AttributeWrapper { attributes: Attributes::new(), string_map: HashMap::new() }
    }


    /**
     *  get index of the string key in the dictionary if not founded, then add
     */
    pub fn string_index(&mut self, key: &str) -> u32  {


        // have to declare scope, so multiple borrowing doesn't occur
        {
            let result = self.string_map.get_mut(key);
            if let Some(index) = result {
                return index.clone();
            }
        }


        let index = self.attributes.get_dictionary().len() as u32 + 1;
        self.attributes.mut_dictionary().insert( index as i32 , String::from(key));
        self.string_map.insert(String::from(key),index);

        index
    }
}
