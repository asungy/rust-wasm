use wasmtime::{
    Engine,
    Instance,
    Module,
    Store,
};
use std::os::raw::c_char;

fn main() -> Result<(), anyhow::Error>{
    let engine = Engine::default();
    let module = Module::from_file(&engine, "./target/wasm32-unknown-unknown/debug/client.wasm")?;
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;

    // my_func
    let my_func = instance.get_func(&mut store, "my_func")
        .expect("`my_func` was not an exported function");
    let my_func = my_func.typed::<(i32, i32), i32>(&store)?;
    let result = my_func.call(&mut store, (2, 7))?;
    println!("Answer: {}", result);

    // other_func
    let other_func = instance.get_func(&mut store, "other_func")
        .expect("`other_func` was not an exported function");
    let other_func = other_func.typed::<(), u32>(&store)?;
    let string_ptr = other_func.call(&mut store, ())?;

    let memory = instance.get_memory(&mut store, "")
        .expect("Could not find default memory");


    Ok(())
}
