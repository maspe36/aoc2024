use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::{Path, PathBuf};

/// Get the lines of the given path from the workspace root.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(workspace_dir().join(filename))?;
    Ok(io::BufReader::new(file).lines())
}

// Shout out to this guy
// https://stackoverflow.com/a/74942075/4603553
fn workspace_dir() -> PathBuf {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;
    let cargo_path = Path::new(std::str::from_utf8(&output).unwrap().trim());
    cargo_path.parent().unwrap().to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = read_lines("helpers/test.txt")
            .unwrap()
            .flatten()
            .collect::<Vec<String>>();
        assert_eq!(result, vec!["foobar"]);
    }
}
