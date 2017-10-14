use istio_client::check_options::CheckOptions;

#[test]
fn test() {

    let ck_opt = CheckOptions::new();
     assert_eq!(ck_opt.network_fail_open,true);
}

