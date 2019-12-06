require 'sequel'
require 'sqlite3'

RSpec.describe Gateway do
  describe '.find_recognition_for' do
    subject { described_class.new(db).find_recognition_for(contract_id, as_of) }

    let!(:db) { SQLite3::Database.open 'my_app.db' }
    let(:sequel) { Sequel.sqlite('my_app.db') }

    before do
      products = sequel[:product]
      p_id1 = products.insert(name: 'Word Processor', type: 'W')
      p_id2 = products.insert(name: 'Spreadsheet', type: 'S')
      p_id3 = products.insert(name: 'Database', type: 'D')

      contracts = sequel[:contract]
      c_id1 = contracts.insert(product_id: p_id1, revenue: 7890, date_signed: Date.today)
      c_id2 = contracts.insert(product_id: p_id2, revenue: 6540, date_signed: Date.today)
      c_id3 = contracts.insert(product_id: p_id3, revenue: 3210, date_signed: Date.today)

      revenue_recognitions = sequel[:revenue_recognition]
      rr_id1 = revenue_recognitions.insert(contract_id: c_id1, amount: 8909, recognized_on: Date.today + 1)
      rr_id2 = revenue_recognitions.insert(contract_id: c_id2, amount: 4430, recognized_on: Date.today + 2)
      rr_id3 = revenue_recognitions.insert(contract_id: c_id3, amount: 7860, recognized_on: Date.today + 4 )


    end

    context 'when values are nil' do
      let(:contract_id) { nil }
      let(:as_of) { nil }

      it 'returns 0 rows' do
        expect(subject.count).to eq(0)
      end
    end

    context 'when values are not present' do
      let(:contract_id) { 555 }
      let(:as_of) { Date.today.to_s }

      it 'returns 0 rows' do
        expect(subject.count).to eq(0)
      end
    end

    context 'when values are present' do
      let(:contract_id) { 2 }
      let(:as_of) { Date.today.to_s }

      it 'returns list of rows' do
        expect(subject.count).to eq(2)
      end
    end

    after do
      products = sequel[:product]
      contracts = sequel[:contract]

      products.truncate
      contracts.truncate
    end

  end
end
