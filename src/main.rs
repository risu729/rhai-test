use rhai::plugin::*;
use rhai::serde::{DynamicDeserializer, from_dynamic};
use rhai::{Dynamic, Engine};
use serde::Deserialize;
use serde::de::Deserializer;

fn main() {
    let mut engine = Engine::new();
    engine
        .register_type_with_name::<MyEnum>("MyEnum")
        .register_static_module("MyEnum", exported_module!(MyEnumModule).into());

    let script = r#"MyEnum::Foo"#;

    let result = engine.eval::<MyEnum>(script).unwrap();
    println!("{:?}", result); // Output: Foo

    let dynamic = engine.eval::<Dynamic>(script).unwrap();

    let result = dynamic.clone().try_cast_result::<MyEnum>().unwrap();
    println!("{:?}", result); // Output: Foo

    let result: MyEnum = from_dynamic(&dynamic).unwrap();
    println!("{:?}", result); // ErrorMismatchOutputType("rhai_test::MyEnum", "rhai_test::MyEnum", none)
}

#[derive(Debug, Clone, Deserialize)]
enum MyEnum {
    Foo,
    Bar,
}

#[export_module]
#[allow(non_snake_case)]
mod MyEnumModule {
    #[allow(non_upper_case_globals)]
    pub const Foo: MyEnum = MyEnum::Foo;
    #[allow(non_upper_case_globals)]
    pub const Bar: MyEnum = MyEnum::Bar;
}
