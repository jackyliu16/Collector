//! This file is using to communicate with local storage
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{self, BufReader, Write},
};

const JSON_STORE_PATH: &str = "weight.json";

pub struct RecordIterator {
    data: Vec<Record>,
}

impl RecordIterator {
    pub fn new(data: Vec<Record>) -> Self {
        let mut sorted_data = data;
        sorted_data.sort_by_key(|r| (-r.create_at.timestamp(), r.weight));
        RecordIterator { data: sorted_data }
    }
}

impl Iterator for RecordIterator {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.pop()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    /// weight
    pub weight: usize,
    /// decompression password
    pub password: String,
    /// generate time
    #[serde(with = "chrono::serde::ts_seconds")]
    pub create_at: DateTime<Utc>,
}

impl Record {
    pub fn new(password: String) -> Self {
        Record {
            weight: 0,
            password,
            create_at: Utc::now(),
        }
    }
}

/// read a vector of record from file
pub fn load_records() -> io::Result<Vec<Record>> {
    if !std::path::Path::new(JSON_STORE_PATH).exists() {
        let mut file = File::create(JSON_STORE_PATH)?;
        file.write_all(b"[]")?;
        return Ok(Vec::new());
    }

    let file = File::open(JSON_STORE_PATH)?;
    let reader = BufReader::new(file);

    Ok(serde_json::from_reader(reader)?)
}

fn store_records(vec: Vec<Record>) -> io::Result<()> {
    let mut file = File::create(JSON_STORE_PATH)?;
    let json_data = serde_json::to_vec(&vec)?;

    file.write_all(&json_data)?;

    Ok(())
}

/// update a record
pub fn add_weight(password: String) -> io::Result<()> {
    let mut records = load_records()?;

    if let Some(record) = records.iter_mut().find(|x| x.password == password) {
        record.weight += 1;
    } else {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Record not found"));
    }

    store_records(records)?;
    Ok(())
}

pub fn add_record(password: String) -> io::Result<()> {
    let mut records = load_records()?;

    if let Some(record) = records.iter_mut().find(|x| x.password == password) {
        Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "Record already exists",
        ))
    } else {
        let new_record = Record::new(password);
        records.push(new_record);
        store_records(records)?;
        Ok(())
    }
}

pub fn record_exist(password: String) -> io::Result<bool> {
    let mut records = load_records()?;
    Ok(records
        .iter_mut()
        .find(|x| x.password == password)
        .is_some())
}

mod tests {
    use super::*;
    #[allow(unused_imports)]
    use std::fs;

    #[allow(dead_code)]
    fn create_test_file_with_records(records: Vec<Record>) -> io::Result<()> {
        let mut file = File::create(JSON_STORE_PATH)?;
        let json_data = serde_json::to_vec(&records)?;
        file.write_all(&json_data)?;
        Ok(())
    }

    #[test]
    fn test_add_record_to_empty_file() {
        if std::path::Path::new(JSON_STORE_PATH).exists() {
            fs::remove_file(JSON_STORE_PATH).unwrap();
        }
        add_record("password".into()).unwrap();
        assert!(record_exist("password".into()).unwrap());
    }

    #[test]
    fn test_add_record_to_exists_file() {
        create_test_file_with_records(vec![Record::new("111".into()), Record::new("222".into())])
            .unwrap();
        add_record("333".into()).unwrap();
        assert!(record_exist("111".into()).unwrap());
        assert!(record_exist("222".into()).unwrap());
        assert!(record_exist("333".into()).unwrap());
    }

    #[test]
    fn test_add_weight_to_empty_files() {
        if std::path::Path::new(JSON_STORE_PATH).exists() {
            fs::remove_file(JSON_STORE_PATH).unwrap();
        }

        let result = add_weight("".to_string());
        assert!(result.is_err(), "Expected error, but got Ok");
    }

    #[test]
    fn test_add_records_non_existing_record() {
        let records = vec![
            Record::new("password123".to_string()),
            Record::new("password456".to_string()),
        ];
        create_test_file_with_records(records).expect("Failed to create test file");

        let result = add_weight("password789".to_string());
        assert!(result.is_err(), "Expected error, but got Ok");

        if let Err(e) = result {
            assert_eq!(
                e.kind(),
                io::ErrorKind::NotFound,
                "Expected NotFound error, but got {:?}",
                e.kind()
            );
        }
    }
}
