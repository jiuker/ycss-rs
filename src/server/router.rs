pub mod my_router {
    use actix_web::{HttpRequest, HttpResponse, Error};
    use std::fs::File;
    use std::io::Read;
    use regex::Regex;
    use std::collections::HashMap;
    use std::sync::MutexGuard;
    use crate::config::config::{YCONF, COMMON, SINGAL, YConfig};
    use std::ops::Add;

    pub async fn syncjs(req: HttpRequest) -> Result<HttpResponse, Error> {
        let mut file = File::open("./res/regexp/js/sync.js").expect("没有读取到文件");
        let mut buf = vec![];
        let _ = file.read_to_end(&mut buf).expect("读取错误");
        let mut common_c:MutexGuard<HashMap<String,Regex>> = COMMON.lock().unwrap();
        let mut singal_c:MutexGuard<HashMap<String,Regex>> = SINGAL.lock().unwrap();
        let mut will_insert_regs = "".to_string();
        for (value,reg) in singal_c.clone(){
            will_insert_regs = will_insert_regs.add(format!("this.regexps.push").as_ref())
        }
        println!("{}",will_insert_regs);
        Ok(HttpResponse::Ok().body(buf))
    }
}