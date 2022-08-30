use std::error::Error;

use wren::*;

fn main() -> Result<(), Box<dyn Error>>{
    let config = ConfigurationWrapper::<()>::new();

    let interpret_str = r#"System.print("Hello, world!")"#;

    let mut vm = VMWrapper::from_configuration(config);

    vm.interpret("main", interpret_str).map(|_| {})
}
