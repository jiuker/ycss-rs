use crate::repl::repl::Repl;
use crate::config::config::{YCONF, COMMON, SINGAL, YConfig};
use std::io::{Read, Write};
use regex::{Regex, Captures};
use std::ops::Add;
use std::collections::{HashSet, HashMap};
use std::fs::{File, OpenOptions};
use std::path::Path;
use crate::run::runner::Result;
use std::convert::TryFrom;
use std::sync::MutexGuard;

pub struct VueRepl{
    path:String,
    file_body:String,
    out_path:String,
    page_common:HashMap<String,Regex>
}
impl Repl for VueRepl {
    fn new(path:String) -> VueRepl {
        VueRepl {
            path,
            file_body:"".to_string(),
            out_path: "".to_string(),
            page_common: Default::default()
        }
    }
    fn init(&mut self)->Result<()>{
        let yconf_c:MutexGuard<YConfig> = YCONF.lock()?;
        let mut file = std::fs::File::open(&self.path)?;
        let mut file_body = String::from("");
        file.read_to_string(&mut file_body)?;
        // 赋值文件的body
        (*self).file_body = file_body.clone();
        // 识别输出路径
        (*self).out_path = match parse_out_path(self.path.clone(),yconf_c.clone().out_path){
            Some(d)=>d,
            None => "@FileDir@FileName@FileType".to_string()
        };
        // 查询页面级别的公共样式
        let mut page_common_str = "".to_string();
        for page_common in &yconf_c.page_common{
            let page_common_reg = Regex::new(page_common)?;
            let rsl_ = page_common_reg.find_iter(file_body.as_str()).map(|x| x.as_str().to_string()).collect::<Vec<_>>();
            for rsl__ in rsl_{
                let mut index = 0;
                for rsl___ in rsl__.split("\""){
                    if index == 1{
                        page_common_str=page_common_str+rsl___;
                    }
                    index = index + 1;
                }
            }
        };
        // dbg!(page_common_str);
        let mut page_common_vec = vec![];
        for  rsl_ in page_common_str.split("<"){
            for mut rsl__ in rsl_.split(">"){
                rsl__ = rsl__.trim();
                rsl__ = rsl__.trim_end();
                if rsl__!=""{
                    page_common_vec.push(String::from(rsl__));
                }
            }
        }
        if page_common_vec.len()%2!=0{
            return Err(Box::try_from("页面级别公共样式不正确!")?);
        }
        let mut common_keys:Vec<String> = vec![];
        let mut common_values:Vec<String> = vec![];
        let mut index = 0;
        while  index<page_common_vec.len(){
            if index%2==0{
                common_keys.push("^".to_string()+ page_common_vec[index].clone().as_ref() + "$".to_string().as_ref())
            }else{
                common_values.push(page_common_vec[index].clone())
            }
            index = index + 1;
        }
        index = 0;
        while  index<common_values.len(){
            self.page_common.insert(common_values[index].clone(),Regex::new(common_keys[index].as_str())?);
            index = index + 1;
        }
        // dbg!(common_values);
        // dbg!(self.page_common.clone());
        Ok(())
    }
    fn get_file_body(&self) -> String {
        self.file_body.clone()
    }
    fn get_class(&self)->Result<Vec<String>>{
        let yconf_c:MutexGuard<YConfig> = YCONF.lock()?;
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
        // 去重
        let mut rsl_unique_map = HashSet::new();
        for rsl_str_split in rsl_str.split(" "){
            if rsl_str_split!=""{
                if rsl_unique_map.insert(rsl_str_split){
                    rsl.push(String::from(rsl_str_split));
                }
            }
        }
        Ok(rsl)
    }

    fn get_new_css(&self, cls:Vec<String>) -> Result<String> {
        let mut common_c:MutexGuard<HashMap<String,Regex>> = COMMON.lock()?;
        let mut page_reg = common_c.clone();
        let singal_c:MutexGuard<HashMap<String,Regex>> = SINGAL.lock()?;
        // 重新组装common
        for (value,reg) in self.page_common.clone(){
            page_reg.insert(value.clone(),reg.clone());
        }
        let mut rsl = String::new() ;
        for cls_ in cls{
            for (value,reg) in page_reg.clone(){
                if reg.is_match(&cls_.as_str()){
                    let class_match = match reg.captures(cls_.as_str()){
                        Some(d)=>d,
                        None=>{
                            return Err(Box::try_from("没有匹配数据!")?)
                        }
                    };
                    let mut value_c = value;
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
                        if value_c_split!=""{
                            for value_c_split_ in value_c_split.split(" "){
                                let value_c_split_trim = value_c_split_.trim().to_string();
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
        let yconf_c:MutexGuard<YConfig> = YCONF.lock()?;
        let out_unit = &yconf_c.out_unit;
        let zoom_size = &yconf_c.zoom;
        let need_zoom_uint_str = format!("([0-9|\\.]{})[ |	]{}({}){}", "{1,10}", "{0,3}", yconf_c.need_zoom_unit, "{1,5}");
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
        // replace static
        let static_map = &yconf_c.static_map;
        for (key,value) in static_map {
            rsl__ = rsl__.replace(format!("@{}{}{}","{",key,"}").as_str(),value.as_str());
        }
        Ok(rsl__)
    }
    fn get_old_css(&self) ->Result<String>{
        let yconf_c:MutexGuard<YConfig> = YCONF.lock()?;
        let file_body = self.get_file_body();
        let mut rsl = String::from("");
        // 路径是一致的
        if self.out_path.eq(&self.path){
            let reg_reg = Regex::new(&yconf_c.old_css_reg.as_str())?;
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
            let mut old_css_reg_c = yconf_c.old_css_reg.clone() as String;
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
        for s in "qazwsxedcrfvtgbnhyujmki,ol.;p'[]1234567890-".chars(){
            let a_ = a.chars().filter(|x| *x==s).collect::<Vec<_>>().len();
            let b_ = b.chars().filter(|x| *x==s).collect::<Vec<_>>().len();
            if a_!=b_{
                // println!("not the same char is {} a:{} b:{}",s,a_,b_);
                rsl = false;
                break
            }
        };
        rsl
    }

    fn write(&self,new_css:String,old_css:String)->Result<()>{
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
            file.flush()?;
        }else{
            let file_body = self.get_file_body();
            let will_write = file_body.replace(old_css.as_str(),new_css.as_str());
            let mut file = File::create(&self.path)?;
            // println!("will_write:{}",will_write);
            file.write(will_write.as_bytes())?;
            file.flush()?;
        }
        Ok(())
    }
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