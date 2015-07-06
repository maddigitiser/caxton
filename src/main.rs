extern crate rocksdb;
extern crate getopts;

use getopts::Options;
use std::env;
use rocksdb::RocksDB;


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let db = RocksDB::open_default("./db").unwrap();

    let mut opts = Options::new();
    opts.optopt("w", "write", "start writing specs", "TEXT");
    opts.optflag("r", "read", "get a spec");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.opt_present("w") {
        let text_to_write = matches.opt_str("w");
        db.put(b"k1", b"test");
    }

    if matches.opt_present("r") {
        db.get(b"k1")
            .map( |value| {
                println!("retrieved value {}", value.to_utf8().unwrap())
            })
            .on_absent( || { println!("value not found") })
            .on_error( |e| { println!("operational problem encountered: {}", e) });
    }


    //db.put(b"my key", b"my value");
    /*
    db.get(b"my key")
        .map( |value| {
            println!("retrieved value {}", value.to_utf8().unwrap())
        })
        .on_absent( || { println!("value not found") })
        .on_error( |e| { println!("operational problem encountered: {}", e) });
    */
    //db.delete(b"my key");
    db.close();
}
