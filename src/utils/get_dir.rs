use std::path::Path;
use directories::UserDirs;

pub fn get_dir(args: &[&str]) -> String {
    match args.len() {
        1 => {
            let user = UserDirs::new().unwrap();
            let dir = if args[0] == "home" {
                user.home_dir()
            } else {
                user.download_dir().expect("Directory not found")
            };

            if !dir.exists() {
                panic!("Directory not found");
            }
            return dir.to_str().unwrap().to_string();
        }
        2 => {
            let custom_dir = Path::new(&args[1]);
            let canonical_dir = custom_dir
                .canonicalize()
                .unwrap_or_else(|_| panic!("Directory not found"));
            return canonical_dir.to_str().unwrap().to_string();
        }
        _ => panic!("Too many arguments"),
    }
}
