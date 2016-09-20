extern crate nss_certdata_parser;

use std::fs::File;
use std::env::args;
use std::io::BufReader;

use nss_certdata_parser::reader::ObjectIter;
use nss_certdata_parser::syntax::Value;

fn main() {
    for path in args().skip(1) {
        println!("[");
        for res_obj in ObjectIter::new(BufReader::new(File::open(path).unwrap())) {
            println!("   {{");
            for (k, v) in res_obj.unwrap() {
                let vj = match v {
                    Value::Token(t, vv) => format!("{{ type: {:?}, value: {:?} }}", t, vv),
                    Value::String(s) => format!("{:?}", s),
                    // base64 would be a more usual (and compact) way
                    // to embed an octet stream into JSON, but that
                    // would need additional crate dependences.
                    Value::Binary(b) => format!("{:?}", b).trim_matches('"').to_owned(),
                };
                println!("      {:?}: {},", k, vj);
            }
            println!("   }},");
        }
        println!("]");
    }
}
