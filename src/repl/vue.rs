use crate::repl::repl::Repl;
use crate::config::config::{YCONF, COMMON, SINGAL};
use std::io::{Read, Write};
use regex::{Regex, Captures};
use std::ops::Add;
use std::collections::{VecDeque, HashMap};
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::error;
use std::convert::TryFrom;
pub struct VueRepl{
    path:String,
    file_body:String,
    out_path:String,
}
impl Repl for VueRepl {
    fn new(path:String) -> VueRepl {
        VueRepl {
            path,
            file_body:"".to_string(),
            out_path: "".to_string(),
        }
    }
    fn init(&mut self)->Result<(),Box<dyn error::Error>>{
        let yconf_c = YCONF.lock()?;
        let mut file = std::fs::File::open(&self.path)?;
        let mut file_body = String::from("");
        file.read_to_string(&mut file_body)?;
        (*self).file_body = file_body;
        (*self).out_path = match parse_out_path(self.path.clone(),yconf_c.clone().outPath){
            Some(d)=>d,
            None => "@FileDir@FileName@FileType".to_string()
        };
        Ok(())
    }
    fn get_file_body(&self) -> String {
        self.file_body.clone()
    }
    fn get_class(&self)->Result<Vec<String>,Box<dyn error::Error>>{
        let yconf_c = YCONF.lock()?;
        let file_body = (*self).get_file_body();
        let mut rsl_str = String::from("");
        let mut rsl: Vec<String> = vec![];
        for reg in &yconf_c.reg{
            let reg_reg = Regex::new(reg)?;
            let rsl_ = reg_reg.find_iter(file_body.as_str()).map(|x| x.as_str().to_string()).collect::<Vec<_>>();
            for rsl__ in rsl_{
                let mut index = 0;
                for rsl___ in rsl__.split("\""){
                    if index == 1{
                        rsl_str = rsl_str.add(" ").add(rsl___);
                    }
                    index = index + 1;
                }
            }
        }
        let mut cls_map:VecDeque<String> = VecDeque::new();
        for rsl_str_split in rsl_str.split(" "){
            if rsl_str_split!=""{
                cls_map.insert(cls_map.len(),String::from(rsl_str_split));
            }
        }
        for key in cls_map.iter(){
            rsl.push(key.to_owned());
        }
        Ok(rsl)
    }

    fn get_new_css(&self, cls:Vec<String>) -> Result<String,Box<dyn error::Error>> {
        let common_c = COMMON.lock()?;
        let singal_c = SINGAL.lock()?;
        let mut rsl = String::new() ;
        for cls_ in cls{
            for (value,reg) in common_c.clone(){
                if reg.is_match(&cls_.as_str()){
                    let class_match = match reg.captures(cls_.as_str()){
                        Some(d)=>d,
                        None=>{
                            return Err(Box::try_from("没有匹配数据!")?)
                        }
                    };
                    let mut value_c:String = value;
                    for match_index in 0..class_match.len() {
                        if !value_c.contains("$"){
                            break;
                        }
                        value_c = value_c.replace(format!("${}",match_index).as_str(),&class_match[match_index]);
                    }
                    // get the common replace value:bb-1-fff \n c-1-fff
                    // println!("{:?}",value_c);
                    let mut css_content = String::from("");
                    for value_c_split in value_c.split("\n"){
                        let value_c_split_trim = value_c_split.trim().to_string();
                        for (sv,sr) in singal_c.clone(){
                            let mut sv_c = sv;
                            let value_c1 = &value_c_split_trim;
                            if sr.is_match(value_c1.as_str()){
                                let sr_match =match sr.captures(value_c1.as_str()){
                                    Some(d)=>d,
                                    None=>{
                                        return Err(Box::try_from("没有匹配数据!")?)
                                    }
                                };
                                for mr_index in 0..sr_match.len() {
                                    if !sv_c.contains("$"){
                                        break;
                                    }
                                    sv_c = sv_c.replace(format!("${}",mr_index).as_str(),&sr_match[mr_index])
                                }
                                // println!("{:?}",sv_c);
                                // set as css
                                if !css_content.is_empty(){
                                    sv_c = sv_c.add("\r\n");
                                }
                                css_content = css_content.add(sv_c.trim());
                            }
                        }
                    }
                    let rsl_string = format!(".{}{}{}{}", cls_.as_str(), "{", css_content.as_str(),"}\r\n");
                    // println!("{:?}",rsl_string.as_str());
                    rsl = rsl.add(rsl_string.as_str());
                    break
                }
            }
        }
        rsl = format!("/* Automatic generation Start */\r\n{}\r\n/*",rsl);
        // 缩放
        let yconf_c = YCONF.lock()?;
        let out_unit = &yconf_c.outUnit;
        let zoom_size = &yconf_c.zoom;
        let need_zoom_uint_str = format!("([0-9|\\.]{})[ |	]{}({}){}","{1,10}","{0,3}",yconf_c.needZoomUnit,"{1,5}");
        let reg_need_zoom = Regex::new(need_zoom_uint_str.as_str())?;
        let rsl_ = reg_need_zoom.replace_all(rsl.as_str(), |caps:&Captures| -> String {
            let base = match caps[1].parse::<f32>(){
                Ok(d)=>d,
                Err(_)=>{
                    return caps[0].to_string();
                }
            };
            let data = zoom_size*base;
            format!("{}{}",data,out_unit)
        });
        // 如果不是自己的文件需要追加地址
        let mut rsl__:String = rsl_.parse()?;
        if !self.out_path.eq(&self.path) {
            // 地址不一样
            rsl__ = rsl__.add(format!(" {}", self.path).as_ref());
        }
        println!("go here");
        // replace static
        let static_map = &yconf_c.static_map;
        for (key,value) in static_map {
            println!("key is {},value is {}",key,value);
            rsl__ = rsl__.replace(format!("@{}{}{}","{",key,"}").as_str(),value.as_str());
        }
        Ok(rsl__)
    }
    fn get_old_css(&self) ->Result<String,Box<dyn error::Error>>{
        let yconf_c = YCONF.lock()?;
        let file_body = self.get_file_body();
        let mut rsl = String::from("");
        // 路径是一致的
        if self.out_path.eq(&self.path){
            let reg_reg = Regex::new(&yconf_c.oldCssReg.as_str())?;
            let rsl_ = match  reg_reg.find(file_body.as_str()) {
                Some(d)=>d,
                None =>{
                    return Err(Box::try_from("没有匹配到数据")?);
                }
            };
            rsl = rsl.add(rsl_.as_str());
        }else{
            // 路径不一致
            let out_path = &self.out_path;
            let mut file_body = "".to_string();
            println!("old file is {}",out_path);
            OpenOptions::new().read(true).write(true).open(out_path)?.read_to_string(&mut file_body)?;
            let mut old_css_reg_c = yconf_c.oldCssReg.clone() as String;
            old_css_reg_c = old_css_reg_c.add(format!(" {}", self.path).as_ref());
            println!("old css reg is {}",old_css_reg_c);
            let reg_reg = Regex::new(old_css_reg_c.as_str())?;
            rsl = match reg_reg.find(file_body.as_str()) {
                None => "".to_string(),
                Some(d)=> rsl.add(d.as_str())
            }
        }
        Ok(rsl)
    }

    fn is_same(&self,a: String, b: String) -> bool {
        let mut rsl = true;
        for s in match same_str(){
            Ok(d)=>d,
            Err(e)=>{
                println!("compare err is {}",e);
                return false
            }
        }.bytes(){
           let s_:char = char::from(s);
           let a_ = a.chars().filter(|x| *x==s_).collect::<Vec<_>>().len();
           let b_ = b.chars().filter(|x| *x==s_).collect::<Vec<_>>().len();
           if a_!=b_{
               println!("not the same char is {} a:{} b:{}",s_,a_,b_);
               rsl = false;
               break
           }

        };
        rsl
    }

    fn write(&self,new_css:String,old_css:String)->Result<(),Box<dyn error::Error>>{
        // 如果不是自己的文件需要追加地址
        let out_path = self.out_path.clone();
        if !out_path.eq(&self.path) {
            let mut file_body = "".to_string();
            println!("old file is {}",out_path);
            {
                let mut file = OpenOptions::new().read(true).write(true).open(out_path.clone())?;
                file.read_to_string(&mut file_body)?;
            }
            let mut file = File::create(out_path)?;
            let will_write = file_body.replace(old_css.as_str(),new_css.as_str());
            // println!("will_write:{}",will_write);
            file.write(will_write.as_bytes())?;
        }else{
            let file_body = self.get_file_body();
            let will_write = file_body.replace(old_css.as_str(),new_css.as_str());
            let mut file = File::create(&self.path)?;
            // println!("will_write:{}",will_write);
            file.write(will_write.as_bytes())?;
        }
        Ok(())
    }
}
pub fn same_str() ->Result<String,Box<dyn error::Error>>{
     let rsl = match "qazwsxedcrfvtgbnhyujmki,ol.;p'[]1234567890-".parse(){
         Ok(d)=>d,
         Err(_)=>{
             return Err(Box::try_from("字符串解析异常")?);
         }
     };
     Ok(rsl)
}
// 解析输出路径，以下是全路径
// @FileDir@FileName@FileType
pub fn parse_out_path(file_path:String,out_path:String)->Option<String>{
    let file_path_c = Path::new(&file_path);
    let mut rsl = out_path.clone();
    let file_dir = file_path.replace(file_path_c.file_name()?.to_str()?,"");
    let file_type = file_path_c.file_name()?.to_str()?.replace(file_path_c.file_stem()?.to_str()?,"");
    let file_name = file_path_c.file_name()?.to_str()?.replace(file_type.as_str(),"");
    rsl = rsl.replace("@FileDir",file_dir.as_str());
    rsl = rsl.replace("@FileName",file_name.as_str());
    rsl = rsl.replace("@FileType",file_type.as_str());
    Some(rsl)
}