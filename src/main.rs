mod config;

use config::Config;

fn main() {
    let config = Config::init();
    config.verify_config();
}
