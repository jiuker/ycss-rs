use lazy_static::lazy_static;
extern crate serde_json;
use std::io::Read;
use crate::watch;
use std::thread;
use std::ops::Add;
use std::sync::{Mutex, Arc};
use std::error;

use quick_xml::events::Event;
use quick_xml::Reader;
use std::convert::TryFrom;
use std::collections::HashMap;
use regex::Regex;

pub fn set_config_path(path:String,cb:fn(path:String))->Result<(),Box<dyn error::Error>> {
    let path_c = path.clone().add("/config.json");
    thread::spawn(move||{
        cb(path_c.to_owned());
    });
    watch::watch::watch_dir(vec![path.clone()], "json".to_owned(),false, cb)?;
    Ok(())
}
#[derive(Debug,serde_derive::Serialize,serde_derive::Deserialize, Clone)]
pub struct YConfig{
   pub debug:bool,
   pub hType:String,
   pub common:Vec<String>,
   pub single:Vec<String>,
   pub watchDir:Vec<String>,
   pub oldCssReg:String,
   pub reg:Vec<String>,
   pub needZoomUnit:String,
   pub zoom:f32,
   pub outUnit:String,
   pub outPath:String,
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
            oldCssReg: "".to_string(),
            reg: vec![],
            needZoomUnit: "".to_string(),
            zoom: 0.0,
            outUnit: "".to_string(),
            outPath: "".to_string()
        }
    }
}

lazy_static! {
    #[derive(Debug)]
    pub static ref YCONF:Arc<Mutex<YConfig>> = Arc::new(Mutex::new(YConfig::new()));
    #[derive(Debug)]
    pub static ref COMMON:Arc<Mutex<HashMap<String,Regex>>> = Arc::new(Mutex::new(HashMap::new()));
    #[derive(Debug)]
    pub static ref SINGAL:Arc<Mutex<HashMap<String,Regex>>> = Arc::new(Mutex::new(HashMap::new()));
}
// pub static  COMMON:Arc<Mutex<HashMap<String,Regex>>> = Arc::new(Mutex::new(HashMap::new()));
// pub static  SINGAL:Arc<Mutex<HashMap<String,Regex>>> = Arc::new(Mutex::new(HashMap::new()));
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
    {
        let mut common_c = COMMON.lock()?;
        (*common_c) = read_reg_file((*yconf_c).common.clone())?;
        let mut singal_c = SINGAL.lock()?;
        (*singal_c) = read_reg_file((*yconf_c).single.clone())?;
    }
    Ok(())
}
pub fn is_debug() ->Result<bool,Box<dyn error::Error>>{
    let yconf_c = YCONF.lock()?;
    Ok(yconf_c.is_debug())
}

pub fn read_reg_file(paths:Vec<String>)->Result<HashMap<String,Regex>, Box<dyn error::Error>>{
    println!("start read regexp!");
    let mut common_keys:Vec<String> = vec![];
    let mut common_values:Vec<String> = vec![];
    for p in paths{
        let mut reader = Reader::from_file(p.clone())?;
        let mut buf = Vec::new();
        loop{
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"css"=>{
                            let attr = e.attributes().map(|x| x.unwrap().value).collect::<Vec<_>>();
                            for x in attr {
                                common_keys.push(String::from_utf8(x.to_vec())?);
                            }
                        },
                        _=>{}
                    }
                },
                Ok(Event::Text(t))=> {
                    let _text = t.unescape_and_decode(&reader)?;
                    if _text.trim()!=""{
                        common_values.push(_text.trim().to_string());
                    }
                },
                Ok(Event::Eof) => break,
                Err(e) =>{
                    println!("reader err is {:?} and position is {}",e,reader.buffer_position());
                    break
                }
                _ => {}
            }
        }
    }
    println!("keys   is {:?}",common_keys);
    println!("values is {:?}",common_values);
    if common_keys.len() != common_values.len(){
        return Err(Box::try_from("通用配置出现异常!")?);
    }
    let mut common_reg_map:HashMap<String,Regex> = HashMap::new();
    let mut index = 0;
    while  index<common_values.len(){
        common_reg_map.insert(common_values[index].clone(),Regex::new(common_keys[index].as_str())?);
        index = index + 1;
    }
    println!("reg_map is{:?}",common_reg_map);
    println!("read reg file done!");
    Ok(common_reg_map)
}