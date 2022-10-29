use recursive_regex::JustStrDeserializer;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq)]
struct Data<T>(T);

macro_rules! test_type {
    ($t:ty, $name:ident, $data:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let data_str = $data;
            let data_struct = Data::<$t>::deserialize(JustStrDeserializer::from_str(data_str));
            assert_eq!(data_struct, Ok(Data($expected)))
        }
    };
}

test_type!(bool, test_bool, "true", true);
test_type!(i32, test_i32, "-0324", -324);
test_type!(u32, test_u32, "52", 52);
test_type!(f32, test_f32, "4235.2", 4235.2);
test_type!(char, test_char, "d", 'd');
test_type!(
    String,
    test_string,
    "hello world",
    "hello world".to_string()
);
test_type!(&str, test_str, "hello world", "hello world");
test_type!(
    Option<&str>,
    test_option,
    "hello world",
    Some("hello world")
);
test_type!((), test_unit, "yf78iy f37y", ());
