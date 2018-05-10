fn arg_is_a_dir_path(arg: &str) -> bool {
    let dir_metadata = std::fs::metadata(&arg);
    match dir_metadata {
        Ok(md) => md.is_dir(),
        Err(_) => false,
    }
}

fn get_path_arg() -> String {
    for arg in std::env::args().skip(2).rev() {
        if arg_is_a_dir_path(&arg) {
            return arg.clone()
        }
    }

    // FIXUP: Would using std::env::current_dir() be better here?
    ".".to_string()
}

fn main() {
    let path_arg = get_path_arg();
    println!("Path argument is: {}", path_arg);
}
