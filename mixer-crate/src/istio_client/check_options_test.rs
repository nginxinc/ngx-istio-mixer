use istio_client::options::CheckOptions;
use istio_client::options::ReportOptions;

#[test]
fn test_check_options() {

    let ck_opt = CheckOptions::new();
     assert_eq!(ck_opt.network_fail_open,true);
     assert_eq!(ck_opt.num_entries,10000);
}

#[test]
fn test_report_options() {
    let rep_options = ReportOptions::new();
    assert_eq!(rep_options.max_batch_entries,1000);
}

