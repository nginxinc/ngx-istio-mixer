use mixer::attributes::Attributes;

pub trait MixerServerInfo  {

    fn get_server_name(&self) -> &str;

    fn get_server_port(&self) -> u16;

    fn get_attributes(&self) -> &Attributes;
}