/**
 * mixer client, based on Istio mixer client implementation
 */
use super::options::{ CheckOptions, ReportOptions, QuotaOptions };
use super::common::MixerServerInfo;
use super::check_cache::CheckCache;
use super::quota_cache::QuotaCache;
use super::status:: { Status, StatusCodeEnum };
use mixer::check::CheckRequest;
use mixer::check::CheckResponse;
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


impl MixerClientWrapper  {

    pub fn new() -> MixerClientWrapper {

        MixerClientWrapper{
            options: MixerClientOptions::new(),
            check_cache: CheckCache::new(),
            quota_cache: QuotaCache::new()
        }
    }

    pub fn check(&self, mixer_info: &MixerServerInfo,
                 transport: fn(request: CheckRequest, info: &MixerServerInfo)) -> Status  {


        let attribute_wrapper = mixer_info.get_attributes();
        let result = self.check_cache.check(attribute_wrapper);
        if result.is_cache_hit() && !result.get_status().ok() {
            // on_done(check_result->status());
            return Status::with_code(StatusCodeEnum::NOT_FOUND);

        }

        let mut message_dict = MessageDictionary::new(GlobalDictionary::new());
        let attributes = attribute_wrapper.as_attributes(&mut message_dict);

        let mut check_request = CheckRequest::new();
        check_request.set_attributes(attributes);
        check_request.set_global_word_count(message_dict.global_dict_size() as u32);

        log(&format!("ready to send check"));
        /*
        let response_closure = | response: CheckResponse | {
            log(&format!("response is ready to process in client wrapper"));
        };
        */
        transport(check_request, mixer_info);

        log(&format!("returning ok for check"));
        Status::new()
    }
}