use crate::globals;
use std::fs;
use std::path::Path;

pub fn create_dirs(dir: &str) {
   for val in globals::DIRS {
      let path = Path::new(&dir).join(val);
      
      if !path.exists() {
         if let Err(e) = fs::create_dir(&path) {
            panic!("Unable to create {} directory: {}", val, e);
         }
         println!("{} directory created.", val)
      }
   }
}