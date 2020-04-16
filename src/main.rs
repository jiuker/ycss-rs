extern crate ycss;
use ycss::config::config;
use std::thread::sleep;
use std::time::Duration;
use ycss::repl::repl::Repl;
use ycss::repl::vue::VueRepl;

fn main() {
    config::set_config_path("./res/config".to_owned(),file_change);
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
        let rep:VueRepl = Repl::new(path.clone());
        let cls = match rep.get_class(){
            Ok(d)=>d,
            Err(e)=>{
                println!("get class err is {}",e);
                return
            }
        };
        println!("cls is {:?}",cls);
        let new_css = match rep.get_new_css(cls){
            Ok(d)=>d,
            Err(e)=>{
                println!("get new css err is {}",e);
                return;
            }
        };
        // println!("new_css is {}",new_css);
        let old_css = rep.get_old_css();
        // println!("old_css is {}",old_css);
        if old_css==""{
            println!("not find the auto css contain!forget?[{}]",path);
            return;
        }
        if !rep.is_same(new_css.clone(),old_css.clone()){
            rep.write(new_css.clone(),old_css.clone());
            println!("replace success!");
        }else{
            println!("is the same!do noting!");
        }
    }
}