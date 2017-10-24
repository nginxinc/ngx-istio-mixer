/**
 * mixer client, based on Istio mixer client implementation
 */
use super::options::{ CheckOptions, ReportOptions, QuotaOptions };
use super::info::MixerServerInfo;

pub struct MixerClientOptions  {

    check_options: CheckOptions,
    report_options: ReportOptions,
    quota_options: QuotaOptions

}

pub struct MixerClientWrapper {

}

impl MixerClientWrapper  {

    pub fn new() -> MixerClientWrapper {
        MixerClientWrapper{}
    }

    pub fn check(&self, mixer_info: &MixerServerInfo) -> bool  {


        /*

        let client = MixerClient::new_plain( &info.server_name, info.server_port , Default::default()).expect("init");

        let mut check_request = CheckRequest::new();
        check_request.set_attributes(info.attributes);

        let result = client.check(grpc::RequestOptions::new(), check_request).wait();

        //       log(&format!("mixer check {:?}",result));
        match result   {
            Ok(response) =>  {
                let (m1, check_response, m2) = response;
                cache.set_reponse(&check_response);
            },

            Err(err)  =>  {
                // TODO: fix log error to nginx error logger
                log(&format!("error calling check {:?}",err));
            }

        }*/
        false
    }
}