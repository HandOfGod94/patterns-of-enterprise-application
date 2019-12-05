class Gateway
  def initialize(db)
    @db = db
  end

  def find_recognition_for(contract_id, as_of)
    @db.query(FIND_RECOGNITION_STATEMENT, contract_id, as_of)
  end

  private

  FIND_RECOGNITION_STATEMENT = "SELECT * FROM revenue_recognition WHERE contract_id = ? AND recognized_on = ?"

end
