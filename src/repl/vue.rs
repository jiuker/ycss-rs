use crate::config::config::{YConfig, COMMON, SINGAL, YCONF};
use crate::repl::repl::Repl;
use crate::run::runner::Result;
use crate::set_reg_hash;
use regex::{Captures, Regex};
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::ops::Add;
use std::path::Path;
use std::sync::MutexGuard;

macro_rules! char_count {
    ($countStr:ident,$countChar:ident) => {
        $countStr
            .chars()
            .filter(|x| *x == $countChar)
            .collect::<Vec<_>>()
            .len()
    };
}

macro_rules! str_match_reg {
    ($file_body:ident,$reg:expr,$result:ident) => {
        for reg_str in $reg {
            let reg = Regex::new(reg_str)?;
            let rsl_ = reg
                .find_iter($file_body.as_str())
                .map(|x| x.as_str().to_string())
                .collect::<Vec<_>>();
            for rsl__ in rsl_ {
                let mut index = 0;
                for rsl___ in rsl__.split("\"") {
                    if index == 1 {
                        $result = $result + " " + rsl___;
                    }
                    index = index + 1;
                }
            }
        }
    };
}

pub struct VueRepl {
    path: String,
    file_body: String,
    out_path: String,
    page_common: HashMap<String, Regex>,
}
impl Repl for VueRepl {
    fn new(path: String) -> VueRepl {
        VueRepl {
            path,
            file_body: "".to_string(),
            out_path: "".to_string(),
            page_common: Default::default(),
        }
    }
    fn init(&mut self) -> Result<()> {
        let yconf_c = YCONF.lock()?;
        let mut file = std::fs::File::open(&self.path)?;
        let mut file_body = String::from("");
        file.read_to_string(&mut file_body)?;
        // 赋值文件的body
        (*self).file_body = file_body.clone();
        // 识别输出路径
        (*self).out_path = match parse_out_path(self.path.clone(), yconf_c.clone().out_path) {
            Some(d) => d,
            None => "@FileDir@FileName@FileType".to_string(),
        };
        // 查询页面级别的公共样式
        let mut page_common_str = "".to_string();
        str_match_reg!(file_body, &yconf_c.page_common, page_common_str);
        // dbg!(page_common_str);
        let mut page_common_vec = vec![];
        for rsl_ in page_common_str.split("<") {
            for mut rsl__ in rsl_.split(">") {
                rsl__ = rsl__.trim();
                if rsl__ != "" {
                    page_common_vec.push(String::from(rsl__));
                }
            }
        }
        if page_common_vec.len() % 2 != 0 {
            return Err(Box::from("页面级别公共样式不正确!"));
        }
        let mut common_keys = vec![];
        let mut common_values = vec![];
        let mut index = 0;
        while index < page_common_vec.len() {
            if index % 2 == 0 {
                common_keys.push(
                    "^".to_string()
                        + page_common_vec[index].clone().as_ref()
                        + "$".to_string().as_ref(),
                )
            } else {
                common_values.push(page_common_vec[index].clone())
            }
            index = index + 1;
        }
        set_reg_hash!(common_keys, common_values, self.page_common);
        // dbg!(common_values);
        // dbg!(self.page_common.clone());
        Ok(())
    }
    fn get_file_body(&self) -> String {
        self.file_body.clone()
    }
    fn get_class(&self) -> Result<Vec<String>> {
        let yconf_c = YCONF.lock()?;
        let file_body = (*self).get_file_body();
        let mut rsl_str = String::from("");
        let mut rsl: Vec<String> = vec![];
        str_match_reg!(file_body, &yconf_c.reg, rsl_str);
        // 去重
        let mut rsl_unique_map = HashSet::new();
        for rsl_str_split in rsl_str.split(" ") {
            if rsl_str_split != "" {
                if rsl_unique_map.insert(rsl_str_split) {
                    rsl.push(String::from(rsl_str_split));
                }
            }
        }
        Ok(rsl)
    }

    fn get_new_css(&self, cls: Vec<String>) -> Result<String> {
        let common_c = COMMON.lock()?;
        let mut page_reg = common_c.clone();
        let singal_c = SINGAL.lock()?;
        // 重新组装common
        for (value, reg) in self.page_common.clone() {
            page_reg.insert(value.clone(), reg.clone());
        }
        let mut rsl = String::new();
        for cls_ in cls {
            if cls_.is_empty() {
                continue;
            }
            for (value, reg) in page_reg.clone() {
                if reg.is_match(&cls_.as_str()) {
                    let class_match = match reg.captures(cls_.as_str()) {
                        Some(d) => d,
                        None => return Err(Box::from("没有匹配数据!")),
                    };
                    let mut value_c = value;
                    for match_index in 0..class_match.len() {
                        if !value_c.contains("$") {
                            break;
                        }
                        value_c = value_c.replace(
                            format!("${}", match_index).as_str(),
                            &class_match[match_index],
                        );
                    }
                    // get the common replace value:bb-1-fff \n c-1-fff
                    // println!("{:?}",value_c);
                    let mut css_content = String::from("");
                    for value_c_split in value_c.split("\n") {
                        if value_c_split != "" {
                            for value_c_split_ in value_c_split.split(" ") {
                                let value_c_split_trim = value_c_split_.trim().to_string();
                                for (sv, sr) in singal_c.clone() {
                                    let mut sv_c = sv;
                                    let value_c1 = &value_c_split_trim;
                                    if sr.is_match(value_c1.as_str()) {
                                        let sr_match = match sr.captures(value_c1.as_str()) {
                                            Some(d) => d,
                                            None => return Err(Box::from("没有匹配数据!")),
                                        };
                                        for mr_index in 0..sr_match.len() {
                                            if !sv_c.contains("$") {
                                                break;
                                            }
                                            sv_c = sv_c.replace(
                                                format!("${}", mr_index).as_str(),
                                                &sr_match[mr_index],
                                            )
                                        }
                                        // println!("{:?}",sv_c);
                                        // set as css
                                        if !css_content.is_empty() {
                                            sv_c = sv_c.add("\r\n");
                                        }
                                        css_content = css_content.add(sv_c.trim());
                                    }
                                }
                            }
                        }
                    }
                    let rsl_string = format!(
                        ".{}{}{}{}",
                        cls_.as_str(),
                        "{",
                        css_content.as_str(),
                        "}\r\n"
                    );
                    // println!("{:?}",rsl_string.as_str());
                    rsl = rsl.add(rsl_string.as_str());
                    break;
                }
            }
        }
        rsl = format!("/* Automatic generation Start */\r\n{}\r\n/*", rsl);
        // 缩放
        let yconf_c = YCONF.lock()?;
        let out_unit = &yconf_c.out_unit;
        let zoom_size = &yconf_c.zoom;
        let need_zoom_uint_str = format!(
            "([0-9|\\.]{})[ |	]{}({}){}",
            "{1,10}", "{0,3}", yconf_c.need_zoom_unit, "{1,5}"
        );
        let reg_need_zoom = Regex::new(need_zoom_uint_str.as_str())?;
        let rsl_ = reg_need_zoom.replace_all(rsl.as_str(), |caps: &Captures| -> String {
            let base = match caps[1].parse::<f32>() {
                Ok(d) => d,
                Err(_) => {
                    return caps[0].to_string();
                }
            };
            let data = zoom_size * base;
            format!("{}{}", data, out_unit)
        });
        // 如果不是自己的文件需要追加地址
        let mut rsl__: String = rsl_.parse()?;
        if !self.out_path.eq(&self.path) {
            // 地址不一样
            rsl__ = rsl__.add(format!(" {}", self.path).as_ref());
        }
        // replace static
        let static_map = &yconf_c.static_map;
        for (key, value) in static_map {
            rsl__ = rsl__.replace(format!("@{}{}{}", "{", key, "}").as_str(), value.as_str());
        }
        Ok(rsl__)
    }
    fn get_old_css(&self) -> Result<String> {
        let yconf_c = YCONF.lock()?;
        let file_body = self.get_file_body();
        let mut rsl = String::from("");
        // 路径是一致的
        if self.out_path.eq(&self.path) {
            let reg_reg = Regex::new(&yconf_c.old_css_reg.as_str())?;
            let rsl_ = match reg_reg.find(file_body.as_str()) {
                Some(d) => d,
                None => {
                    return Err(Box::from("没有匹配到数据"));
                }
            };
            rsl = rsl.add(rsl_.as_str());
        } else {
            // 路径不一致
            let out_path = &self.out_path;
            let mut file_body = "".to_string();
            println!("old file is {}", out_path);
            OpenOptions::new()
                .read(true)
                .write(true)
                .open(out_path)?
                .read_to_string(&mut file_body)?;
            let mut old_css_reg_c = yconf_c.old_css_reg.clone() as String;
            old_css_reg_c = old_css_reg_c.add(format!(" {}", self.path).as_ref());
            println!("old css reg is {}", old_css_reg_c);
            let reg_reg = Regex::new(old_css_reg_c.as_str())?;
            rsl = match reg_reg.find(file_body.as_str()) {
                None => "".to_string(),
                Some(d) => rsl.add(d.as_str()),
            }
        }
        Ok(rsl)
    }

    fn is_same(&self, a: String, b: String) -> bool {
        let mut rsl = true;
        for s in "qazwsxedcrfvtgbnhyujmki,ol.;p'[]1234567890-".chars() {
            let a_ = char_count!(a, s);
            let b_ = char_count!(b, s);
            if a_ != b_ {
                // println!("not the same char is {} a:{} b:{}",s,a_,b_);
                rsl = false;
                break;
            }
        }
        rsl
    }

    fn write(&self, new_css: String, old_css: String) -> Result<()> {
        // 如果不是自己的文件需要追加地址
        let out_path = self.out_path.clone();
        if !out_path.eq(&self.path) {
            let mut file_body = "".to_string();
            println!("old file is {}", out_path);
            {
                let mut file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(out_path.clone())?;
                file.read_to_string(&mut file_body)?;
            }
            let mut file = File::create(out_path)?;
            let will_write = file_body.replace(old_css.as_str(), new_css.as_str());
            // println!("will_write:{}",will_write);
            file.write(will_write.as_bytes())?;
            file.flush()?;
        } else {
            let file_body = self.get_file_body();
            let will_write = file_body.replace(old_css.as_str(), new_css.as_str());
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
pub fn parse_out_path(file_path: String, out_path: String) -> Option<String> {
    let file_path_c = Path::new(&file_path);
    let mut rsl = out_path.clone();
    let file_dir = file_path.replace(file_path_c.file_name()?.to_str()?, "");
    let file_type = file_path_c
        .file_name()?
        .to_str()?
        .replace(file_path_c.file_stem()?.to_str()?, "");
    let file_name = file_path_c
        .file_name()?
        .to_str()?
        .replace(file_type.as_str(), "");
    rsl = rsl.replace("@FileDir", file_dir.as_str());
    rsl = rsl.replace("@FileName", file_name.as_str());
    rsl = rsl.replace("@FileType", file_type.as_str());
    Some(rsl)
}
