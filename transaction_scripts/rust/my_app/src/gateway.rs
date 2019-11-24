use postgres::rows::Rows;
use postgres::stmt::Statement;
use postgres::{Connection, Error};

pub struct Gateway {
    conn: Connection,
}

impl Gateway {
    pub fn new(conn: Connection) -> Gateway {
        Gateway { conn }
    }

    pub fn find_recognition_for(
        &self,
        contract_id: i64,
        as_of: chrono::NaiveDate,
    ) -> Result<Rows, Error> {
        let stmt = self.find_recognition_statement()?;
        stmt.query(&[&contract_id, &as_of])
    }

    fn find_recognition_statement(&self) -> postgres::Result<Statement<'_>> {
        self.conn
            .prepare("SELECT * FROM revenueRecognition WHERE contract = $1 AND recognizedOn = $2")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use postgres::TlsMode;

    #[test]
    fn test_find_recognition_statement() {
        let conn = Connection::connect(
            "postgresql://postgres@localhost:5432/poea_rs",
            TlsMode::None,
        )
        .unwrap();
        let gateway = Gateway::new(conn);
        let date = NaiveDate::from_ymd(2019, 3, 10);
        let result = gateway.find_recognition_for(200, date).unwrap();
        assert_eq!(result.len(), 0);
    }
}
