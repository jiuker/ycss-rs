extern crate ycss;
use ycss::config::config;
use std::thread::sleep;
use std::time::Duration;
use ycss::repl::repl::Repl;
use ycss::repl::vue::VueRepl;
use std::convert::TryFrom;

fn main() {
    match config::set_config_path("./res/config".to_owned(),file_change){
        Ok(_)=>(),
        Err(e)=>{
            println!("some err is {}",e)
        }
    };
    sleep(Duration::from_secs(24*60*60))
}
fn file_change(path:String){
    println!("path is{}",path);
    if path.contains("./res/config/config.json"){
        match config::load_config(path,file_change) {
            Ok(())=>println!("set config watch ok!"),
            Err(e)=>println!("err is {:?}",e)
        }
    }else{
        // 不是配置文件变动
        println!("get {} changed!",path);
        let mut rep:VueRepl = Repl::new(path.to_owned());
        match rep.init().and_then(
            |_| rep.get_class().and_then(
                |cls| rep.get_new_css(cls).and_then(
                    |new_css| rep.get_old_css().and_then(
                        |old_css| {
                            if old_css==""{
                                println!("not find the auto css contain!forget?[{}]",path);
                                return Err(Box::try_from(format!("not find the auto css contain!forget?[{}]",path))?);
                            }
                            if !rep.is_same(new_css.clone(),old_css.clone()){
                                rep.write(new_css.clone(),old_css.clone())?;
                            }else{
                                println!("is the same!do noting!");
                            };
                            Ok(())
                        }
                    )
                )
            )
        ){
            Ok(d)=>{},
            Err(e)=>{
                println!("e is {}",e);
            }
        };
    }
}