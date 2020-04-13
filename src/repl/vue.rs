use crate::repl::repl::Repl;
use crate::config::config::{YCONF, COMMON, SINGAL};
use std::io::{Read, Write};
use regex::{Regex, Captures};
use std::ops::Add;
use std::collections::VecDeque;
use std::fs::File;

pub struct VueRepl{
    path:String,
    file_body:String,
}
impl Repl for VueRepl {
    fn new(path:String) -> VueRepl {
        let mut file = std::fs::File::open(&path).unwrap();
        let mut file_body = String::from("");
        file.read_to_string(&mut file_body).unwrap();
        VueRepl {
            path,
            file_body
        }
    }
    fn get_file_body(&self) -> String {
        self.file_body.clone()
    }
    fn get_class(&self)->Vec<String>{
        let yconf_c = YCONF.lock().unwrap();
        let file_body = (*self).get_file_body();
        let mut rsl_str = String::from("");
        let mut rsl: Vec<String> = vec![];
        for reg in &yconf_c.reg{
            let reg_reg = Regex::new(reg).unwrap();
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
        return rsl;
    }

    fn get_new_css(&self, cls:Vec<String>) -> String {
        let common_c = COMMON.lock().unwrap();
        let singal_c = SINGAL.lock().unwrap();
        let mut rsl = String::new() ;
        for cls_ in cls{
            for (value,reg) in common_c.clone(){
                if reg.is_match(&cls_.as_str()){
                    let class_match = reg.captures(cls_.as_str()).unwrap();
                    let mut value_c:String = value.clone();
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
                                let sr_match = sr.captures(value_c1.as_str()).unwrap();
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
                    println!("{:?}",rsl_string.as_str());
                    rsl = rsl.add(rsl_string.as_str());
                    break
                }
            }
        }
        rsl = format!("/* Automatic generation Start */\r\n{}\r\n/*",rsl);
        // 缩放
        let yconf_c = YCONF.lock().unwrap();
        let out_unit = &yconf_c.outUnit;
        let zoom_size = &yconf_c.zoom;
        let need_zoom_uint_str = format!("([0-9|\\.]{})[ |	]{}({}){}","{1,10}","{0,3}",yconf_c.clone().needZoomUnit,"{1,5}");
        let reg_need_zoom = Regex::new(need_zoom_uint_str.as_str()).unwrap();
        let rsl_ = reg_need_zoom.replace_all(rsl.as_str(),|caps:&Captures| {
            let data = zoom_size*&caps[1].parse::<f32>().unwrap();
            format!("{}{}",data,out_unit)
        });
        rsl_.parse().unwrap()
    }
    fn get_old_css(&self) ->String{
        let yconf_c = YCONF.lock().unwrap();
        let file_body = self.get_file_body();
        let rsl = String::from("");
        let reg_reg = Regex::new(&yconf_c.oldCssReg.as_str()).unwrap();
        rsl.add(reg_reg.find(file_body.as_str()).unwrap().as_str())
    }

    fn is_same(&self,a: String, b: String) -> bool {
        let mut rsl = true;
        for s in same_str().bytes(){
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

    fn write(&self,new_css:String,old_css:String) {
        let file_body = self.get_file_body();
        let will_write = file_body.replace(old_css.as_str(),new_css.as_str());
        let mut file = File::create(&self.path).unwrap();
        // println!("will_write:{}",will_write);
        file.write(will_write.as_bytes()).unwrap();
    }
}
pub fn same_str() ->String{
     "qazwsxedcrfvtgbnhyujmki,ol.;p'[]1234567890-".parse().unwrap()
}