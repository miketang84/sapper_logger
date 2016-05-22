
extern crate sapper;
extern crate sapper_request_basic_logger;
extern crate env_logger;
#[macro_use]
extern crate log;

use sapper::{SApp, SAppWrapper, Request, Response, Result};



mod biz;
use biz::Biz;



#[derive(Clone)]
struct MyApp;
// must impl it
// total entry and exitice
impl SAppWrapper for MyApp {
    fn before(&self, req: &mut Request) -> Result<()> {
        println!("{}", "in SAppWrapper before.");
        sapper_request_basic_logger::init(req).unwrap();
        
        Ok(())
    }
    
    fn after(&self, req: &Request, res: &mut Response) -> Result<()> {
        println!("{}", "in SAppWrapper after.");
        
        sapper_request_basic_logger::log(req, res).unwrap();
        Ok(())
    }
}



pub fn main() {
    env_logger::init().unwrap();
    
    let mut sapp = SApp::new();
    sapp.address("127.0.0.1")
        .port(1337)
        .with_wrapper(MyApp)
        .add_module(Box::new(Biz));
    
    println!("Listening on http://127.0.0.1:1337");
    sapp.run();
    
}
