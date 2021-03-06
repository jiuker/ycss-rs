use std::ops::Add;
extern crate regex;
use crate::run::runner::Result;

pub fn read_all_paths(dir: &String, file_type: &String) -> Result<Vec<String>> {
    let dirs = std::fs::read_dir(dir)?;
    let mut paths: Vec<String> = vec![];
    let file_type_more = String::from(".").add(file_type.as_str()).add("$");
    let reg = regex::Regex::new(file_type_more.as_str())?;
    for x in dirs {
        let x_u = x?;
        let x_meta = x_u.metadata()?;
        if x_meta.is_dir() {
            for path in read_all_paths(
                &match x_u.path().to_str() {
                    Some(d) => d,
                    None => {
                        return Err(Box::from("没有找到该目录"));
                    }
                }
                .to_owned(),
                file_type,
            )? {
                paths.push(path);
            }
        } else {
            let x_path = match x_u.path().to_str() {
                Some(d) => d,
                None => return Err(Box::from("没有找到路径")),
            }
            .to_string();
            if reg.is_match(x_path.as_str()) {
                paths.push(x_path);
            }
        }
    }
    Ok(paths)
}
