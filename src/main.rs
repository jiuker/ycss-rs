extern crate ycss_rs;

use std::thread::{spawn};

use ycss_rs::repl::repl::Repl;
use ycss_rs::repl::vue::VueRepl;
use std::convert::TryFrom;
use ycss_rs::run::runner::{Runner, FileType};

fn main() {
    println!(r#"ycss-rs start
            go....
                go...
                    go...
    "#);
    let run = Runner::new("./res/config/config.json");
    run.add_dir_watch(vec!["./res/config".to_string()],".json".to_string(),FileType::Config("".to_string())).unwrap();
    let run_c = run.clone();
    spawn(move||{
        run_c.watch().unwrap();
    });
    loop{
        match run.receiver.lock().unwrap().recv().unwrap(){
            FileType::Config(path)=>{
                run.load_config(path.as_str()).unwrap()
            },
            FileType::Normal(path)=>{
                // 不是配置文件变动
                println!("get {} changed!",path);
                let mut rep:VueRepl = Repl::new(path.to_owned());
                match rep.init().and_then(
                    |_| rep.get_class().and_then(
                        |cls| rep.get_new_css(cls).and_then(
                            |new_css| rep.get_old_css().and_then(
                                |old_css| {
                                    if old_css==""{
                                        return Err(Box::try_from(format!("not find the auto css contain!forget?[{}]",path))?);
                                    }
                                    if !rep.is_same(new_css.clone(),old_css.clone()){
                                        rep.write(new_css.clone(),old_css.clone())?;
                                    }else{
                                        return Err(Box::try_from("is the same! do nothing！")?);
                                    };
                                    Ok(())
                                }
                            )
                        )
                    )
                ){
                    Ok(_d)=>{
                        println!("handle file success done!");
                    },
                    Err(e)=>{
                        println!("{}",e);
                    }
                };
            }
        }
    }
}
