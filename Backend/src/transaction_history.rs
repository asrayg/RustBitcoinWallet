extern crate bitcoin;
extern crate chrono;

use bitcoin::network::constants::Network;
use chrono::{DateTime, Utc, serde::ts_seconds};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{self, BufReader, BufWriter};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionRecord {
    pub txid: String,
    pub amount: i64,
    #[serde(with = "ts_seconds")]
    pub timestamp: DateTime<Utc>,
}

pub struct TransactionHistory {
    records: Vec<TransactionRecord>,
    file_path: String,
}

impl TransactionHistory {
    pub fn new(file_path: &str) -> Self {
        TransactionHistory {
            records: vec![],
            file_path: file_path.to_string(),
        }
    }

    pub fn add_record(&mut self, txid: String, amount: i64) {
        let record = TransactionRecord {
            txid,
            amount,
            timestamp: Utc::now(),
        };
        self.records.push(record);
        self.save().unwrap();
    }

    pub fn load(&mut self) -> io::Result<()> {
        let path = Path::new(&self.file_path);
        if path.exists() {
            let file = OpenOptions::new().read(true).open(&path)?;
            let reader = BufReader::new(file);
            self.records = serde_json::from_reader(reader)?;
        }
        Ok(())
    }

    pub fn save(&self) -> io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self.records)?;
        Ok(())
    }

    pub fn get_records(&self) -> &Vec<TransactionRecord> {
        &self.records
    }
}
