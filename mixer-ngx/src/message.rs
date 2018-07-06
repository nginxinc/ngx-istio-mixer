use std::sync::mpsc::{ Sender,Receiver};
use std::sync::Mutex;


use ngx_mixer_transport::mixer_grpc::attributes::CompressedAttributes;

// Struct used to send and receive data from mixer hub
// Channels utilize mutexes so that communication can be
// Restricted to a single item at one time.
pub struct Channels<T> {
    pub tx: Mutex<Sender<T>>,
    pub rx: Mutex<Receiver<T>>
}

// Struct to hold mixer hub server and ports, as well as
// information to send
#[derive(Clone, Debug)]
pub struct MixerInfo  {
    pub server_name: String,
    pub server_port: u16,
    pub attributes: CompressedAttributes
}


