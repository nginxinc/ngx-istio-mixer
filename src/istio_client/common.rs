// This file contains common traits shared among modules

use attribute::attr_wrapper::AttributeWrapper;

pub trait MixerServerInfo  {

    fn get_server_name(&self) -> &str;

    fn get_server_port(&self) -> u16;

    fn get_attributes(&self) -> &AttributeWrapper;
}

