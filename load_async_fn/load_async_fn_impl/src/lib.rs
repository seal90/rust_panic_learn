#[no_mangle]
pub extern "C-unwind" fn load_lib_fun() -> i32 {

    panic!("on no");

}