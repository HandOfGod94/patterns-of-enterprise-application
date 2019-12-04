Sequel.migration do
  change do
    create_table?(:product) do
      primary_key :id
      String :name
      String :type
    end
  end
end
