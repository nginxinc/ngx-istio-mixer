use attr_global::GlobalList;

#[test]
fn global_list() {

    let list = GlobalList();
    assert_eq!(list.get("source.ip"), 0);
}
