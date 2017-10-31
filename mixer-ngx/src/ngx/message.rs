use std::sync::mpsc::{ Sender,Receiver};
use std::sync::Mutex;


use ngx_mixer_transport::mixer_grpc::attributes::Attributes;

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


