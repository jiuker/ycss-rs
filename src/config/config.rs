use lazy_static::lazy_static;
extern crate serde_json;
use crate::run::runner::Result;
use quick_xml::events::Event;
use quick_xml::Reader;
use regex::Regex;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::sync::{Arc, Mutex};

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize, Clone)]
pub struct YConfig {
    pub debug: bool,
    pub h_type: String,
    pub common: Vec<String>,
    pub single: Vec<String>,
    pub watch_dir: Vec<String>,
    pub old_css_reg: String,
    pub reg: Vec<String>,
    pub page_common: Vec<String>,
    pub need_zoom_unit: String,
    pub zoom: f32,
    pub out_unit: String,
    pub out_path: String,
    pub static_map: HashMap<String, String>,
}
impl YConfig {
    pub fn is_debug(&self) -> bool {
        return self.debug;
    }
    pub fn new() -> YConfig {
        YConfig {
            debug: false,
            h_type: "".to_string(),
            common: vec![],
            single: vec![],
            watch_dir: vec![],
            old_css_reg: "".to_string(),
            reg: vec![],
            page_common: vec![],
            need_zoom_unit: "".to_string(),
            zoom: 0.0,
            out_unit: "".to_string(),
            out_path: "".to_string(),
            static_map: Default::default(),
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
// keys，values，写入的地方
#[macro_export(set_reg_hash)]
macro_rules! set_reg_hash {
    ($key:ident,$value:ident,$hash:expr) => {
        let mut index = 0;
        while index < $value.len() {
            $hash.insert($value[index].clone(), Regex::new($key[index].as_str())?);
            index = index + 1;
        }
    };
}
pub fn read_reg_file(paths: Vec<String>) -> Result<HashMap<String, Regex>> {
    println!("start read regexp!");
    let mut common_keys: Vec<String> = vec![];
    let mut common_values: Vec<String> = vec![];
    for p in paths {
        let mut reader = Reader::from_file(&p)?;
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name() {
                    b"css" => {
                        let attr = e
                            .attributes()
                            .map(|x| match x {
                                Ok(d) => return Some(d.value),
                                Err(_) => return None,
                            })
                            .collect::<Option<Vec<_>>>();
                        for x in match attr {
                            Some(d) => d,
                            None => vec![],
                        } {
                            common_keys.push(String::from_utf8(x.to_vec())?);
                        }
                    }
                    _ => {}
                },
                Ok(Event::Text(t)) => {
                    let _text = t.unescape_and_decode(&reader)?;
                    if _text.trim() != "" {
                        common_values.push(_text.trim().to_string());
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    println!(
                        "reader err is {:?} and position is {}",
                        e,
                        reader.buffer_position()
                    );
                    break;
                }
                _ => {}
            }
        }
    }
    println!("keys   is {:?}", common_keys);
    println!("values is {:?}", common_values);
    if common_keys.len() != common_values.len() {
        return Err(Box::from("通用配置出现异常!"));
    }
    let mut common_reg_map: HashMap<String, Regex> = HashMap::new();
    set_reg_hash!(common_keys, common_values, common_reg_map);
    println!("reg_map is{:?}", common_reg_map);
    println!("read reg file done!");
    Ok(common_reg_map)
}
