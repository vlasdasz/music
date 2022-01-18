use serde::{Deserialize, Serialize};

pub fn ser<T: Serialize>(value: &T) -> Vec<u8> {
    match bincode::serialize(value) {
        Err(e) => {
            let type_name = std::any::type_name::<T>();
            panic!("Failed to serialize {type_name}: {e}")
        }
        Ok(a) => a,
    }
}

pub fn de<'a, T: Deserialize<'a>>(buff: &'a [u8]) -> T {
    match bincode::deserialize(buff) {
        Err(e) => {
            let type_name = std::any::type_name::<T>();
            panic!("Failed to deserialize {type_name}: {e}")
        }
        Ok(a) => a,
    }
}
