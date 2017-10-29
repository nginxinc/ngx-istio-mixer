// This file contains common traits shared among modules

use mixer::check::{ CheckResponse } ;
use transport::status:: { Status  };




// return status given checkresponse
pub struct TransportCallback {
    pub callback: Box<FnMut(&CheckResponse, Status)>,
}

impl TransportCallback {
    fn set_callback<CB: 'static + FnMut(&CheckResponse, Status)>(&mut self, c: CB) {
        self.callback = Box::new(c);
    }

    pub fn invoke(&mut self,response: &CheckResponse,status: Status) {
        (self.callback)(response, status);
    }
}

// trait for when check is done
pub trait CheckDoneCallBack {

    fn invoke();

}