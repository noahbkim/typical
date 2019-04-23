use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use crate::dictionary::DictionaryError::TypeError;

#[derive(Debug)]
pub enum DictionaryError {
    FileError,
    TypeError,
}

pub trait Dictionary {
    fn next(&self) -> String;
}

pub trait DictionaryLoader {
    fn load(lines: Lines<BufReader<File>>) -> Result<Box<Self>, DictionaryError>;
}

pub struct ComposeDictionary {
    words: Vec<String>,
}

impl Dictionary for ComposeDictionary {
    fn next(&self) -> String {
        "".to_string()
    }
}

impl DictionaryLoader for ComposeDictionary {
    fn load(lines: Lines<BufReader<File>>) -> Result<Box<Self>, DictionaryError> {
        let mut words: Vec<String> = Vec::new();
        for line in lines {
            match line {
                Ok(contents) => words.push(contents.trim().to_string()),
                Err(error) => return Err(DictionaryError::FileError)
            }
        }
        Ok(Box::new(ComposeDictionary { words }))
    }
}

pub struct VerbatimDictionary {
    lines: Vec<String>,
}

pub fn load_dictionary(path: String) -> Result<Box<impl Dictionary>, DictionaryError> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(error) => return Err(DictionaryError::FileError),
    };

    let mut reader = BufReader::new(file);
    let mut first = String::new();
    reader.read_line(&mut first).expect("failed to read first line!");
    first = first.trim().to_lowercase();

    if first == "compose" {
        return ComposeDictionary::load(reader.lines())
    }

    Err(DictionaryError::TypeError)
}
