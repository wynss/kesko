use std::path::Path;

fn main() {
    flatc_rust::run(flatc_rust::Args {
        lang: "python",
        inputs: &[
            Path::new("../kesko/crates/kesko_urdf/definitions/urdf.fbs"), 
            Path::new("../kesko/crates/placeholder_box/definitions/spawn.fbs"),
            Path::new("../kesko/crates/placeholder_box/definitions/clear.fbs"),
            Path::new("../kesko/crates/sdf_model_loader/definitions/spawn.fbs"),
        ],
        out_dir: Path::new("./messages"),
        // extra: &["--python-no-type-prefix-suffix", "--python-typing"],
        ..Default::default()
    })
    .expect("flatc");
}