use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{SystemTime, Duration};
use std::collections::HashMap;
use std::error;
use crate::watch::watch::read_all_paths;
use std::convert::TryFrom;
use crate::config::config::{read_reg_file};
use regex::Regex;
use std::io::Read;
use crate::config::config::{YCONF, COMMON, SINGAL, YConfig};
use std::thread::{spawn, sleep};

#[derive(Debug,Clone)]
pub struct Runner<'a>{
    config_path:&'a str,
    config_file_watch:Arc<Mutex<HashMap<String,SystemTime>>>,
    normal_file_watch:Arc<Mutex<HashMap<String,SystemTime>>>,
}
pub enum FileType{
    Config(String),
    Normal(String)
}
const WATCH_FILE_MAX:i32=10000;
impl <'a>Runner<'a>{
    pub fn new(path:&'a str)->Self{
        let run = Runner{
            config_path: path,
            config_file_watch: Arc::new(Mutex::new(Default::default())),
            normal_file_watch: Arc::new(Mutex::new(Default::default()))
        };
        run.load_config(path).unwrap();
        run
    }
    pub fn add_dir_watch(&self,dir:Vec<String>,file_type:String,typ:FileType)->Result<(),Box<dyn error::Error>>{
        match typ {
            FileType::Config(d)=>{
                let mut paths:Vec<String> = vec![];
                for _dir in dir {
                    for path in  read_all_paths(_dir,file_type.clone())?{
                        paths.push(path);
                    }
                }
                println!("load file:{:?}",paths);
                let mut config_file_watch = self.config_file_watch.lock().expect("锁上失败!");
                for path in paths{
                    config_file_watch.insert(path.clone(), std::fs::File::open(path.clone())?.metadata()?.modified()?);
                }
                if (config_file_watch.len() as i32) >= WATCH_FILE_MAX {
                    return Err(Box::try_from( "over flow max watch file numbers!")?);
                }
            },
            FileType::Normal(d)=>{
                let mut paths:Vec<String> = vec![];
                for _dir in dir {
                    for path in  read_all_paths(_dir,file_type.clone())?{
                        paths.push(path);
                    }
                }
                println!("load file:{:?}",paths);
                let mut normal_file_watch = self.normal_file_watch.lock().expect("锁上失败!");
                for path in paths{
                    normal_file_watch.insert(path.clone(), std::fs::File::open(path.clone())?.metadata()?.modified()?);
                }
                if (normal_file_watch.len() as i32) >= WATCH_FILE_MAX {
                    return Err(Box::try_from( "over flow max watch file numbers!")?);
                }
            }
        };
        Ok(())
    }
    pub fn watch(&self)->Result<(),Box<dyn error::Error>>{
        {
            let mut yconf_c:MutexGuard<YConfig> = YCONF.lock()?;
            self.add_dir_watch(yconf_c.watchDir.clone(),yconf_c.hType.clone(),FileType::Normal("".to_string()))?;
        }
        loop{
            let mut config_file_watch = self.config_file_watch.lock().unwrap();
            for (path, time) in config_file_watch.iter() {
                let now_time = std::fs::File::open(path.clone())?.metadata()?.modified()?;
                if !time.eq(&now_time){
                    config_file_watch.insert(path.clone(),now_time);
                    println!("config change");
                }
            }

            let mut normal_file_watch = self.normal_file_watch.lock().unwrap();
            for (path, time) in normal_file_watch.iter() {
                let now_time = std::fs::File::open(path.clone())?.metadata()?.modified()?;
                if !time.eq(&now_time){
                    normal_file_watch.insert(path.clone(),now_time);
                    println!("normal_file change");
                }
            }
            sleep(Duration::from_millis(500));
            println!("once!");
        }
        Ok(())
    }
    pub fn load_config(&self,path:&'a str)->Result<(),Box<dyn error::Error>>{
        // 重置配置文件
        self.config_file_watch.lock().expect("锁失败!").clear();
        self.normal_file_watch.lock().expect("锁失败!").clear();
        // 读取配置
        println!("set config path is {}",path);
        let mut f = std::fs::File::open(path)?;
        let mut file_body = String::from("");
        f.read_to_string(&mut file_body)?;
        println!("load config:\r\n {}",file_body.to_string());
        let _yconf:YConfig = serde_json::from_str(file_body.as_str())?;
        let mut yconf_c:MutexGuard<YConfig> = YCONF.lock()?;
        (*yconf_c) = _yconf.clone();
        println!("watch unlock");
        {
            let mut common_c:MutexGuard<HashMap<String,Regex>> = COMMON.lock()?;
            (*common_c) = read_reg_file((*yconf_c).common.clone())?;
            let mut singal_c:MutexGuard<HashMap<String,Regex>> = SINGAL.lock()?;
            (*singal_c) = read_reg_file((*yconf_c).single.clone())?;
        }
        Ok(())
    }
}