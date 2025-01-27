use std::env;

mod globals;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let dir = utils::get_dir(&args);
    utils::set_write_perms(&dir);
    utils::create_dirs(&dir);
    utils::organize(&dir);
}
