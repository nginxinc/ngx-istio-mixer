extern crate mixer;



#[cfg(test)]
mod tests {

    use mixer::attr_dict::AttributeWrapper;
    use mixer::encode:: { encode_istio_header, decode_istio_header };
    use std::collections::HashMap;

    #[test]
    fn check_first_index() {

        let mut dict = AttributeWrapper::new();
        let index = dict.string_index("key1");
        assert_eq!(index,1);

        let index2 = dict.string_index("key2");
        assert_eq!(index2,2);

        let index3 = dict.string_index("key1");
        assert_eq!(index3,1);

        let index4 = dict.string_index("key2");
        assert_eq!(index4,2);

    }


    #[test]
    fn check_encoding() {

        let mut list = Vec::new();

        list.push( ( "source_ip","10.0.0.0"));
        list.push( ("source_uid", "kubernetes://productpage-v1-3990756607-plqt5.default"));

        let encoded = encode_istio_header(&list);

        assert_eq!(&encoded,"c291cmNlX2lwQDEwLjAuMC4wIXNvdXJjZV91aWRAa3ViZXJuZXRlczovL3Byb2R1Y3RwYWdlLXYxLTM5OTA3NTY2MDctcGxxdDUuZGVmYXVsdCE=");

        let decoded_map = decode_istio_header(&encoded);

        let ip_result = decoded_map.get("source_ip");

        match ip_result  {
            Some(ip) =>   assert_eq!(ip,"10.0.0.0"),
            None => assert!(true, "source ip not founded")
        }

        let uid_result = decoded_map.get("source_uid");

        match uid_result  {
            Some(uid) =>   assert_eq!(uid,"kubernetes://productpage-v1-3990756607-plqt5.default"),
            None => assert!(true, "source uid not founded")
        }


    }
}