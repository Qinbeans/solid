use std::{path::PathBuf};
use std::fs::File;
use std::io::Read;
use sha2::{Sha256, Digest};

const IGNORES: [&str; 2] = [".DS_Store","checksum"];

pub struct Integrity {
    expected: String,
    directory: PathBuf,
}

impl Integrity {
    pub fn new(expected: String, directory: PathBuf) -> Self {
        Self { expected, directory }
    }
    pub fn check(&self) -> Result<(), String> {
        // Check if the directory exists
        if !self.directory.exists() {
            return Err("Directory does not exist".to_string());
        }
        // Check if the directory is a directory
        if !self.directory.is_dir() {
            return Err("Path is not a directory".to_string());
        }
        // Check if the directory is empty
        if self.directory.read_dir().unwrap().next().is_none() {
            return Err("Directory is empty".to_string());
        }
        //recursively go down the directory and sum the files
        let mut sum = Vec::new();
        let paths = self.directory.read_dir();
        if paths.is_err() {
            return Err("Could not read directory from paths".to_string());
        }
        let paths = paths.unwrap();
        for entry in paths {
            if entry.is_err() {
                return Err("Could not read directory from entry".to_string());
            }
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                let res = self.rec_sum(path.to_owned());
                if let Err(err) = res {
                    return Err(format!("Could not read directory {:?} from res: {}", path, err));
                }
                sum.append(&mut res.unwrap());
            } else {
                //if the file is in the ignore list, skip it
                if IGNORES.contains(&path.file_name().unwrap().to_str().unwrap()) {
                    continue;
                }
                let res = self.file_sum(path);
                if let Err(err) = res {
                    return Err(format!("Could not read file from res: {}", err));
                }
                sum.append(&mut res.unwrap());
            }
        }
        // Check if the sum is the same as the expected sum
        let mut hasher = Sha256::new();
        hasher.update(sum);
        let sum = hasher.finalize();
        let sum = format!("{:x}", sum);
        if self.expected != sum {
            println!("Expected {}, got {}", self.expected, sum);
            return Err("Integrity check failed".to_string());
        }
        Ok(())
    }
    fn rec_sum(&self, path: PathBuf) -> Result<Vec<u8>, String> {
        let mut sum = Vec::new();
        let paths = path.read_dir();
        if paths.is_err() {
            return Err("Could not read directory from paths(rec_sum)".to_string());
        }
        let paths = paths.unwrap();
        for entry in paths {
            if entry.is_err() {
                return Err("Could not read directory from entry(rec_sum)".to_string());
            }
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                let res = self.rec_sum(path.to_owned());
                if let Err(err) = res {
                    return Err(format!("Could not read directory {:?} from res(rec_sum): {}", path, err));
                }
                sum.append(&mut res.unwrap());
            } else {
                //if the file is in the ignore list, skip it
                if IGNORES.contains(&path.file_name().unwrap().to_str().unwrap()) {
                    continue;
                }
                let res = self.file_sum(path.to_owned());
                if let Err(err) = res {
                    return Err(format!("Could not read file {:?} from res(rec_sum): {}", path, err));
                }
                sum.append(&mut res.unwrap());
            }
        }
        Ok(sum)
    }
    fn file_sum(&self, path: PathBuf) -> Result<Vec<u8>, String> {
        let file = File::open(path.to_owned());
        if file.is_err() {
            return Err("Could not open file(file_sum)".to_string());
        }
        let mut file = file.unwrap();
        let mut buf = Vec::new();
        //read the file in as bytes
        let res = file.read_to_end(&mut buf);
        if let Err(err) = res {
            return Err(format!("Could not read file {:?} from res(file_sum): {:?}", path, err));
        }
        Ok(buf)
    }
}