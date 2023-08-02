use std::env;

use phper::{
    modules::Module,
    functions::Argument,
    php_get_module, values::ZVal,
};

fn hello_ext(arguments: &mut [ZVal]) -> phper::Result<()> {
    let name = arguments[0].expect_z_str()?.to_str()?;
    match name {
        name => Ok(println!("Hello, {}!", name)),
    }
}

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    module
        .add_function("hello_ext", hello_ext)
        .argument(Argument::by_val("name"));

    module
}
