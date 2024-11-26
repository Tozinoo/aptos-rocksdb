use rocksdb::{Options, DB, ColumnFamilyDescriptor, IteratorMode};

fn main() {
    let db_path = "/home/hs/Documents/mainnet/data/db/state_kv_db/shard_0";

    // RocksDB 옵션 설정
    let mut opts = Options::default();
    opts.create_if_missing(false);

    // 컬럼 패밀리 정의
    let cfs = vec![
        ColumnFamilyDescriptor::new("default", Options::default()),
        ColumnFamilyDescriptor::new("db_metadata", Options::default()),
        ColumnFamilyDescriptor::new("stale_state_value_index_by_key_hash", Options::default()),
        ColumnFamilyDescriptor::new("state_value_by_key_hash", Options::default()),
        ColumnFamilyDescriptor::new("state_value_index", Options::default()),
    ];

    // RocksDB 열기
    let db = DB::open_cf_descriptors(&opts, db_path, cfs).expect("Failed to open RocksDB");

    // default 컬럼 패밀리 데이터 읽기
    let cf = db.cf_handle("default").expect("Column family not found");
    let iter = db.iterator_cf(cf, IteratorMode::Start);

    for item in iter.take(100) { // 최대 100개의 키-값만 읽기
        match item {
            Ok((key, value)) => {
                println!(
                    "Key: {:?}, Value: {:?}",
                    String::from_utf8_lossy(&key),
                    String::from_utf8_lossy(&value)
                );
            }
            Err(e) => {
                eprintln!("Error reading RocksDB: {}", e);
            }
        }
    }
}
