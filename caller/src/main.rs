use wasmtime::{
    Engine,
    Instance,
    Module,
    Store,
};

fn main() -> Result<(), anyhow::Error>{
    let engine = Engine::default();
    let module = Module::from_file(&engine, "./target/wasm32-unknown-unknown/debug/client.wasm")?;
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;
    let my_func = instance.get_func(&mut store, "my_func")
        .expect("`my_func` was not an exported function");
    let my_func = my_func.typed::<(i32, i32), i32>(&store)?;
    let result = my_func.call(&mut store, (2, 7))?;
    println!("Answer: {}", result);

    Ok(())
}
