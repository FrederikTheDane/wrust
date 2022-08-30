use crate::VMWrapper;

pub fn default_write_fn<T>(_: &VMWrapper<T>, text: &str) {
    println!("{}", text);
}

