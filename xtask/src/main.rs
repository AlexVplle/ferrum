use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("build") => build(),
        Some("run") => {
            let memory: Option<String> = parse_optional_str_arg(&args, "--memory");
            let window: bool = args.contains(&"--window".to_string());
            let gdb: bool = args.contains(&"--gdb".to_string());
            let numa: Option<usize> = parse_optional_str_arg(&args, "--numa")
                .map(|s| s.parse::<usize>().unwrap_or_else(|_| panic!("--numa requires a number")));
            let smp: Option<usize> = parse_optional_str_arg(&args, "--smp")
                .map(|s| s.parse::<usize>().unwrap_or_else(|_| panic!("--smp requires a number")));
            run(memory.as_deref(), window, gdb, numa, smp);
        }
        Some("test") => {
            let miri: bool = args.contains(&"--miri".to_string());
            let loom: bool = args.contains(&"--loom".to_string());
            test(miri, loom);
        }
        _ => eprintln!("Usage: cargo xtask [build|run [--memory <size>] [--window] [--gdb] [--numa <n>] [--smp <n>]|test [--miri|--loom]]"),
    }
}

fn test(miri: bool, loom: bool) {
    const EXCLUDE: &[&str] = &["--exclude", "ferrum", "--exclude", "xtask"];
    if loom {
        run_test_cmd(&["test", "-p", "ferrum-core"], &[], Some("--cfg loom"), None);
    } else if miri {
        run_test_cmd(&["miri", "test", "--workspace"], EXCLUDE, None, Some("-Zmiri-tree-borrows"));
    } else {
        run_test_cmd(&["test", "--workspace"], EXCLUDE, None, None);
        run_test_cmd(&["test", "-p", "ferrum-core"], &[], Some("--cfg loom"), None);
        run_test_cmd(&["miri", "test", "--workspace"], EXCLUDE, None, Some("-Zmiri-tree-borrows"));
    }
}

fn run_test_cmd(subargs: &[&str], extra: &[&str], rustflags: Option<&str>, miriflags: Option<&str>) {
    let mut cmd: Command = Command::new("cargo");
    cmd.arg("+nightly").args(subargs).args(extra).current_dir(workspace_root());
    if let Some(flags) = rustflags {
        cmd.env("RUSTFLAGS", flags);
    }
    if let Some(flags) = miriflags {
        cmd.env("MIRIFLAGS", flags);
    }
    let status: std::process::ExitStatus = cmd.status().expect("failed to run cargo");
    if !status.success() {
        std::process::exit(1);
    }
}

fn parse_optional_str_arg(args: &[String], flag: &str) -> Option<String> {
    let pos: usize = args.iter().position(|a| a == flag)?;
    Some(args.get(pos + 1).unwrap_or_else(|| panic!("missing value for {flag}")).clone())
}

fn build() {
    let linker_script: std::path::PathBuf = workspace_root().join("ferrum/linker.lds");
    let rustflags: String = format!("-C link-arg=-T{} -C link-arg=-pie", linker_script.display());

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

fn run(memory: Option<&str>, window: bool, gdb: bool, numa: Option<usize>, smp: Option<usize>) {
    build();

    let kernel: std::path::PathBuf = workspace_root()
        .join("target/riscv64-ferrum/release/ferrum");

    let display_args: &[&str] = if window {
        &["-serial", "vc:640x480", "-display", "cocoa,zoom-to-fit=on", "-monitor", "none"]
    } else {
        &["-nographic"]
    };

    let gdb_args: &[&str] = if gdb {
        eprintln!("GDB stub listening on port 1234 - connect with:");
        eprintln!("  riscv64-unknown-elf-gdb target/riscv64-ferrum/release/ferrum");
        eprintln!("  (gdb) target remote :1234");
        &["-s", "-S"]
    } else {
        &[]
    };

    let mut cmd: Command = Command::new("qemu-system-riscv64");
    cmd.args(["-machine", "virt", "-kernel", kernel.to_str().unwrap()]);

    if let Some(n) = numa {
        let node_mem: &str = memory.unwrap_or("128M");
        let total_mem: String = format!("{}M", parse_megabytes(node_mem) * n as u64);
        let total_cpus: usize = smp.unwrap_or(n);
        cmd.args(["-m", &total_mem]);
        cmd.args(["-smp", &total_cpus.to_string()]);
        for i in 0..n {
            cmd.args(["-object", &format!("memory-backend-ram,size={node_mem},id=mem{i}")]);
        }
        let cpus_per_node: usize = total_cpus / n;
        let remainder: usize = total_cpus % n;
        let mut cpu_offset: usize = 0;
        for i in 0..n {
            let count: usize = cpus_per_node + if i < remainder { 1 } else { 0 };
            let start: usize = cpu_offset;
            let end: usize = cpu_offset + count - 1;
            let cpus_arg: String = if start == end {
                format!("cpus={start}")
            } else {
                format!("cpus={start}-{end}")
            };
            cmd.args(["-numa", &format!("node,nodeid={i},memdev=mem{i},{cpus_arg}")]);
            cpu_offset += count;
        }
    } else {
        cmd.args(["-m", memory.unwrap_or("128M")]);
        if let Some(s) = smp {
            cmd.args(["-smp", &s.to_string()]);
        }
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
