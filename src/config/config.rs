extern crate serde_json;
use std::io::Read;
use crate::watch;
use std::thread;
use std::ops::Add;

pub fn set_config_path(path:String,cb:fn(path:String)) {
    let path_c = path.clone().add("/config.json");
    thread::spawn(move||{
        cb(path_c.to_owned());
    });
    watch::watch::watch_dir(vec![path.clone()], "json".to_owned(), cb).unwrap();
}
#[derive(Debug,serde_derive::Serialize,serde_derive::Deserialize)]
struct YConfig{
    debug:bool,
    hType:String,
    common:Vec<String>,
    single:Vec<String>,
    watchDir:Vec<String>,
}
impl YConfig{
    pub fn is_debug(&self)->bool{
        return self.debug
    }
}
pub fn load_config(path:String,cb:fn(path:String)){
    println!("set config path is {}",path);
    let mut f = std::fs::File::open(path).unwrap();
    let mut file_body = String::from("");
    f.read_to_string(&mut file_body).unwrap();
    println!("load config:\r\n {}",file_body.to_string());
    let yconf:YConfig = serde_json::from_str(file_body.as_str()).unwrap();
    println!("{:?}",yconf);
    watch::watch::watch_dir(yconf.watchDir, yconf.hType.to_owned(), cb).unwrap();
}