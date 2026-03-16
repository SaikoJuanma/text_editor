use std::fs;
use std::io::Error;
use std::path::Path;

pub fn open_or_make_file(path: Option<&str>) -> Result<String, std::io::Error> {
    match path {
        Some(p) => {
            if Path::new(p).exists() {
                open_file(p)
            } else {
                make_file(Some(p))
            }
        }

        None => make_file(None),
    }
}

fn open_file(path: &str) -> Result<String, Error> {
    fs::read_to_string(path)
}

fn make_file(path: Option<&str>) -> Result<String, Error> {
    match path {
        Some(p) => {
            fs::File::create(p)?;
            Ok(String::new())
        }

        None => {
            fs::File::create("untitled.txt")?;
            Ok(String::new())
        }
    }
}

pub fn save_file(path: &str, content: &str) -> Result<(), Error> {
    fs::write(path, content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn opens_existing_file_with_content() {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        write!(file, "hello").unwrap();
        let content = open_or_make_file(Some(file.path().to_str().unwrap())).unwrap();
        assert_eq!(content, "hello");
    }

    #[test]
    fn creates_new_file_when_path_does_not_exist() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("new.txt");
        let content = open_or_make_file(Some(path.to_str().unwrap())).unwrap();
        assert!(content.is_empty());
        assert!(path.exists());
    }

    #[test]
    fn creates_untitled_when_no_path_given() {
        let dir = tempfile::tempdir().unwrap();
        let original = std::env::current_dir().unwrap();
        std::env::set_current_dir(&dir).unwrap();
        let content = open_or_make_file(None).unwrap();
        std::env::set_current_dir(original).unwrap();
        assert!(content.is_empty());
        assert!(dir.path().join("untitled.txt").exists());
    }

    #[test]
    fn save_file_writes_content_to_disk() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.txt");
        save_file(path.to_str().unwrap(), "hello world").unwrap();
        let saved = fs::read_to_string(&path).unwrap();
        assert_eq!(saved, "hello world");
    }
}
