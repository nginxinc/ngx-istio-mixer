// This file contains common traits shared among modules

use attribute::attr_wrapper::AttributeWrapper;
use mixer::check::CheckResponse;

pub trait MixerServerInfo  {

    fn get_server_name(&self) -> &str;

    fn get_server_port(&self) -> u16;

    fn get_attributes(&self) -> &AttributeWrapper;
}

pub struct TransportCallback {
    pub callback: Box<FnMut(&CheckResponse)>,
}

impl TransportCallback {
    fn set_callback<CB: 'static + FnMut(&CheckResponse)>(&mut self, c: CB) {
        self.callback = Box::new(c);
    }

    pub fn invoke(&mut self,response: &CheckResponse) {
        (self.callback)(response);
    }
}