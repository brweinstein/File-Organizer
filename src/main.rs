use std::fs;

fn main() {
    let entries = fs::read_dir("C:/Users/Brwei/Downloads").unwrap();

    let paths: Vec<_> = entries.map(|entry| entry.unwrap().path()).collect();

    for path in &paths {
        println!("{:?}", path);
    }
}
