extern crate serde_json;
use std::io::Read;
use std::borrow::Borrow;

pub fn set_config_path(path:String) {
    println!("set config path is {}",path);
    let mut f = std::fs::File::open(path).unwrap();
    let mut file_body = String::from("");
    f.read_to_string(&mut file_body).unwrap();
    println!("load config:\r\n {}",file_body.to_string());
    let yconf:YConfig = serde_json::from_str(file_body.as_str()).unwrap();
    println!("{:?}",yconf);
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
    pub fn Debug(&self)->bool{
        return self.debug
    }
}