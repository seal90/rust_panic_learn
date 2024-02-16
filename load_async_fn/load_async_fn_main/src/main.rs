use libloading::{Library, Symbol};

fn main() {
    unsafe {
        // TODO change
        let file = "/Users/seal/github/rust_panic_learn/target/debug/libload_async_fn_impl.dylib";
        
        // load many lib create by many developer
        if let Ok(lib) = Library::new(file) {
        
            let load_lib_fun_result: std::result::Result<Symbol<unsafe extern "C-unwind" fn() -> i32>,  libloading::Error> = lib.get(b"load_lib_fun");
            
            if let Ok(load_lib_fun) = load_lib_fun_result {
                let result = std::panic::catch_unwind(||{
                    let load_async_trait = load_lib_fun();
                    println!("result : {}", load_async_trait);
                });
                if result.is_ok() {

                } else {
                    println!("not ok")
                }
            }
        }
    }
}
