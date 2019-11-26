use crate::gateway::Gateway;
use chrono::NaiveDate;
use std::ops::Add;
use time::Duration;

type Money = i64;

#[derive(Debug)]
pub struct RecognitionService {
    db: Gateway
}

impl RecognitionService {
    pub fn new(db: Gateway) -> RecognitionService {
        RecognitionService {
            db
        }
    }

    pub fn recognized_revenue(&self, contract_num: i64, as_of: NaiveDate) -> Money {
        let rows = self.db.find_recognition_for(contract_num, as_of).unwrap();

        rows.iter().map(|row| {
            let amount: i64 = row.get(1);
            amount
        }).sum()
    }

    pub fn calculate_revenue_recognitions(&self, contract_num: i64) {
        // TODO: change it to use `?` symbol
        let contracts = self.db.find_contract(contract_num).unwrap();
        for row in contracts.iter() {
            let total_revenue: Money = row.get(2);
            let recognition_date: chrono::NaiveDate = row.get(3);
            let product_type: i64 = row.get(1);
            if product_type == 1 {
                // product_type == Spreadsheet
                let allocations = self.allocate(total_revenue, 3);
                self.db.insert_recognition(contract_num, allocations[0], recognition_date);
                self.db.insert_recognition(contract_num, allocations[1], recognition_date.add(Duration::days(60)));
                self.db.insert_recognition(contract_num, allocations[2], recognition_date.add(Duration::days(90)));
            } else if product_type == 2 {
                // product_type == Word Processor
                self.db.insert_recognition(contract_num, total_revenue, recognition_date);
            } else if product_type == 3 {
                // product_type == database
                let allocations = self.allocate(total_revenue, 3);
                self.db.insert_recognition(contract_num, allocations[0], recognition_date);
                self.db.insert_recognition(contract_num, allocations[1], recognition_date.add(Duration::days(30)));
                self.db.insert_recognition(contract_num, allocations[2], recognition_date.add(Duration::days(60)));
            }
        }
    }

    fn allocate(&self, money: Money, split: i64) -> Vec<Money> {
        let split = money / split;
        let mut money_bag: Vec<i64> = Vec::new();
        for _i in 0..split {
            money_bag.push(split);
        }

        money_bag
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use postgres::{Connection, TlsMode};

    #[test]
    fn test_recognized_revenue() {
        let conn = Connection::connect("postgresql://postgres@localhost:5432/poea_rs", TlsMode::None).unwrap();
        let db = Gateway::new(conn);
        let service = RecognitionService::new(db);

        let date = NaiveDate::from_ymd(2019, 3, 10);
        let result = service.recognized_revenue(23400, date);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_calculate_revenue_recognitions() {
        let conn = Connection::connect("postgresql://postgres@localhost:5432/poea_rs", TlsMode::None).unwrap();
        let db = Gateway::new(conn);
        let service = RecognitionService::new(db);

        service.calculate_revenue_recognitions(200);
        assert_eq!(1, 1);
    }
}