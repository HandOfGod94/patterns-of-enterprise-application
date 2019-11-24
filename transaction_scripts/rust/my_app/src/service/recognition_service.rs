use crate::gateway::Gateway;
use chrono::NaiveDate;

#[derive(Debug)]
pub struct RecognitionService {
    gateway: Gateway
}

type Money = i64;

impl RecognitionService {
    pub fn new(gateway: Gateway) -> RecognitionService {
        RecognitionService {
            gateway
        }
    }

    pub fn recognized_revenue(&self, contract_num: i64, as_of: NaiveDate) -> Money {
        let rows = self.gateway.find_recognition_for(contract_num, as_of).unwrap();

        rows.iter().map(|row| {
            let amount: i64 = row.get(1);
            amount
        }).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use postgres::{Connection, TlsMode};

    #[test]
    fn test_recognized_revenue() {
        let conn = Connection::connect("postgresql://postgres@localhost:5432/poea_rs", TlsMode::None).unwrap();
        let gateway = Gateway::new(conn);

        let service = RecognitionService::new(gateway);
        let date = NaiveDate::from_ymd(2019, 3, 10);
        let result = service.recognized_revenue(23400, date);

        assert_eq!(result, 0);
    }
}