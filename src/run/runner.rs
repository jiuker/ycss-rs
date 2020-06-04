use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{SystemTime, Duration};
use std::collections::HashMap;
use std::{error, result};
use crate::watch::watch::read_all_paths;
use std::convert::TryFrom;
use crate::config::config::{read_reg_file};
use regex::Regex;
use std::io::Read;
use crate::config::config::{YCONF, COMMON, SINGAL, YConfig};
use std::thread::{sleep};
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};

#[derive(Debug,Clone)]
pub struct Runner<'a>{
    config_path:&'a str,
    config_file_watch:Arc<Mutex<HashMap<String,SystemTime>>>,
    normal_file_watch:Arc<Mutex<HashMap<String,SystemTime>>>,
    sender:SyncSender<FileType>,
    pub receiver:Arc<Mutex<Receiver<FileType>>>,
}
pub enum FileType{
    Config(String),
    Normal(String)
}
const WATCH_FILE_MAX:i32=10000;
pub type Result<T> = result::Result<T,Box<dyn error::Error>>;
impl <'a>Runner<'a>{
    pub fn new(path:&'a str)->Self{
        let (sender,receiver)  = sync_channel::<FileType>(10);
        let run = Runner{
            config_path: path,
            config_file_watch: Arc::new(Mutex::new(Default::default())),
            normal_file_watch: Arc::new(Mutex::new(Default::default())),
            sender,
            receiver:Arc::new(Mutex::new(receiver)),
        };
        run.load_config(path).unwrap();
        run
    }
    pub fn add_dir_watch(&self,dir:Vec<String>,file_type:String,typ:FileType)->Result<()>{
        match typ {
            FileType::Config(_d)=>{
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
            FileType::Normal(_d)=>{
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
    pub fn watch(&self)->Result<()>{
        loop{
            {
                let mut config_file_watch = self.config_file_watch.lock().unwrap();
                for (path, time) in config_file_watch.iter_mut() {
                    let now_time = std::fs::File::open(path.clone())?.metadata()?.modified()?;
                    if !(*time).eq(&now_time){
                        *time = now_time;
                        self.sender.send(FileType::Config(path.clone()))?;
                    }
                }

                let mut normal_file_watch = self.normal_file_watch.lock().unwrap();
                for (path, time) in normal_file_watch.iter_mut() {
                    let now_time = std::fs::File::open(path.clone())?.metadata()?.modified()?;
                    if !(*time).eq(&now_time){
                        *time = now_time;
                        self.sender.send(FileType::Normal(path.clone()))?;
                    }
                }
            }
            sleep(Duration::from_millis(500));
        }
    }
    pub fn load_config(&self,path:&'a str)->Result<()>{
        // 重置配置文件
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
        let mut common_c:MutexGuard<HashMap<String,Regex>> = COMMON.lock()?;
        (*common_c) = read_reg_file((*yconf_c).common.clone())?;
        let mut singal_c:MutexGuard<HashMap<String,Regex>> = SINGAL.lock()?;
        (*singal_c) = read_reg_file((*yconf_c).single.clone())?;
        // 添加文件监听
        self.add_dir_watch(yconf_c.watch_dir.clone(), yconf_c.h_type.clone(), FileType::Normal("".to_string()))?;
        Ok(())
    }
}