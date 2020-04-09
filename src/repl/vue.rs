use crate::repl::repl::Repl;
use std::io::Read;

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
}
