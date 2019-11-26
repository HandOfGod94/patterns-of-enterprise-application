use postgres::rows::Rows;
use postgres::{Connection, Error};

type Money = i64;

#[derive(Debug)]
pub struct Gateway {
    conn: Connection,
}

impl Gateway {
    const FIND_RECOGNITION_STATEMENT: &'static str = "SELECT * FROM revenueRecognition WHERE contract = $1 AND recognizedOn = $2";
    const FIND_CONTRACT_STATEMENT: &'static str = "SELECT * FROM contracts, product WHERE contracts.id = $1 AND product = product.id";
    const INSERT_RECOGNITION_STATEMENT: &'static str = "INSERT INTO revenueRecognition VALUES($1,$2,$3)";

    pub fn new(conn: Connection) -> Gateway {
        Gateway { conn }
    }

    pub fn find_recognition_for(&self, contract_id: i64, as_of: chrono::NaiveDate) -> Result<Rows, Error> {
        let stmt = self.conn.prepare(Gateway::FIND_RECOGNITION_STATEMENT)?;
        stmt.query(&[&contract_id, &as_of])
    }

    pub fn find_contract(&self, contract_id: i64) -> Result<Rows, Error> {
        let stmt = self.conn.prepare(Gateway::FIND_CONTRACT_STATEMENT)?;
        stmt.query(&[&contract_id])
    }

    pub fn insert_recognition(&self, contract_id: i64, amount: Money, as_of: chrono::NaiveDate) -> postgres::Result<u64> {
        let stmt = self.conn.prepare(Gateway::INSERT_RECOGNITION_STATEMENT)?;
        stmt.execute(&[&contract_id, &amount, &as_of])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use postgres::TlsMode;

    fn setup() -> Gateway {
        let conn = Connection::connect(
            "postgresql://postgres@localhost:5432/poea_rs",
            TlsMode::None,
        ).unwrap();
        Gateway::new(conn)
    }

    #[test]
    fn test_find_recognition_statement() {
        let gateway = setup();
        let date = NaiveDate::from_ymd(2019, 3, 10);
        let result = gateway.find_recognition_for(200, date).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_find_contract_statement() {
        let gateway = setup();
        let result = gateway.find_contract(200).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_insert_recognition() {
        let gateway = setup();
        let date = NaiveDate::from_ymd(2019, 3, 10);
        let result = gateway.insert_recognition(20, 300, date);
        assert_eq!(result.unwrap(), 1);
    }
}
