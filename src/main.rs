// std::fs::Metadata.is_dir() will confirm that the path points to a directory.

fn main() {
    let path_arg = std::env::args()
        .skip(2)
        .rev()
        .next()
        .unwrap_or(".".to_string());

    let dir_metadata = std::fs::metadata(&path_arg).expect("Could not open path!");

    match dir_metadata.is_dir() {
        true => println!("Path argument '{}' is a directory! OuO", path_arg),
        false => println!("Path argument '{}' is not a directory! OnO", path_arg),
    }
}
