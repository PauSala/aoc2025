use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// This is actually super cool
///
/// 1. Get the file where the macro is called
/// 2. Build the input .txt file path assuming the name is the same
///    as the executing file.
///
/// It assumes you have the files in aoc/.data
#[macro_export]
macro_rules! day_input_path {
    () => {{
        let file_path = std::path::Path::new(file!());
        let day = file_path.file_stem().unwrap().to_str().unwrap();
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(".data")
            .join(format!("{}.txt", day))
    }};
}
