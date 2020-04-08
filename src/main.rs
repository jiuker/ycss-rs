extern crate ycss;

use ycss::config::config;

fn main() {
    config::set_config_path(String::from("./res/config/config.json"));
}
