use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("build") => build(),
        Some("run") => {
            let runner: std::path::PathBuf = parse_arg(&args, "--runner");
            run(&runner);
        }
        _ => eprintln!("Usage: cargo xtask [build|run --runner <path>]"),
    }
}

fn parse_arg(args: &[String], flag: &str) -> std::path::PathBuf {
    let pos: usize = args.iter().position(|a| a == flag)
        .unwrap_or_else(|| panic!("missing {flag} <path>"));
    std::path::PathBuf::from(args.get(pos + 1).unwrap_or_else(|| panic!("missing value for {flag}")))
}

fn build() {
    let linker_script: std::path::PathBuf = workspace_root().join("ferrum/linker.lds");
    let rustflags: String = format!("-C link-arg=-T{}", linker_script.display());

    let status: std::process::ExitStatus = Command::new("cargo")
        .args([
            "+nightly",
            "build",
            "--release",
            "--package",
            "ferrum",
            "--target",
            "ferrum/riscv64-ferrum.json",
            "-Z",
            "build-std=core,compiler_builtins",
            "-Z",
            "build-std-features=compiler-builtins-mem",
            "-Z",
            "json-target-spec",
        ])
        .env("RUSTFLAGS", rustflags)
        .current_dir(workspace_root())
        .status()
        .expect("failed to run cargo build");

    if !status.success() {
        std::process::exit(1);
    }
}

fn run(runner: &std::path::Path) {
    build();

    let kernel: std::path::PathBuf = workspace_root()
        .join("target/riscv64-ferrum/release/ferrum");

    let status: std::process::ExitStatus = Command::new("cargo")
        .args(["xtask", "run", "--kernel", kernel.to_str().unwrap()])
        .current_dir(runner)
        .status()
        .expect("failed to run runner xtask");

    std::process::exit(status.code().unwrap_or(1));
}

fn workspace_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has no parent")
        .to_path_buf()
}
