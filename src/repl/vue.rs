use crate::config::config::{COMMON, SINGAL, YCONF};
use crate::repl::repl::Repl;
use crate::run::runner::Result;
use crate::set_reg_hash;
use crate::web_log;
use regex::{Captures, Regex};
use std::collections::{HashMap, HashSet};

use crate::log::log::LOGCH;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::ops::Add;
use std::path::Path;

macro_rules! char_count {
    ($countStr:ident,$countChar:ident) => {
        $countStr
            .chars()
            .filter(|x| *x == $countChar)
            .collect::<Vec<_>>()
            .len()
    };
}
// 输入数据，匹配规则，接收结果
macro_rules! str_match_reg {
    ($file_body:expr,$reg:expr,$result:ident) => {
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
// 查找匹配的
macro_rules! find_captures {
    ($reg:ident,$str:ident) => {
        match $reg.captures($str.as_str()) {
            Some(d) => d,
            None => return Err(Box::from("没有匹配数据!")),
        }
    };
}
// 替换$   p-$1-$2=>padding:[$1]px [$2]px
macro_rules! replace_placeholder {
    ($target:ident,$rules:ident) => {
        for index in 0..$rules.len() {
            if !$target.contains("$") {
                break;
            }
            $target = $target.replace(format!("${}", index).as_str(), &$rules[index])
        }
    };
}
// w-12 h-15 => .w-12{width:12px}.h-15{height:15px}
macro_rules! class_to_css {
    ($class:ident,$reg:ident,$out:ident) => {
        for value_c_split_ in $class.split(" ") {
            let value_c_split_trim = value_c_split_.trim().to_string();
            for (mut sv, sr) in $reg.clone() {
                if sr.is_match(value_c_split_trim.as_str()) {
                    let sr_match = find_captures!(sr, value_c_split_trim);
                    replace_placeholder!(sv, sr_match);
                    // web_log!(LOGCH,"{:?}",sv_c);
                    // set as css
                    if !$out.is_empty() {
                        sv = sv.add("\r\n");
                    }
                    $out = $out.add(sv.trim());
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
        self.file_body = file_body.clone();
        // 识别输出路径
        self.out_path = match parse_out_path(&self.path, &yconf_c.out_path) {
            Some(d) => d,
            None => "@FileDir@FileName@FileType".to_string(),
        };
        // 查询页面级别的公共样式
        let mut page_common_str = "".to_string();
        str_match_reg!(&file_body, &yconf_c.page_common, page_common_str);
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
                        + page_common_vec.get(index).unwrap()
                        + "$".to_string().as_ref(),
                )
            } else {
                common_values.push(page_common_vec.get(index).unwrap())
            }
            index = index + 1;
        }
        set_reg_hash!(common_keys, common_values, self.page_common);
        Ok(())
    }
    fn get_file_body(&self) -> String {
        self.file_body.clone()
    }
    fn get_class(&self) -> Result<Vec<String>> {
        let yconf_c = YCONF.lock()?;
        let file_body = self.get_file_body();
        let mut rsl_str = String::from("");
        let mut rsl: Vec<String> = vec![];
        str_match_reg!(file_body, &yconf_c.reg, rsl_str);
        // 去重
        let mut rsl_unique_map = HashSet::new();
        for rsl_str_split in rsl_str.split(" ") {
            if rsl_str_split != "" {
                if rsl_unique_map.insert(rsl_str_split) {
                    rsl.push(format!(".{}", rsl_str_split));
                }
            }
        }
        Ok(rsl)
    }
    // 如果 common 里面不包含特殊正则匹配，也就是不包含 $ 的时候需要把这个样式无条件输出到页面上
    fn get_new_css(&self, cls: Vec<String>) -> Result<String> {
        let common_c = COMMON.lock()?;
        let mut page_reg = common_c.clone();
        let singal_c = SINGAL.lock()?;
        // 重新组装页面级别的规则到所有匹配规则
        for (value, reg) in self.page_common.iter() {
            page_reg.insert(value.clone(), reg.clone());
        }
        // 匹配替换全部的css
        let mut rsl = String::new();
        for cls_ in cls {
            if cls_.is_empty() {
                continue;
            }
            for (mut value, reg) in page_reg.clone() {
                if reg.is_match(&cls_.as_str()) {
                    let class_match = find_captures!(reg, cls_);
                    replace_placeholder!(value, class_match);
                    // get the common replace value:bb-1-fff \n c-1-fff
                    // web_log!(LOGCH,"{:?}",value_c);
                    let mut css_content = String::from("");
                    for value_c_split in value.split("\n") {
                        if value_c_split != "" {
                            class_to_css!(value_c_split, singal_c, css_content);
                        }
                    }
                    rsl = format!(
                        "{}{}{}{}{}",
                        rsl,
                        cls_.as_str(),
                        "{",
                        css_content.as_str(),
                        "}\r\n"
                    );
                    // web_log!(LOGCH,"{:?}",rsl_string.as_str());
                    break;
                }
            }
        }
        // 追加 page_common string 里面不含有$规则
        let mut _page_common_rsl = "".to_string();
        for (value, reg) in self.page_common.clone() {
            if !value.contains("$") {
                let mut css_content = String::from("");
                class_to_css!(value, singal_c, css_content);
                _page_common_rsl = format!(
                    "{}{}{}{}{}",
                    _page_common_rsl,
                    reg.to_string().replace("^", "").replace("$", ""),
                    "{",
                    css_content,
                    "}\r\n"
                );
            }
        }
        rsl = format!(
            "/* Automatic generation Start */\r\n{}{}\r\n/*",
            rsl, _page_common_rsl
        );
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
            web_log!("old file is {}", out_path);
            OpenOptions::new()
                .read(true)
                .write(true)
                .open(out_path)?
                .read_to_string(&mut file_body)?;
            let mut old_css_reg_c = yconf_c.old_css_reg.clone() as String;
            old_css_reg_c = old_css_reg_c.add(format!(" {}", self.path).as_ref());
            web_log!("old css reg is {}", old_css_reg_c);
            let reg_reg = Regex::new(old_css_reg_c.as_str())?;
            rsl = match reg_reg.find(file_body.as_str()) {
                None => "".to_string(),
                Some(d) => rsl.add(d.as_str()),
            }
        }
        Ok(rsl)
    }

    fn is_same(&self, a: &String, b: &String) -> bool {
        let mut rsl = true;
        for s in "qazwsxedcrfvtgbnhyujmki,ol.;p'[]1234567890-".chars() {
            let a_ = char_count!(a, s);
            let b_ = char_count!(b, s);
            if a_ != b_ {
                // web_log!(LOGCH,"not the same char is {} a:{} b:{}",s,a_,b_);
                rsl = false;
                break;
            }
        }
        rsl
    }

    fn write(&self, new_css: &String, old_css: &String) -> Result<()> {
        // 如果不是自己的文件需要追加地址
        let out_path = self.out_path.clone();
        if !out_path.eq(&self.path) {
            let mut file_body = "".to_string();
            web_log!("old file is {}", out_path);
            {
                let mut file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(out_path.clone())?;
                file.read_to_string(&mut file_body)?;
            }
            let mut file = File::create(out_path)?;
            let will_write = file_body.replace(old_css.as_str(), new_css.as_str());
            // web_log!(LOGCH,"will_write:{}",will_write);
            file.write(will_write.as_bytes())?;
            file.flush()?;
        } else {
            let file_body = self.get_file_body();
            let will_write = file_body.replace(old_css.as_str(), new_css.as_str());
            let mut file = File::create(&self.path)?;
            // web_log!(LOGCH,"will_write:{}",will_write);
            file.write(will_write.as_bytes())?;
            file.flush()?;
        }
        Ok(())
    }
}

// 解析输出路径，以下是全路径
// @FileDir@FileName@FileType
pub fn parse_out_path(file_path: &String, out_path: &String) -> Option<String> {
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
