use attr_global::GlobalList;
use attr_dict::AttributeWrapper;

#[test]
fn simple_test() {

    let global_dict = GlobalList();
    let attr_wrapper = AttributeWrapper::new();

    attr_wrapper.insert_string_attribute("source.ip","")
}
