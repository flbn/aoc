use std::{
    env, fs,
    path::{Path, PathBuf},
};

fn cwd() -> PathBuf {
    return env::current_dir().expect("failed to get current working directory");
}

pub fn read_file(file: &str) -> String {
    let parent = Path::new(file).parent();
    let file = match parent {
        Some(par) => format!(
            "{}/input.txt",
            par.to_str()
                .expect("could not parse parent file directory as str")
        ),
        None => panic!("failed to get input.txt"),
    };
    let file_path = cwd().join(file);
    fs::read_to_string(file_path).expect("failed to read file")
}
