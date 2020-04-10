use std::collections::HashMap;
use std::time::{SystemTime, Duration};
use std::thread::{sleep, spawn};
use lazy_static::lazy_static;
const WATCH_FILE_MAX:i32 = 1000i32;
use std::sync::{Mutex, Arc};
use std::ops::Add;
extern crate regex;
use std::convert::TryFrom;
use std::error;
lazy_static!{
     static ref WATCH_MAP:Arc<Mutex<HashMap<String,SystemTime>>> = Arc::new(Mutex::new(HashMap::new()));
}
pub fn watch_dir(dir:Vec<String>,file_type:String,clear:bool,cb:fn(path:String))->Result<(),Box<dyn error::Error>>{
    println!("file_type is {} dir is {1:?}",file_type,dir);
    let mut paths:Vec<String> = vec![];
    for _dir in dir {
        for path in  read_all_paths(_dir,file_type.clone())?{
            paths.push(path);
        }
    }
    println!("load file:{:?}",paths);
    {
        let mut watch_map = WATCH_MAP.lock()?;
        if clear{
            watch_map.clear()
        }
        for path in paths{
            watch_map.insert(path.clone(), std::fs::File::open(path.clone())?.metadata()?.modified()?);
        }
        if (watch_map.len() as i32) >= WATCH_FILE_MAX {
            return Err(Box::try_from( "over flow max watch file numbers!").unwrap());
        }
    }
    // 只有第一次设置监听才有用
    if !clear{
        spawn(move || {
            loop {
                {
                    // 出了作用域需要解锁,不然这里就是一直持有锁
                    let mut watch_map_c = WATCH_MAP.lock().unwrap();
                    for (path, time) in watch_map_c.clone() {
                        let now_time = std::fs::File::open(path.clone()).unwrap().metadata().unwrap().modified().unwrap();
                        if !time.eq(&now_time){
                            watch_map_c.insert(path.clone(),now_time);
                            cb(path.clone());
                        }
                    }
                }
                sleep(Duration::from_millis(500));
            }
        });
    }
    return Ok(());
}
fn read_all_paths(dir:String,file_type:String)->Result<Vec<String>,Box<dyn error::Error>>{
    let dirs = std::fs::read_dir(dir)?;
    let mut paths:Vec<String> = vec![];
    let file_type_more= String::from(".").add(file_type.clone().as_str()).add("$");
    let reg = regex::Regex::new(file_type_more.as_str())?;
    for x in dirs {
        let x_u = x?;
        let x_meta = x_u.metadata()?;
        if x_meta.is_dir(){
            for path in  read_all_paths(x_u.path().to_str().unwrap().to_owned(),file_type.clone())?{
                paths.push(path);
            }
        }else{
            let x_path = x_u.path();
            if reg.is_match(x_path.to_str().unwrap()){
                paths.push(x_path.to_str().unwrap().to_owned());
            }
        }
    }
    Ok(paths)
}