use std::path::Path;

fn main() {
    flatc_rust::run(flatc_rust::Args {
        lang: "python",
        inputs: &[Path::new("../kesko/crates/kesko_urdf/definitions/urdf.fbs")],
        out_dir: Path::new("./messages"),
        ..Default::default()
    })
    .expect("flatc");
}