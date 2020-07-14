use actix_web::{middleware, web, App, Error as AWError, HttpResponse, HttpServer};

extern crate ycss_rs;
use actix_files as fs;
use actix_web::middleware::Logger;

use std::env::set_var;
use std::sync::mpsc::channel;
use std::thread::spawn;
use ycss_rs::repl::repl::Repl;
use ycss_rs::repl::vue::VueRepl;
use ycss_rs::run::runner::{FileType, Runner};
use ycss_rs::server::router::my_router;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    spawn(move || handle());
    set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(web::resource("/res/regexp/js/sync.js").to(my_router::syncjs))
            .service(web::resource("/test").to(my_router::test_html))
            .service(web::resource("/").to(my_router::main_html))
            .service(fs::Files::new("/res/", "res/"))
            .route("/api/get_config", web::get().to(my_router::get_config))
            .route("/ws/log", web::get().to(my_router::log))
    })
    .bind("127.0.0.1:5060")?
    .run()
    .await
}
fn handle() {
    println!(
        "ycss-rs start
            go....
                go...
                    go...
    "
    );
    let run = Runner::new("./res/config/config.json");
    run.add_dir_watch(
        vec!["./res/config".to_string()],
        ".json".to_string(),
        FileType::Config("".to_string()),
    )
    .unwrap();
    let run_c = run.clone();
    spawn(move || {
        run_c.watch().unwrap();
    });
    while let Ok(file) = run.receiver.lock().unwrap().recv() {
        match file {
            FileType::Config(path) => run.load_config(path.as_str()).unwrap(),
            FileType::Normal(path) => {
                // 不是配置文件变动
                println!("get {:>25} changed!", path);
                let mut rep: VueRepl = Repl::new(path.to_owned());
                match rep.init().and_then(|_| {
                    rep.get_class().and_then(|cls| {
                        rep.get_new_css(cls).and_then(|new_css| {
                            rep.get_old_css().and_then(|old_css| {
                                if old_css == "" {
                                    return Err(Box::from(format!(
                                        "not find the auto css contain!forget?[{}]",
                                        path
                                    )));
                                }
                                if !rep.is_same(new_css.clone(), old_css.clone()) {
                                    rep.write(new_css.clone(), old_css.clone())?;
                                } else {
                                    return Err(Box::from("is the same! do nothing！"));
                                };
                                Ok(())
                            })
                        })
                    })
                }) {
                    Ok(_d) => {
                        println!("handle file success done!");
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                };
            }
        }
    }
}
