use postgres::rows::Rows;
use postgres::stmt::Statement;
use postgres::{Connection, Error};

#[derive(Debug)]
pub struct Gateway {
    conn: Connection,
}

impl Gateway {
    pub fn new(conn: Connection) -> Gateway {
        Gateway { conn }
    }

    pub fn find_recognition_for(&self, contract_id: i64, as_of: chrono::NaiveDate) -> Result<Rows, Error> {
        let stmt = self.find_recognition_statement()?;
        stmt.query(&[&contract_id, &as_of])
    }

    pub fn find_contract(&self, contract_id: i64) -> Result<Rows, Error> {
        let stmt = self.find_contract_statement()?;
        stmt.query(&[&contract_id])
    }

    fn find_recognition_statement(&self) -> postgres::Result<Statement<'_>> {
        self.conn
            .prepare("SELECT * FROM revenueRecognition WHERE contract = $1 AND recognizedOn = $2")
    }

    fn find_contract_statement(&self) -> postgres::Result<Statement<'_>> {
        self.conn.prepare("SELECT * FROM contracts, product WHERE contracts.id = $1 AND product = product.id")
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

}
