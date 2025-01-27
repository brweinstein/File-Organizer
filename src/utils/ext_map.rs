use crate::globals::{AUD_EXT, IMG_EXT, TEXT_EXT, VID_EXT, DIRS};
use std::collections::HashMap;

fn to_map(
    map: &mut HashMap<&'static str, &'static str>,
    ext: &[&'static str],
    dir: &'static str,
  ) {
    for ext in ext {
      map.insert(ext, dir);
    }
}

pub fn ext_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();

    to_map(&mut map, &TEXT_EXT, DIRS[0]);
    to_map(&mut map, &IMG_EXT, DIRS[1]);
    to_map(&mut map, &VID_EXT, DIRS[2]);
    to_map(&mut map, &AUD_EXT, DIRS[3]);

    map
}
  