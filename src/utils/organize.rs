use crate::{globals, utils};
use std::env::consts;
use std::fs;

pub fn organize(path: &str) {
   let main_dir = fs::read_dir(&path).expect("Unable to open");
   let map = utils::ext_map();
   let dirs = globals::DIRS.to_vec();

   for entry in main_dir {
      let entry = entry.unwrap().path();
      let name = entry.file_name().unwrap().to_str().unwrap();
      let ignore_dir = dirs.iter().position(|&v| v == name) != None;

      if ignore_dir {
         continue;
      }

      let ext = entry.extension();

      if ext != None {
         let dir = map.get(ext.unwrap().to_str().unwrap());

         if dir != None {
            let dir_str = &format!("{}/{}", path, dir.unwrap());
            let dir = std::path::Path::new(dir_str);
            let _ = utils::move_file(entry, dir);  // Ignore Result
         } else {
            let dir_str = format!("{}/{}", path, globals::DIRS[4]);
            let dir = std::path::Path::new(&dir_str);
            let _ = utils::move_file(entry, dir);  // Ignore Result
         }
      } else if consts::OS != "windows" {
         let dir_str = format!("{}/{}", path, globals::DIRS[4]);
         let dir = std::path::Path::new(&dir_str);
         let _ = utils::move_file(entry, dir);  // Ignore Result
      }
   }
}
