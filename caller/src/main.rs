use wasmtime::*;

fn main() -> Result<(), anyhow::Error>{
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let module = Module::from_file(&engine, "./target/wasm32-unknown-unknown/debug/client.wasm")?;
    let linker = Linker::new(&engine);
    let instance = linker.instantiate(&mut store, &module)?;

    let memory = instance.get_memory(&mut store, "memory").unwrap();
    let get_hello_message = instance
        .get_typed_func::<(), i32>(&mut store, "get_hello_message")?;
    let get_message_len = instance
        .get_typed_func::<(), i32>(&mut store, "get_message_len")?;

    let message_ptr = get_hello_message.call(&mut store, ())? as usize;
    let message_len = get_message_len.call(&mut store, ())? as usize;

    let message_bytes = memory.data(&mut store)[message_ptr..message_ptr+message_len].to_vec();
    let message = String::from_utf8(message_bytes).unwrap();

    println!("Message from WASM: {}", message);

    Ok(())
}
