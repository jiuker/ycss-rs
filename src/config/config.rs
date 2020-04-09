use lazy_static::lazy_static;
extern crate serde_json;
use std::io::Read;
use crate::watch;
use std::thread;
use std::ops::Add;
use std::sync::{Mutex, Arc};
use std::error;
pub fn set_config_path(path:String,cb:fn(path:String)) {
    let path_c = path.clone().add("/config.json");
    thread::spawn(move||{
        cb(path_c.to_owned());
    });
    watch::watch::watch_dir(vec![path.clone()], "json".to_owned(),false, cb).unwrap();
    println!("watch unlock");
}
#[derive(Debug,serde_derive::Serialize,serde_derive::Deserialize, Clone)]
struct YConfig{
    debug:bool,
    hType:String,
    common:Vec<String>,
    single:Vec<String>,
    watchDir:Vec<String>,
    oldCssReg:String,
}
impl YConfig{
    pub fn is_debug(&self)->bool{
        return self.debug
    }
    pub fn new() -> YConfig{
        YConfig{
            debug: false,
            hType: "".to_string(),
            common: vec![],
            single: vec![],
            watchDir: vec![],
            oldCssReg: "".to_string()
        }
    }
}
lazy_static! {
    static ref YCONF:Arc<Mutex<YConfig>> = Arc::new(Mutex::new(YConfig::new()));
}
pub fn load_config(path:String, cb:fn(path:String))->Result<(),Box<dyn error::Error>>{
    println!("set config path is {}",path);
    let mut f = std::fs::File::open(path)?;
    let mut file_body = String::from("");
    f.read_to_string(&mut file_body)?;
    println!("load config:\r\n {}",file_body.to_string());
    let _yconf:YConfig = serde_json::from_str(file_body.as_str())?;
    let mut yconf_c = YCONF.lock()?;
    (*yconf_c) = _yconf.clone();
    watch::watch::watch_dir((*yconf_c).clone().watchDir, (*yconf_c).clone().hType.to_owned(),true, cb)?;
    println!("watch unlock");
    Ok(())
}
pub fn is_debug() ->Result<bool,Box<dyn error::Error>>{
    let yconf_c = YCONF.lock()?;
    Ok(yconf_c.is_debug())
}