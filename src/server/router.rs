pub mod my_router {
    use actix_web::{HttpRequest, HttpResponse, Error};
    use std::fs::File;
    use std::io::Read;
    use regex::Regex;
    use std::collections::HashMap;
    use std::sync::MutexGuard;
    use crate::config::config::{SINGAL};
    use std::ops::Add;

    pub async fn syncjs(_req: HttpRequest) -> Result<HttpResponse, Error> {
        let mut file = File::open("./res/regexp/js/sync.js").expect("没有读取到文件");
        let mut buf = vec![];
        let _ = file.read_to_end(&mut buf).expect("读取错误");
        let singal_c:MutexGuard<HashMap<String,Regex>> = SINGAL.lock().unwrap();
        let mut will_insert_regs = "".to_string();
        for (value,reg) in singal_c.clone(){
            will_insert_regs = will_insert_regs.add(format!("   this.regexps.push{}\r\n","({").as_ref());
            will_insert_regs = will_insert_regs.add(format!("       rp:new RegExp(/{}/),\r\n",reg.as_str()).as_ref());
            will_insert_regs = will_insert_regs.add(format!("       rep:'{}',\r\n",value.replace("\n","")).as_ref());
            will_insert_regs = will_insert_regs.add(format!("   {}\r\n","})").as_ref());
        }
        let buf_str = String::from_utf8(buf).unwrap().replace("//insertHere", will_insert_regs.as_str());
        Ok(HttpResponse::Ok().body(buf_str))
    }

    pub async fn test_html(_req: HttpRequest) -> Result<HttpResponse, Error> {
        Ok(HttpResponse::Found().header("LOCATION","./res/sample/js/test.html").finish())
    }
    pub async fn main_html(_req: HttpRequest) -> Result<HttpResponse, Error> {
        Ok(HttpResponse::Found().header("LOCATION","./res/sample/js/main.html").finish())
    }
    pub async fn get_config(_req: HttpRequest) -> Result<HttpResponse, Error> {
        let mut f = std::fs::File::open("./res/config/config.json")?;
        let mut file_body = String::from("");
        f.read_to_string(&mut file_body)?;
        Ok(HttpResponse::Ok().header("Content-Type","application/json").body(file_body))
    }
}
