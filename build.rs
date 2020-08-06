use std::{fs::File, io::Write, path::Path};
use witx::Documentation;

fn main() {
    let witx_path = Path::new("witx/networking.witx");
    let doc = witx::load(&[&witx_path]).unwrap();

    let docs_path = Path::new("docs.md");

    let mut file = File::create(&docs_path)
        .expect("create output file");
    file.write_all(doc.to_md().as_bytes())
        .expect("write output file");

    println!("cargo:rerun-if-changed={}", witx_path.display());
    println!(
        "cargo:rerun-if-changed={}",
        witx_path.with_file_name("typenames.witx").display(),
    );
}
