use std::collections::HashMap;
use global_dict::GlobalDictionary;

pub struct MessageDictionary {

    global_dict: GlobalDictionary,
    message_words: Vec<String>,
    message_dict:  HashMap<String,i32>
}

impl MessageDictionary  {

    pub fn new(global_dict: GlobalDictionary) -> MessageDictionary  {

        MessageDictionary {
            global_dict,
            message_words: Vec::new(),
            message_dict: HashMap::new()
        }

    }

    //find index, try look up in the global, otherwise look up in the local
    pub fn index_of(&mut self, name: &str) -> i32  {

        if let Some(index) = self.global_dict.index_of(name) {
            return *index;
        }

        if let Some(index) = self.message_dict.get(name) {
           return *index;
        }

        let index = self.message_words.len();
        self.message_words.push(String::from(name));
        self.message_dict.insert(String::from(name),index as i32 );

        return index as i32;

    }

}


