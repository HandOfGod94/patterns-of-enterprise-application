Sequel.migration do
  change do
    create_table?(:contract) do
      primary_key :id
      foreign_key :product_id, :product
      BigDecimal :revenue
      Date :date_signed
    end
  end
end
