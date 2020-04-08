use std::collections::HashMap;
use std::time::{SystemTime, Duration};
use std::io::{Error, ErrorKind};
use std::thread::sleep;

const WATCH_FILE_MAX:i32 = 1000i32;

pub fn watch_dir(dir:Vec<String>,file_type:String,cb:fn(path:String))->Result<(),Error>{
    println!("file_type is {}",file_type);
    let mut paths:Vec<String> = vec![];
    for _dir in dir {
        for path in  read_all_paths(_dir,file_type.clone()).unwrap(){
            paths.push(path);
        }
    }
    let mut watch_map:HashMap<String,SystemTime> = HashMap::new();
    for path in paths{
        watch_map.insert(path.clone(), std::fs::File::open(path.clone()).unwrap().metadata().unwrap().modified().unwrap());
    }
    if (watch_map.len() as i32) >= WATCH_FILE_MAX {
        return Err(Error::new(ErrorKind::Other,"over flow max watch file numbers!"));
    }
    println!("init file watcher:{:?}",watch_map);
    loop {
        if watch_map.clone().len()==0{
            break
        }
        sleep(Duration::from_millis(500));
        for (path, time) in watch_map.clone() {
            let now_time = std::fs::File::open(path.clone()).unwrap().metadata().unwrap().modified().unwrap();
            if !time.eq(&now_time){
                watch_map.insert(path.clone(),now_time);
                cb(path.clone());
            }
        }
    }
    Ok(())
}
fn read_all_paths(dir:String,file_type:String)->Result<Vec<String>,Error>{
    let dirs = std::fs::read_dir(dir).unwrap();
    let mut paths:Vec<String> = vec![];
    for x in dirs {
        let x_u = x.unwrap();
        let x_meta = x_u.metadata().unwrap();
        if x_meta.is_dir(){
            for path in  read_all_paths(x_u.path().to_str().unwrap().to_owned(),file_type.clone()).unwrap(){
                paths.push(path);
            }
        }else{
            if x_u.path().to_str().unwrap().to_owned().contains(&file_type){
                paths.push(x_u.path().to_str().unwrap().to_owned());
            }
        }
    }
    Ok(paths)
}