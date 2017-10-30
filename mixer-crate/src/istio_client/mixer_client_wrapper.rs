/**
 * mixer client, based on Istio mixer client implementation
 */
extern crate futures;
extern crate grpc;

use futures::future::Future;
use super::options::{ CheckOptions, ReportOptions, QuotaOptions };
use super::check_cache:: { CheckCache } ;
use super::quota_cache::QuotaCache;
use transport::status:: { Status  };
use transport::mixer_grpc::Transport;
use mixer::check:: { CheckRequest, CheckResponse} ;
use attribute::global_dict::GlobalDictionary;
use attribute::message_dict::MessageDictionary;


use ngx_rust::nginx_http::log;

pub struct MixerClientOptions  {

    check_options: CheckOptions,
    report_options: ReportOptions,
    quota_options: QuotaOptions

}

impl MixerClientOptions {

    pub fn new() -> MixerClientOptions {

        MixerClientOptions{
            check_options: CheckOptions::new(),
            report_options: ReportOptions::new(),
            quota_options: QuotaOptions::new()
        }
    }
}



pub struct MixerClientWrapper {

    options: MixerClientOptions,
    check_cache: CheckCache,
    quota_cache: QuotaCache
}

impl MixerClientWrapper {

    pub fn new() -> MixerClientWrapper {

        MixerClientWrapper{
            options: MixerClientOptions::new(),
            check_cache: CheckCache::new(),
            quota_cache: QuotaCache::new()
        }
    }


    pub fn check<T: Transport>(&self,transport: T) -> Box<Future<Item = CheckResponse, Error=Status>>  {
    //pub fn check<T: Transport>(&self,transport: T) -> grpc::SingleResponse<CheckResponse>  {

        let attribute_wrapper = transport.get_attributes();

        /*
            TODO: implement cache,
            this should return future with immediate value
            let result = self.check_cache.check(attribute_wrapper);
            if result.is_cache_hit() && !result.get_status().ok() {
                // on_done(check_result->status());
                return Status::with_code(StatusCodeEnum::NOT_FOUND);

            }
        */


        // prepare input for check
        let mut message_dict = MessageDictionary::new(GlobalDictionary::new());
        let attributes = attribute_wrapper.as_attributes(&mut message_dict);

        let mut check_request = CheckRequest::new();
        check_request.set_attributes(attributes);
        check_request.set_global_word_count(message_dict.global_dict_size() as u32);

        log(&format!("ready to send check"));

        transport.check(check_request)
    }

}
