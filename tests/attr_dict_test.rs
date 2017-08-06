extern crate mixer;


#[cfg(test)]
mod tests {

    use mixer::attr_dict::AttributeWrapper;

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
}