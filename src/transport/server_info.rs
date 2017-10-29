use attribute::attr_wrapper::AttributeWrapper;




pub struct MixerInfo  {
    pub server_name: String,
    pub server_port: u16,
}


impl MixerInfo  {


    pub fn get_server_name(&self) -> &str {
        &self.server_name
    }

    pub fn get_server_port(&self) -> u16 {
        self.server_port
    }

}

