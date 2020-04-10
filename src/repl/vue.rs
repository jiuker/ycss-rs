use crate::repl::repl::Repl;
use crate::config::config::{YCONF,COMMON,SINGAL};
use std::io::Read;
use regex::Regex;
use std::ops::Add;
use std::collections::HashMap;

pub struct VueRepl {
    path:String,
}
impl Repl for VueRepl {
    fn new(path:String) -> VueRepl {
        VueRepl {
            path:path
        }
    }
    fn get_file_body(&self) -> String {
        let mut file = std::fs::File::open(&self.path).unwrap();
        let mut file_body = String::from("");
        file.read_to_string(&mut file_body).unwrap();
        file_body
    }
    fn get_class(&self)->Vec<String>{
        let yconf_c = YCONF.lock().unwrap();
        let file_body = self.get_file_body();
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
        let mut cls_map:HashMap<String,i32> = HashMap::new();
        for rsl_str_split in rsl_str.split(" "){
            if rsl_str_split!=""{
                cls_map.insert(String::from(rsl_str_split),0);
            }
        }
        for (key,_value) in cls_map{
            rsl.push(key);
        }
        return rsl;
    }

    fn get_new_class(&self,cls:Vec<String>) -> String {
        let common_c = COMMON.lock().unwrap();
        let singal_c = SINGAL.lock().unwrap();
        let mut rsl = String::new() ;
        for cls_ in cls{
            for (value,reg) in common_c.clone(){
                if reg.is_match(cls_.clone().as_str()){
                    let class_match = reg.captures(cls_.as_str()).unwrap();
                    let mut value_c = value.clone();
                    for match_index in 0..class_match.len() {
                        if !value_c.contains("$"){
                            break;
                        }
                        value_c = value_c.replace(format!("${}",match_index).as_str(),&class_match[match_index])
                    }
                    // get the common replace value:bb-1-fff
                    // println!("{:?}",value_c);
                    for (sv,sr) in singal_c.clone(){
                        let mut sv_c = sv;
                        let value_c1 = value_c.clone();
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
                            rsl = rsl.add(format!(".{}{}{}{}", value_c.as_str(), "{", sv_c.as_str(),"}\r\n").as_ref());
                        }
                    }
                    break
                }
            }
        }
        rsl
    }
}
