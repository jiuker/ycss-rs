extern crate ycss;

use ycss::config::config;
use std::thread::sleep;
use std::time::Duration;
use ycss::repl::repl::Repl;
use ycss::repl::vue::VueRepl;

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
        let cls = rep.get_class();
        println!("cls is {:?}",cls);
        let new_css = rep.get_new_class(cls);
        println!("new_css is {}",new_css);
    }
}