use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    write!(
        &mut file,
        "static KEYWORDS: phf::Set<&'static str> = {}",
        phf_codegen::Set::new()
            .entry("loop")
            .build()
    )
    .unwrap();
    write!(&mut file, ";\n").unwrap();
}