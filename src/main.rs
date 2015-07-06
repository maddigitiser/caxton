extern crate rocksdb;
use rocksdb::RocksDB;

fn main() {
    let db = RocksDB::open_default("./db").unwrap();
    db.put(b"my key", b"my value");
    db.get(b"my key")
        .map( |value| {
            println!("retrieved value {}", value.to_utf8().unwrap())
        })
        .on_absent( || { println!("value not found") })
        .on_error( |e| { println!("operational problem encountered: {}", e) });

    db.delete(b"my key");
    db.close();
}
