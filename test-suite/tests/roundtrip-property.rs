#[macro_use]
extern crate quickcheck;

use toml::map::Map;
use toml::Value;

quickcheck! {
    fn value_roundtrip(raw_input: Value) -> bool {
        let input = match raw_input {
            Value::Table(_) => raw_input,
            _ => {
                let mut map = Map::new();
                map.insert("field".to_string(), raw_input);
                Value::Table(map)
            }
        };
        eprintln!("input = {:?}", input);
        match toml::to_string(&input.clone()) {
            Ok(serialized) => {
                let result = toml::from_str(&serialized);
                eprintln!("deserialized = {:?}",  result);
                eprintln!("<serialized>\n{}</serialized>\n", serialized);
                Ok(input) == result
            }
            Err(_) => false,
        }
    }
}
