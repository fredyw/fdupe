extern crate walkdir;
extern crate unicode_segmentation;

use std::cmp;
use std::path::Path;
use self::unicode_segmentation::UnicodeSegmentation;
use self::walkdir::WalkDir;

fn edit_distance(str1: &str, str2: &str) -> usize {
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
                dp[i][j] = 1 + cmp::min(
                    dp[i][j - 1],
                    cmp::min(dp[i - 1][j],
                        dp[i - 1][j - 1]));
            }
        }
    }
    dp[str1_size - 1 as usize][str2_size - 1 as usize]
}

#[derive(Debug)]
struct NamePath {
    name: String,
    path: String,
}

fn get_paths<'a>(dir: &str) -> Vec<NamePath> {
    let mut name_paths: Vec<NamePath> = vec![];
    let walker = WalkDir::new(dir).into_iter();
    for entry in walker.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let name_path = NamePath{
                name: path.file_name().unwrap().to_str().unwrap().to_string(),
                path: path.to_str().unwrap().to_string(),
            };
            name_paths.push(name_path);
        }
    }
    name_paths
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
        assert_eq!(5, fdupe::get_paths(Path::new("src").join("testdata").to_str().unwrap()).len());
    }
}