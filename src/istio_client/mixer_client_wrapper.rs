/**
 * mixer client, based on Istio mixer client implementation
 */
use super::options::{ CheckOptions, ReportOptions, QuotaOptions };
use super::info::MixerServerInfo;
use super::check_cache::CheckCache;
use super::check_cache::CheckResult;
use super::quota_cache::QuotaCache;
use super::status:: { Status, StatusCodeEnum };

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

    pub fn check(&self, mixer_info: &MixerServerInfo) -> Status  {


        let result = self.check_cache.check(mixer_info.get_attributes());
        if result.is_cache_hit() && !result.get_status().ok() {
            // on_done(check_result->status());
            return Status::with_code(StatusCodeEnum::NOT_FOUND);

        }

        /*
        converter_.Convert(attributes, request.mutable_attributes());
        request.set_global_word_count(converter_.global_word_count());
        request.set_deduplication_id(deduplication_id_base_ +
            std::to_string(deduplication_id_.fetch_add(1)));

        // Need to make a copy for processing the response for check cache.
        Attributes *request_copy = new Attributes(attributes);
        auto response = new CheckResponse;
        // Lambda capture could not pass unique_ptr, use raw pointer.
        CheckCache::CheckResult *raw_check_result = check_result.release();
        QuotaCache::CheckResult *raw_quota_result = quota_result.release();
        if (!transport) {
            transport = options_.check_transport;
        }
        return transport(
            request, response, [this, request_copy, response, raw_check_result,
                raw_quota_result, on_done](const Status &status) {
        raw_check_result->SetResponse(status, *request_copy, *response);
        raw_quota_result->SetResponse(status, *request_copy, *response);
        if (on_done) {
        if (!raw_check_result->status().ok()) {
        on_done(raw_check_result->status());
        } else {
        on_done(raw_quota_result->status());
        }
        }
        delete raw_check_result;
        delete raw_quota_result;
        delete request_copy;
        delete response;

        if (InvalidDictionaryStatus(status)) {
        converter_.ShrinkGlobalDictionary();
        }
        });

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

        Status::new()
    }
}