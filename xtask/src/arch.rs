use crate::parse_optional_str_arg;

const VALID: &[&str] = &["riscv64", "riscv32", "aarch64", "x86_64"];

pub struct Arch(String);

impl Arch {
    pub fn from_args(args: &[String]) -> Self {
        let name = parse_optional_str_arg(args, "--arch")
            .unwrap_or_else(|| "riscv64".to_string());
        if !VALID.contains(&name.as_str()) {
            eprintln!("Unknown arch: {name}. Valid: {}", VALID.join(", "));
            std::process::exit(1);
        }
        Self(name)
    }

    pub fn target_json(&self) -> String {
        format!("ferrum/targets/{}-ferrum.json", self.0)
    }

    pub fn linker_script(&self) -> String {
        format!("ferrum/linkers/{}.lds", self.0)
    }

    pub fn target_dir(&self) -> String {
        format!("{}-ferrum", self.0)
    }

    pub fn qemu_binary(&self) -> String {
        format!("qemu-system-{}", self.0)
    }

    pub fn qemu_machine(&self) -> &'static str {
        match self.0.as_str() {
            "x86_64" => "q35",
            _ => "virt",
        }
    }

    pub fn gdb_prefix(&self) -> String {
        format!("{}-unknown-elf-gdb", self.0)
    }
}
