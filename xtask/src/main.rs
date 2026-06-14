use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("build") => build(),
        Some("run") => {
            let memory: Option<String> = parse_optional_str_arg(&args, "--memory");
            let window: bool = args.contains(&"--window".to_string());
            let gdb: bool = args.contains(&"--gdb".to_string());
            let numa: bool = args.contains(&"--numa".to_string());
            run(memory.as_deref(), window, gdb, numa);
        }
        _ => eprintln!("Usage: cargo xtask [build|run [--memory <size>] [--window] [--gdb] [--numa]]"),
    }
}

fn parse_optional_str_arg(args: &[String], flag: &str) -> Option<String> {
    let pos: usize = args.iter().position(|a| a == flag)?;
    Some(args.get(pos + 1).unwrap_or_else(|| panic!("missing value for {flag}")).clone())
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

fn run(memory: Option<&str>, window: bool, gdb: bool, numa: bool) {
    build();

    let kernel: std::path::PathBuf = workspace_root()
        .join("target/riscv64-ferrum/release/ferrum");

    let display_args: &[&str] = if window {
        &["-serial", "vc:640x480", "-display", "cocoa,zoom-to-fit=on", "-monitor", "none"]
    } else {
        &["-nographic"]
    };

    let gdb_args: &[&str] = if gdb {
        eprintln!("GDB stub listening on port 1234 — connect with:");
        eprintln!("  riscv64-unknown-elf-gdb target/riscv64-ferrum/release/ferrum");
        eprintln!("  (gdb) target remote :1234");
        &["-s", "-S"]
    } else {
        &[]
    };

    let mut cmd: Command = Command::new("qemu-system-riscv64");
    cmd.args(["-machine", "virt", "-kernel", kernel.to_str().unwrap()]);

    if numa {
        let node_mem: &str = memory.unwrap_or("128M");
        let total_mem: String = format!("{}M", parse_megabytes(node_mem) * 2);
        cmd.args(["-m", &total_mem]);
        cmd.args(["-smp", "2"]);
        cmd.args(["-object", &format!("memory-backend-ram,size={node_mem},id=mem0")]);
        cmd.args(["-object", &format!("memory-backend-ram,size={node_mem},id=mem1")]);
        cmd.args(["-numa", "node,nodeid=0,memdev=mem0,cpus=0"]);
        cmd.args(["-numa", "node,nodeid=1,memdev=mem1,cpus=1"]);
    } else {
        cmd.args(["-m", memory.unwrap_or("128M")]);
    }

    cmd.args(display_args).args(gdb_args);

    let status: std::process::ExitStatus = cmd.status().expect("failed to run qemu-system-riscv64");
    std::process::exit(status.code().unwrap_or(1));
}

fn parse_megabytes(s: &str) -> u64 {
    let s: &str = s.trim_end_matches(|c: char| c.is_alphabetic());
    s.parse::<u64>().unwrap_or(128)
}

fn workspace_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has no parent")
        .to_path_buf()
}
