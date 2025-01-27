use std::fs;
use std::path::Path;

pub fn set_write_perms<P: AsRef<Path>>(path: P) {
    //Remove readonly restriction on all files in given directory
    for entry in fs::read_dir(path).expect("unable to open") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if let Ok(mut perms) = fs::metadata(&path).map(|mt| mt.permissions()) {
                perms.set_readonly(false);
                fs::set_permissions(path, perms).expect("unable to set permissions");
            }
        }
    }
}
