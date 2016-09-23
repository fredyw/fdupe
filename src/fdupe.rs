extern crate walkdir;
extern crate unicode_segmentation;
extern crate regex;

use std::cmp::min;
use std::error::Error;
use self::unicode_segmentation::UnicodeSegmentation;
use self::walkdir::WalkDir;
use self::regex::Regex;

pub fn find_duplicates(dir: &str, expected_dist: i32, filter: Option<String>) -> Result<(), String> {
    let paths = try!(get_paths(dir, filter));
    for i in 0..paths.len() {
        let mut duplicates: Vec<&NamePath> = vec![];
        let ref path1 = paths[i];
        for j in (i + 1)..paths.len() {
            let ref path2 = paths[j];
            let dist = edit_distance(&path1.name, &path2.name);
            if dist <= expected_dist {
                duplicates.push(path2);
            }
        }
        if duplicates.len() > 0 {
            println!("-----------------------------------------------------------------------");
            println!("* {} --> {} *", path1.name, path1.path);
            println!("-----------------------------------------------------------------------");
            for dupe in duplicates {
                println!("- Possible duplicate: {} --> {}", dupe.name, dupe.path);
            }
        }
    }
    Ok(())
}

fn edit_distance(str1: &str, str2: &str) -> i32 {
    let str1_vec = UnicodeSegmentation::graphemes(str1, true).collect::<Vec<&str>>();
    let str2_vec = UnicodeSegmentation::graphemes(str2, true).collect::<Vec<&str>>();
    let str1_size = str1_vec.len() + 1;
    let str2_size = str2_vec.len() + 1;
    let mut dp = vec![vec![0; str2_size]; str1_size];
    for i in 0..str1_size {
        for j in 0..str2_size {
            if i == 0 {
                dp[i][j] = j;
            } else if j == 0 {
                dp[i][j] = i;
            } else if str1_vec[i - 1] == str2_vec[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + min(
                    dp[i][j - 1],
                    min(dp[i - 1][j],
                        dp[i - 1][j - 1]));
            }
        }
    }
    dp[str1_size - 1 as usize][str2_size - 1 as usize] as i32
}

#[derive(Debug)]
struct NamePath {
    name: String,
    path: String,
}

fn get_paths(dir: &str, filter: Option<String>) -> Result<Vec<NamePath>, String> {
    let mut name_paths: Vec<NamePath> = vec![];
    let walker = WalkDir::new(dir).into_iter();
    for entry in walker.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let file_name = match path.file_name() {
                Some(f1) => {
                    match f1.to_str() {
                        Some(f2) => { f2.to_string() }
                        None => { return Err(String::from("File name does not exist")) }
                    }
                }
                None => { return Err(String::from("File name does not exist")) }
            };
            let file_path = match path.to_str() {
                Some(p) => { p.to_string() }
                None => { return Err(String::from("Path does not exist")) }  
            };
            match filter {
                Some(ref regex) => {
                    let re = match Regex::new(regex) {
                        Ok(regex) => { regex }
                        Err(err) => { return Err(err.description().to_string()) }
                    };
                    if re.is_match(&file_name) {
                        let name_path = NamePath{
                            name: file_name,
                            path: file_path,
                        };
                        name_paths.push(name_path); 
                    } 
                }
                None => {
                    let name_path = NamePath{
                        name: file_name,
                        path: file_path,
                    };
                    name_paths.push(name_path);
                }
            }
        }
    }
    Ok(name_paths)
}

#[cfg(test)]
mod test {
    use fdupe;
    use std::path::Path;

    #[test]
    fn test_edit_distance() {
        assert_eq!(2, fdupe::edit_distance("hello", "holla"));
        assert_eq!(2, fdupe::edit_distance("hell", "holla"));
        assert_eq!(3, fdupe::edit_distance("abc", "def"));
        assert_eq!(1, fdupe::edit_distance("忠犬ハチ公", "忠犬ハチ"));
        assert_eq!(2, fdupe::edit_distance("犬ハチ公", "忠犬ハチ"));
    }

    #[test]
    fn test_get_paths() {
        assert_eq!(8, fdupe::get_paths(
            Path::new("src").join("testdata").to_str().unwrap(),
            Option::None).unwrap().len());

        assert_eq!(5, fdupe::get_paths(
            Path::new("src").join("testdata").to_str().unwrap(),
                Option::Some(String::from("test.*"))).unwrap().len());
    }

    #[test]
    fn test_find_duplicates() {
        match fdupe::find_duplicates(
            Path::new("src").join("testdata").to_str().unwrap(), 3, Option::None) {
            Ok(()) => {}
            Err(err) => { panic!(err) }
        }

        match fdupe::find_duplicates(
            Path::new("src").join("testdata").to_str().unwrap(), 3,
                Option::Some(String::from("test.*"))) {
            Ok(()) => {}
            Err(err) => { panic!(err) }
        }
    }
}