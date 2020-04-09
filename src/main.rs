extern crate ycss;

use ycss::config::config;
use std::thread::sleep;
use std::time::Duration;
use ycss::repl::repl::Repl;
use ycss::repl::vue::VueRepl;
use select::document::Document;
use select::predicate::{ Class};

fn main() {
    config::set_config_path("./res/config".to_owned(),file_change);
    sleep(Duration::from_secs(24*60*60))
}
fn file_change(path:String){
    println!("path is{}",path);
    if path.contains("./res/config/config.json"){
        match config::load_config(path,file_change) {
            Ok(())=>println!("set config watch ok!"),
            Err(e)=>println!("err is {:?}",e)
        }
    }else{
        // 不是配置文件变动
        println!("get {} changed!",path);
        let rep:VueRepl = Repl::new(path.clone());
        let fileBody = rep.get_file_body();
        let doc = Document::from(fileBody.as_str());
        for node in doc.find(Class("test")){
            println!("{}",node.text());
        }
    }
}