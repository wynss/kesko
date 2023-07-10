use std::path::Path;

fn main() {
    flatc_rust::run(flatc_rust::Args {
        lang: "python",
        inputs: &[
            Path::new("../kesko/crates/kesko_urdf/definitions/urdf.fbs"), 
            Path::new("../kesko/crates/placeholder_box/definitions/placeholder_box.fbs")
        ],
        out_dir: Path::new("./messages"),
        ..Default::default()
    })
    .expect("flatc");
}