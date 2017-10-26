use std::vec::Vec;
use ngx_rust::nginx_http::log;

use mixer::check::{ ReferencedAttributes, ReferencedAttributes_Condition } ;
use attribute::global_dict::Get_Global_Words;
use attribute::attr_wrapper::AttributeWrapper;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct Referenced {

    absence_keys: Vec<String>,
    exact_Keys: Vec<String>
}

impl Referenced {

    pub fn new() -> Referenced {
        Referenced {
            absence_keys: vec![],
            exact_Keys: vec![]
        }
    }


    pub fn fill(&mut self, reference: &ReferencedAttributes) -> bool {
        let global_words = Get_Global_Words();
        let mut name: &str;

        for attr_match in reference.get_attribute_matches() {
            let idx = attr_match.get_name() as usize;
            if idx >= 0 {
                if idx >= global_words.len() {
                    log(&format!("Global word index is too big: {}, >= {}", idx, global_words.len()));
                    return false;
                }
                name = global_words[idx];
            } else {
                // per-message index is negative, its format is:
                //    per_message_idx = -(array_idx + 1)
                let m_idx = (- (idx as i32) - 1) as usize;
                if m_idx >= reference.get_words().len() as usize {
                    log(&format!("Per message word index is too big: {}, >= {}", idx, reference.get_words().len()));
                    return false;
                }
                name = &reference.get_words()[m_idx];
            }

            match attr_match.get_condition() {
                ReferencedAttributes_Condition::ABSENCE => self.absence_keys.push(name.to_string()),
                ReferencedAttributes_Condition::EXACT => self.exact_Keys.push(name.to_string()),
                ReferencedAttributes_Condition::REGEX => {
                    log(&format!("Received REGEX in Reference Attributes {}", name));
                    return false;
                },
                _ => {}
            }
        }

        true
    }

    // compute signature based on attribute content and extra key provided

    pub fn signature(&self, attributes: &AttributeWrapper, extra_key: &str) -> Option<u64> {
        // if an "absence" key exists, return false for mis-match.
        for key in &self.absence_keys {
            if attributes.key_exists(&key) {
                return None;
            }
        }

        let mut hashing = DefaultHasher::new();

        for key in &self.exact_Keys {
            attributes.hash(&key, &mut hashing);
        }

        extra_key.hash(&mut hashing);

        let hash_value = hashing.finish();

        return Some(hash_value)
    }
}

impl Hash for Referenced {

    fn hash<H: Hasher>(&self, state: &mut H) {

        let mut sorted_absence_keys = self.absence_keys.clone();
        sorted_absence_keys.sort();

        for key in sorted_absence_keys {
            key.hash(state);
        }

        let mut sorted_exact_keys = self.exact_Keys.clone();
        sorted_exact_keys.sort();

        for key in sorted_exact_keys {
            key.hash(state);
        }

    }

}

