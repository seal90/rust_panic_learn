use futures::FutureExt;
use libloading::{Library, Symbol};
use load_define::LoadAsyncTrait;


#[tokio::main]
async fn main() {
    let mut trait_impl = None;
    unsafe {
        // TODO change
        let file = "/Users/seal/github/rust_panic_learn/target/debug/libload_impl.dylib";
        
        // load many lib create by many developer
        if let Ok(lib) = Library::new(file) {
        
            // load factory func
            let load_lib_fun_result: std::result::Result<Symbol<unsafe extern "C-unwind" fn() -> Box<dyn LoadAsyncTrait>>,  libloading::Error> = lib.get(b"load_lib_fun");
            
            if let Ok(load_lib_fun) = load_lib_fun_result {
                let load_async_trait = load_lib_fun();
                trait_impl = Some(load_async_trait);
                
            }
        }
    }

    // do req
    let result = async {
        // AssertUnwindSafe moved to the future
        std::panic::AssertUnwindSafe(trait_impl.unwrap().load_fun()).catch_unwind().await
    }.await;

    // parse response
    if result.is_ok() {

    } else {
        println!("not ok");
    }
}
