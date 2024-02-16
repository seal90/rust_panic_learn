use async_trait::async_trait;
use load_define::LoadAsyncTrait;


#[no_mangle]
pub extern "C-unwind" fn load_lib_fun() -> Box<dyn LoadAsyncTrait> {

    return Box::new(LoadAsyncTraitImpl::new());
}

struct LoadAsyncTraitImpl {

}

impl LoadAsyncTraitImpl {
    pub fn new() -> Self {
        Self {  }
    }
}

#[async_trait]
impl LoadAsyncTrait for LoadAsyncTraitImpl {

    async fn load_fun(&self)->String {
        println!("hello panic");
        panic!("on no")
    }
}
