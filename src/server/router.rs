pub mod my_router {
    use actix_web::{HttpRequest, HttpResponse, Error};
    use std::fs::File;
    use std::io::Read;
    

    pub async fn syncjs(req: HttpRequest) -> Result<HttpResponse, Error> {
        println!("REQ: {:?}", req);
        let mut file = File::open("./res/regexp/js/sync.js").expect("没有读取到文件");
        let mut buf = vec![];
        let _ = file.read_to_end(&mut buf).expect("读取错误");
        println!("{}",buf.len());
        Ok(HttpResponse::Ok().body(buf))
    }
}