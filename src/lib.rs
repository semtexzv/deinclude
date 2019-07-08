use std::path::{Path, PathBuf};
use std::{io, fs};
use regex::Captures;

pub const RE: &str = r#"#include\s+["<]([.\w\\/]+)[">]"#;
pub const COM_RE: &str = r#"/\*([^*]|[\r\n]|(\*([^/]|[\r\n])))*\*/"#;

pub fn read_within(name: &str, path: &Path) -> Option<(PathBuf, String)> {
    let file = path.join(name);
    //let file = fs::canonicalize(path).unwrap();
    if file.exists() {
        return Some((file.clone(), fs::read_to_string(file).unwrap()));
    }
    return None;
}

pub fn deinclude_contents(data: &str, root_path: &Path, local_path: &Path, ignore: &[String]) -> io::Result<String> {
    let re = regex::Regex::new(RE).unwrap();
    let com = regex::Regex::new(COM_RE).unwrap();

    let data = com.replace_all(data, |c: &Captures| {
        //panic!("{:?}", c.get(0).unwrap().as_str());
        "".to_string()
    }).to_owned();

    let replacer = |c: &regex::Captures| {
        if let Some(m) = c.get(1) {
            let name = m.as_str();

            if ignore.iter().find(|s| s.as_str() == name).is_some() {
                return "".to_string();
            }

            if let Some((file, data)) = read_within(name, root_path) {
                return deinclude_contents(&data, root_path, file.parent().unwrap(), ignore).unwrap();
            } else if let Some((file, data)) = read_within(name, local_path) {
                return deinclude_contents(&data, root_path, file.parent().unwrap(), ignore).unwrap();
            } else {
                panic!("Could not deinclude {:?} , {:?}, {:?}", name, root_path, local_path)
            }
        }
        "".to_string()
    };
    Ok(re.replace_all(&data, replacer).into_owned())
}


pub fn deinclude(path: impl AsRef<Path>, ignore: &[String]) -> io::Result<String> {
    let path = path.as_ref();
    let path = std::fs::canonicalize(path).unwrap();

    let file = std::fs::read_to_string(&path)?;

    deinclude_contents(&file, path.parent().unwrap(), path.parent().unwrap(), ignore)
}