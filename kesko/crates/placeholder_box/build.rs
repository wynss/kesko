use std::path::Path;

fn main() {
    flatc_rust::run(flatc_rust::Args {
        inputs: &[Path::new("definitions/placeholder_box.fbs")],
        out_dir: Path::new("./src"),
        ..Default::default()
    })
    .expect("flatc");
}