pub mod my_router {
    use crate::config::config::SINGAL;
    use crate::log::log::LOGCH;
    use actix::clock::Duration;
    use actix::{Actor, AsyncContext, StreamHandler};
    use actix_web::web::Payload;
    use actix_web::{Error, HttpRequest, HttpResponse};
    use actix_web_actors::ws;
    use actix_web_actors::ws::{Message, ProtocolError};
    use regex::Regex;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::Read;
    use std::ops::Add;
    use std::sync::MutexGuard;

    pub async fn syncjs(_req: HttpRequest) -> Result<HttpResponse, Error> {
        let mut file = File::open("./res/regexp/js/sync.js").expect("没有读取到文件");
        let mut buf = vec![];
        let _ = file.read_to_end(&mut buf).expect("读取错误");
        let singal_c: MutexGuard<HashMap<String, Regex>> = SINGAL.lock().unwrap();
        let mut will_insert_regs = "".to_string();
        for (value, reg) in singal_c.iter() {
            will_insert_regs =
                will_insert_regs.add(format!("   this.regexps.push{}\r\n", "({").as_ref());
            will_insert_regs = will_insert_regs
                .add(format!("       rp:new RegExp(/{}/),\r\n", reg.as_str()).as_ref());
            will_insert_regs = will_insert_regs
                .add(format!("       rep:'{}',\r\n", value.replace("\n", "")).as_ref());
            will_insert_regs = will_insert_regs.add(format!("   {}\r\n", "})").as_ref());
        }
        let buf_str = String::from_utf8(buf)
            .unwrap()
            .replace("//insertHere", will_insert_regs.as_str());
        Ok(HttpResponse::Ok().body(buf_str))
    }

    pub async fn test_html(_req: HttpRequest) -> Result<HttpResponse, Error> {
        Ok(HttpResponse::Found()
            .header("LOCATION", "./res/sample/js/test.html")
            .finish())
    }
    pub async fn main_html(_req: HttpRequest) -> Result<HttpResponse, Error> {
        Ok(HttpResponse::Found()
            .header("LOCATION", "./res/sample/js/main.html")
            .finish())
    }
    pub async fn get_config(_req: HttpRequest) -> Result<HttpResponse, Error> {
        let mut f = std::fs::File::open("./res/config/config.json")?;
        let mut file_body = String::from("");
        f.read_to_string(&mut file_body)?;
        Ok(HttpResponse::Ok()
            .header("Content-Type", "application/json")
            .body(file_body))
    }
    // 打印日志的websocket
    struct WSLog;
    impl Actor for WSLog {
        type Context = ws::WebsocketContext<Self>;
        fn started(&mut self, ctx: &mut Self::Context) {
            ctx.run_interval(Duration::from_millis(10), |_act, ctx| {
                let mut log_ch = LOGCH.lock().unwrap();
                let log_data = log_ch.receive();
                if !log_data.is_empty() {
                    ctx.text(log_data)
                }
            });
        }
    }
    // 处理接收到的数据
    impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WSLog {
        fn handle(&mut self, msg: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
            match msg {
                Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
                Ok(ws::Message::Text(text)) => ctx.text(text),
                Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
                _ => (),
            }
        }
    }

    pub async fn log(req: HttpRequest, stream: Payload) -> Result<HttpResponse, Error> {
        let resp = ws::start(WSLog {}, &req, stream);
        resp
    }
}
