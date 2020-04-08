extern crate ycss;

use ycss::config::config;

fn main() {
    config::set_config_path("./res/config".to_owned(),config_change);
}
fn config_change(path:String){
    config::load_config(path,file_change)
}
fn file_change(path:String){
    println!("get {} changed!",path)
}