fn main() {
    println!(
        "cargo::rustc-env=BACKEND_URL={}",
        std::env::var("BACKEND_URL")
            .unwrap_or("http://average-benchmark-api.rustlang-es.org".to_string())
    );

    println!("cargo:rerun-if-env-changed=BACKEND_URL");
}
