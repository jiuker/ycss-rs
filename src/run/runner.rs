use crate::config::config::read_reg_file;
use crate::config::config::{YConfig, COMMON, SINGAL, YCONF};
use crate::log::log::LOGCH;
use crate::watch::watch::read_all_paths;
use crate::web_log;
use regex::Regex;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::Read;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use std::{error, result};

#[derive(Debug, Clone)]
pub struct Runner<'a> {
    config_path: &'a str,
    config_file_watch: Arc<Mutex<HashMap<String, SystemTime>>>,
    normal_file_watch: Arc<Mutex<HashMap<String, SystemTime>>>,
    sender: SyncSender<FileType>,
    pub receiver: Arc<Mutex<Receiver<FileType>>>,
}
pub enum FileType {
    Config(String),
    Normal(String),
}
macro_rules! add_dir_watch {
    ($file_watch:expr,$dir:ident,$file_type:ident) => {
        let mut paths: Vec<String> = vec![];
        for _dir in $dir {
            for path in read_all_paths(_dir, $file_type.clone())? {
                paths.push(path);
            }
        }
        web_log!("load file:{:?}", paths);
        let mut file_watch = $file_watch.lock().expect("锁上失败!");
        for path in paths {
            file_watch.insert(
                path.clone(),
                std::fs::File::open(path.clone())?.metadata()?.modified()?,
            );
        }
        if (file_watch.len() as i32) >= WATCH_FILE_MAX {
            return Err(Box::try_from("over flow max watch file numbers!")?);
        }
    };
}

macro_rules! check_file_change {
    ($file_watch:expr,$sender:expr,$file_type:expr) => {
        let mut file_watch = $file_watch.lock().unwrap();
        for (path, time) in file_watch.iter_mut() {
            let now_time = std::fs::File::open(path.clone())?.metadata()?.modified()?;
            if !(*time).eq(&now_time) {
                *time = now_time;
                $sender.send($file_type(path.clone()))?;
            }
        }
    };
}

const WATCH_FILE_MAX: i32 = 10000;
pub type Result<T> = result::Result<T, Box<dyn error::Error>>;
impl<'a> Runner<'a> {
    pub fn new(path: &'a str) -> Self {
        let (sender, receiver) = sync_channel::<FileType>(10);
        let run = Runner {
            config_path: path,
            config_file_watch: Arc::new(Mutex::new(Default::default())),
            normal_file_watch: Arc::new(Mutex::new(Default::default())),
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
        };
        run.load_config(path).unwrap();
        run
    }
    pub fn add_dir_watch(
        &self,
        dir: &Vec<String>,
        file_type: &String,
        typ: FileType,
    ) -> Result<()> {
        match typ {
            FileType::Config(_d) => {
                add_dir_watch!(self.config_file_watch, dir, file_type);
            }
            FileType::Normal(_d) => {
                add_dir_watch!(self.normal_file_watch, dir, file_type);
            }
        };
        Ok(())
    }
    pub fn watch(&self) -> Result<()> {
        loop {
            {
                check_file_change!(self.config_file_watch, self.sender, FileType::Config);
                check_file_change!(self.normal_file_watch, self.sender, FileType::Normal);
            }
            sleep(Duration::from_millis(500));
        }
    }
    pub fn load_config(&self, path: &'a str) -> Result<()> {
        // 重置配置文件
        self.config_file_watch.lock().expect("锁失败!").clear();
        // 读取配置
        web_log!("set config path is {}", path);
        let mut f = std::fs::File::open(path)?;
        let mut file_body = String::from("");
        f.read_to_string(&mut file_body)?;
        web_log!("load config:\r\n {}", file_body.to_string());
        let _yconf: YConfig = serde_json::from_str(file_body.as_str())?;
        let mut yconf_c = YCONF.lock()?;
        (*yconf_c) = _yconf.clone();
        let mut common_c = COMMON.lock()?;
        (*common_c) = read_reg_file((*yconf_c).common.clone())?;
        let mut singal_c = SINGAL.lock()?;
        (*singal_c) = read_reg_file((*yconf_c).single.clone())?;
        // 添加文件监听
        self.add_dir_watch(
            &yconf_c.watch_dir,
            &yconf_c.h_type,
            FileType::Normal("".to_string()),
        )?;
        Ok(())
    }
}
