use std::env;
use std::fs;
use std::option::Option;
use std::path::PathBuf;


trait PathProvider<T> {
    fn get_home(&self) -> Option<T>;
    fn join(&self, &str) -> Self;
    fn is_dir(&self) -> bool;
    fn as_pathbuf(&self) -> PathBuf;
}

impl PathProvider<PathBuf> for PathBuf {
    fn get_home(&self) -> Option<PathBuf> {
        env::home_dir()
    }

    fn join(&self, p: &str) -> PathBuf {
        PathBuf::join(self, p)
    }

    fn is_dir(&self) -> bool {
        PathBuf::is_dir(self)
    }

    fn as_pathbuf(&self) -> PathBuf {
        self.to_owned()
    }
}

struct MockedPathBuf {
    current: String,
    dir: bool,
}

impl MockedPathBuf {
    fn new(p: &str, d: bool) -> MockedPathBuf {
        MockedPathBuf {
            current: p.to_string(),
            dir: d,
        }
    }
}

impl PathProvider<MockedPathBuf> for MockedPathBuf {
    fn get_home(&self) -> Option<MockedPathBuf> {
        Some(self.to_owned())
    }

    fn join(&self, p: &str) -> MockedPathBuf {
        let c = self.current + p;
        MockedPathBuf::new(&c, self.dir)
    }

    fn is_dir(&self) -> bool {
        self.dir
    }

    fn as_pathbuf(&self) -> PathBuf {
        PathBuf::from(&self.current)
    }
}

fn _get_root<T: PathProvider<PathBuf>>(hd: &T) -> Option<PathBuf> {
    match hd.get_home() {
        Some(h) => {
            let d = h.join(".password-store");

            if d.is_dir() {
                return Some(d.as_pathbuf());
            }

            None
        }
        None => None,
    }
}

#[test]
fn test_get_root() {
    let hd = MockedPathBuf::new("/tmp", true);
    //get_root_typed(&hd)
}

pub fn get_root() -> Option<PathBuf> {
    let hd = PathBuf::new();
    _get_root::<PathBuf>(&hd)
}

pub fn get_recipients() -> Option<Vec<PathBuf>> {
    get_root()
        .map(|mut p| {
            p.push(".recipients");
            p
        })
        .and_then(|p| match fs::read_dir(p.as_path()) {
            Ok(l) => Some(l),
            Err(_) => None,
        })
        .map(|l| {
            l.filter(|e| e.as_ref().unwrap().path().is_file())
                .map(|e| e.unwrap().path())
                .collect::<Vec<PathBuf>>()
        })
        .and_then(|l| {
            if l.len() > 0 {
                return Some(l);
            }

            None
        })
}
