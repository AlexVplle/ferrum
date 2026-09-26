use std::process::Command;

mod arch;
mod run_config;
use arch::Arch;
use run_config::RunConfig;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("build") => {
            build(&Arch::from_args(&args), args.contains(&"--debug".to_string()));
        }
        Some("run") => {
            run(RunConfig::from_args(&args));
        }
        Some("test") => {
            let miri: bool = args.contains(&"--miri".to_string());
            let loom: bool = args.contains(&"--loom".to_string());
            test(miri, loom);
        }
        _ => eprintln!("Usage: cargo xtask [build [--arch <arch>] [--debug]|run [--arch <arch>] [--memory <size>] [--window] [--gdb] [--debug] [--numa <n>] [--smp <n>] [--distance src,dst,value]...|test [--miri|--loom]]"),
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

pub fn parse_repeated_str_arg(args: &[String], flag: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut i: usize = 0;
    while i < args.len() {
        if args[i] == flag {
            if let Some(value) = args.get(i + 1) {
                result.push(value.clone());
                i += 2;
                continue;
            }
        }
        i += 1;
    }
    result
}

pub fn parse_optional_str_arg(args: &[String], flag: &str) -> Option<String> {
    let position: usize = args.iter().position(|arg: &String| arg == flag)?;
    Some(args.get(position + 1).unwrap_or_else(|| panic!("missing value for {flag}")).clone())
}

fn build(arch: &Arch, debug: bool) {
    let root: std::path::PathBuf = workspace_root();
    let linker_script: std::path::PathBuf = root.join(arch.linker_script());
    let rustflags: String = format!("-C link-arg=-T{} -C link-arg=-pie", linker_script.display());

    let target_json: String = arch.target_json();
    let mut cargo_args: Vec<&str> = vec![
        "+nightly",
        "build",
        "--package",
        "ferrum",
        "--target",
        &target_json,
        "-Z",
        "build-std=core,alloc,compiler_builtins",
        "-Z",
        "build-std-features=compiler-builtins-mem",
        "-Z",
        "json-target-spec",
    ];

    if !debug {
        cargo_args.push("--release");
    }

    let status: std::process::ExitStatus = Command::new("cargo")
        .args(&cargo_args)
        .env("RUSTFLAGS", rustflags)
        .current_dir(root)
        .status()
        .expect("failed to run cargo build");

    if !status.success() {
        std::process::exit(1);
    }
}

fn run(config: RunConfig) {
    build(&config.arch, config.debug);

    let profile: &str = if config.debug { "debug" } else { "release" };
    let target_dir: String = config.arch.target_dir();
    let kernel: std::path::PathBuf = workspace_root()
        .join(format!("target/{target_dir}/{profile}/ferrum"));

    let display_args: &[&str] = if config.window {
        &["-serial", "vc:640x480", "-display", "cocoa,zoom-to-fit=on", "-monitor", "none"]
    } else {
        &["-nographic"]
    };

    let gdb_args: &[&str] = if config.gdb {
        let elf: String = if config.debug {
            format!("target/{target_dir}/debug/ferrum")
        } else {
            format!("target/{target_dir}/release/ferrum")
        };
        eprintln!("GDB stub listening on port 1234 - connect with:");
        eprintln!("  {} {elf}", config.arch.gdb_prefix());
        eprintln!("  (gdb) target remote :1234");
        &["-s", "-S"]
    } else {
        &[]
    };

    let mut cmd: Command = Command::new(config.arch.qemu_binary());
    cmd.args(["-machine", config.arch.qemu_machine(), "-kernel", kernel.to_str().unwrap()]);

    if let Some(numa) = config.numa {
        let node_mem: &str = config.memory.as_deref().unwrap_or("128M");
        let total_mem: String = format!("{}M", parse_megabytes(node_mem) * numa as u64);
        let total_cpus: usize = config.smp.unwrap_or(numa);
        cmd.args(["-m", &total_mem]);
        cmd.args(["-smp", &total_cpus.to_string()]);
        for i in 0..numa {
            cmd.args(["-object", &format!("memory-backend-ram,size={node_mem},id=mem{i}")]);
        }
        let cpus_per_node: usize = total_cpus / numa;
        let remainder: usize = total_cpus % numa;
        let mut cpu_offset: usize = 0;
        for i in 0..numa {
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
        for distance in &config.distances {
            cmd.args(["-numa", &format!("dist,{distance}")]);
        }
    } else {
        cmd.args(["-m", config.memory.as_deref().unwrap_or("128M")]);
        if let Some(smp) = config.smp {
            cmd.args(["-smp", &smp.to_string()]);
        }
    }

    cmd.args(display_args).args(gdb_args);

    let status: std::process::ExitStatus = cmd.status().expect("failed to run qemu");
    std::process::exit(status.code().unwrap_or(1));
}

fn parse_megabytes(mem: &str) -> u64 {
    let digits: &str = mem.trim_end_matches(|c: char| c.is_alphabetic());
    digits.parse::<u64>().unwrap_or(128)
}

fn workspace_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has no parent")
        .to_path_buf()
}
