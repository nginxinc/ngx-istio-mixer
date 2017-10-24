use std::sync::mpsc::{ Sender,Receiver};
use std::sync::Mutex;


use mixer::attributes::Attributes;
use istio_client::info::MixerServerInfo;

pub struct Channels<T> {
    pub tx: Mutex<Sender<T>>,
    pub rx: Mutex<Receiver<T>>
}


#[derive(Clone, Debug)]
pub struct MixerInfo  {
    pub server_name: String,
    pub server_port: u16,
    pub attributes: Attributes
}

impl MixerServerInfo for MixerInfo  {


    fn get_server_name(&self) -> &str {
        &self.server_name
    }

    fn get_server_port(&self) -> u16 {
        self.server_port
    }

    fn get_attributes(&self) -> &Attributes {
        &self.attributes
    }
}