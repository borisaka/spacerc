use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::error::Error;

use regex::{Regex, Captures};

fn parse_aliases(
    content: &str
) -> Result<HashMap<&str, PathBuf>, Box<dyn Error>> {
    let mut aliases = HashMap::new();
    for line in content.lines() {
        if line.trim().len() == 0 {
            continue
        }

        let (name, value) = parse_pair(&line)?;
        let path = expand_alias_value(&value, &aliases)?;
        aliases.insert(name, path);
    }

    return Ok(aliases);
}

fn expand_alias_value(
    value: &str, aliases: &HashMap<&str, PathBuf>
) -> Result<PathBuf, Box<dyn Error>> {
    Ok(fs::canonicalize(value)?)
}

fn extract_aliases<'a>(
    path_str: &str, aliases: &'a HashMap<&str, PathBuf>
) -> Result<&'a str, Box<dyn Error>> {
    let re = Regex::new(r"\[(.*)\]")?;
    for alias in re.captures_iter(path_str) {
        
    }
}

fn parse_pair(pair: &str) -> Result<(&str, &str), String> {
    let mut splitted = pair.split("=");
    let err = || Err(format!("Could not parse value \"{}\"",pair));

    let first = match splitted.next() {
        Some(val) if val.len() > 0 => val.trim(),
        _ => return err()
    };

    let second = match splitted.next() {
        Some(val) if val.len() > 0 => val.trim(),
        _ => return err()
    };

    // return err if arguments more then 2
    match splitted.next() {
        Some(_) => err(),
        None => Ok((first, second))
    }
}

// TODO make temp directories
#[cfg(test)]
mod test {
    use super::*;
    use std::{env, fs};
    // use std::time::SystemTime;
    // use std::fs::File;
    // use std::io::prelude::*;

    fn assert_parse_err(arg: &str) {
        assert_eq!(
            Err(format!("Could not parse value \"{}\"", arg)),
            parse_pair(arg)
        )
    }

    #[test]
    fn test_parse_pair() {
        assert_eq!(Ok(("a", "b")), parse_pair("a=b"));
        assert_parse_err("a=b=c");
        assert_parse_err("a=");
    }

    #[test]
    fn test_parse_aliases() {
        let aliases = r#"
            a=/home/boris
            b=/etc
        "#;
        let mut expected = HashMap::new();
        expected.insert("a", PathBuf::from("/home/boris"));
        expected.insert("b", PathBuf::from("/etc"));
        assert_eq!(expected, parse_aliases(&aliases).unwrap())
    }

    #[test]
    fn test_complicated_aliases() {
        let aliases = r#"
            home=/home/boris
            fish=[home]/.config/fish
            etc=/etc
            nginx=[etc]/nginx/nginx.conf
        "#;
        let mut expected = HashMap::new();
        expected.insert("home", PathBuf::from("/home/boris"));
        expected.insert("fish", PathBuf::from("/home/boris/.config/fish"));
        expected.insert("etc", PathBuf::from("/etc"));
        expected.insert("nginx", PathBuf::from("/etc/nginx/nginx.conf"));
        assert_eq!(expected, parse_aliases(&aliases).unwrap())
    }
}
