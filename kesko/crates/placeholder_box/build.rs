use std::path::Path;

fn main() {
    flatc_rust::run(flatc_rust::Args {
        inputs: &[Path::new("definitions/spawn.fbs"), Path::new("definitions/clear.fbs")],
        out_dir: Path::new("./src"),
        ..Default::default()
    })
    .expect("flatc");
}