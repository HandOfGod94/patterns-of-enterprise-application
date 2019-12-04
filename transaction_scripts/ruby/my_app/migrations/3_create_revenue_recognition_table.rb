Sequel.migration do
  change do
    create_table?(:revenue_recognition) do
      foreign_key :contract_id, :contract
      BigDecimal :amount
      Date :recognized_on
      primary_key [:contract_id, :amount]
    end
  end
end
