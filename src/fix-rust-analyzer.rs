// use std::fs;
use glob::glob;

fn main() {
    use std::env;
    let out_dir = env::var("RUSTC").expect("RUSTC not there");
    println!("{}", out_dir);
    let mut crates = String::new();
    for e in glob("./exercises/**/*").expect("Glob failed to read pattern") {
        let path = e
            .expect("Unable to extract path")
            .to_string_lossy()
            .to_string();
        if let Some((_, ext)) = path.split_once(".") {
            if ext == "rs" {
                crates.push_str(
                    format!(
                        r#"{{
    "root_module": "{}",
    "edition": "2021",
    "deps": []
}}, 
"#,
                        &path
                    )
                    .as_str(),
                );
            }
        }
    }
    let project = format!(
        r#"{{
	"sysroot_src": "/home/jacko/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library",
	"crates": [
            {}
            ]
        }}"#,
        crates
    );
    std::fs::write("./rust-project.json", project).expect("Failed to write file");
    // let paths = fs::File::("./exercises").unwrap();

    // for path in paths {
    // println!("Name: {}", path.unwrap().path().display());
    // }
}
