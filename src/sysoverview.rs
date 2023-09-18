use crossterm::style::{Color, Stylize};
use display_info::DisplayInfo;
use std::{
    collections::HashSet,
    env,
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};
use sysinfo::{CpuExt, System, SystemExt};

enum SysField {
    HostName(String),
    Underline(String),
    OsName(String),
    Host(String),
    KernelVersion(String),
    Uptime(String),
    Shell(String),
    Resolutions(Vec<String>),
    WM(String),
    Terminal(String),
    CPUs(Vec<String>),
    GPUs(Vec<String>),
    Memory(String),
}

pub struct SysOverview {
    data: Vec<SysField>,
}

impl SysOverview {
    pub fn new() -> Self {
        let sys = System::new_all();
        let host_name = Self::get_host_name(&sys);
        let underline_length = host_name.len();
        let data = vec![
            SysField::HostName(host_name),
            SysField::Underline(Self::get_underline(underline_length)),
            SysField::Host(Self::get_host(&sys)),
            SysField::OsName(Self::get_os_name(&sys)),
            SysField::KernelVersion(Self::get_kernel_version(&sys)),
            SysField::Uptime(Self::get_uptime(&sys)),
            SysField::Shell(Self::get_shell()),
            SysField::Resolutions(Self::get_resolutions()),
            SysField::WM(Self::get_wm()),
            SysField::Terminal(Self::get_terminal()),
            SysField::CPUs(Self::get_cpus(&sys)),
            SysField::GPUs(Self::get_gpu()),
            SysField::Memory(Self::get_memory(&sys)),
        ];
        Self { data }
    }

    fn get_host_name(sys: &System) -> String {
        sys.host_name().unwrap_or_default()
    }

    fn get_underline(length: usize) -> String {
        "-".repeat(length)
    }

    fn get_os_name(sys: &System) -> String {
        sys.long_os_version().unwrap_or_default() // TODO: MORE DETAILS
    }

    fn get_host(sys: &System) -> String {
        sys.distribution_id()
    }

    fn get_kernel_version(sys: &System) -> String {
        sys.kernel_version().unwrap_or_default()
    }

    // TODO: FIX
    fn get_uptime(sys: &System) -> String {
        let mut uptime_seconds = sys.uptime();
        let mut result: Vec<String> = vec![];

        if uptime_seconds > 60 * 60 * 24 {
            let days = uptime_seconds / (60 * 60 * 24);
            uptime_seconds = uptime_seconds.saturating_sub(days * 60 * 60 * 24);
            result.push(format!("{} days", days));
        }

        if uptime_seconds > 60 * 60 {
            let hours = uptime_seconds / (60 * 60);
            uptime_seconds = uptime_seconds.saturating_sub(hours * 60 * 60);
            result.push(format!("{} hours", hours));
        }

        if uptime_seconds > 60 {
            let mins = uptime_seconds / 60;
            uptime_seconds = uptime_seconds.saturating_sub(mins * 60);
            result.push(format!("{} mins", mins));
        }

        if uptime_seconds > 0 {
            let seconds = uptime_seconds;
            result.push(format!("{} seconds", seconds));
        }

        result.join(", ")
    }

    fn get_shell() -> String {
        env::var("SHELL").unwrap_or(String::from("Unable to get shell"))
    }

    fn get_resolutions() -> Vec<String> {
        let mut resolutions: Vec<String> = Vec::new();
        let display_infos = DisplayInfo::all().unwrap();

        if display_infos.is_empty() {
            resolutions.push(String::from("Unable to get display!"));
        } else {
            for display_info in display_infos.iter() {
                resolutions.push(format!("{}x{}", display_info.width, display_info.height));
            }
        }
        resolutions
    }

    fn get_wm() -> String {
        env::var("TERM_PROGRAM").unwrap_or(String::from("Unable to get WM"))
    }

    fn get_terminal() -> String {
        env::var("LC_TERMINAL").unwrap_or(String::from("Unable to get terminal"))
    }

    fn get_cpus(sys: &System) -> Vec<String> {
        let mut cpus = HashSet::new();
        for cpu in sys.cpus() {
            cpus.insert(cpu.brand());
        }

        if cpus.is_empty() {
            vec![String::from("Unable to get CPU!")]
        } else {
            cpus.iter().map(|cpu| String::from(*cpu)).collect()
        }
    }

    // TODO: Improve performance of this function
    fn get_gpu() -> Vec<String> {
        // Run the ioreg command to query GPU information
        let output = Command::new("system_profiler")
            .arg("SPDisplaysDataType")
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to execute ioreg")
            .stdout
            .expect("Failed to capture stdout");

        let reader = BufReader::new(output);
        let mut gpus: Vec<String> = Vec::new();

        let mut name = String::from("Unable to get GPU");
        for line in reader.lines() {
            if let Ok(line) = line {
                if line.contains("Chipset Model") {
                    name = line.split_once(": ").unwrap().1.to_string();
                } else if line.contains("VRAM") {
                    let vram = line.split_once(": ").unwrap().1;
                    let gpu_name = format!("{} ({} VRAM)", name, vram);
                    gpus.push(gpu_name);
                }
            }
        }

        gpus
    }

    fn bytes_to_mebibytes(bytes: u64) -> u64 {
        const MEBIBYTE: f64 = 1024.0 * 1024.0;
        (bytes as f64 / MEBIBYTE) as u64
    }

    fn get_memory(sys: &System) -> String {
        let total_memory = Self::bytes_to_mebibytes(sys.total_memory());
        let used_memory = Self::bytes_to_mebibytes(sys.used_memory());

        format!("{}MiB / {}MiB", used_memory, total_memory)
    }

    fn get_name(&self, sys_field: &SysField) -> String {
        match sys_field {
            SysField::OsName(_) => String::from("OS"),
            SysField::HostName(_) => String::from("HostName"),
            SysField::Underline(_) => String::from("HostUnderline"),
            SysField::Host(_) => String::from("Host"),
            SysField::KernelVersion(_) => String::from("Kernel"),
            SysField::Uptime(_) => String::from("Uptime"),
            SysField::Shell(_) => String::from("Shell"),
            SysField::WM(_) => String::from("WM"),
            SysField::Terminal(_) => String::from("Terminal"),
            SysField::Memory(_) => String::from("Memory"),
            SysField::CPUs(_) => String::from("CPU"),
            SysField::GPUs(_) => String::from("GPU"),
            SysField::Resolutions(_) => String::from("Resolution"),
        }
    }

    pub fn output_strs(&self, primary: Color, secondary: Color) -> Vec<String> {
        let mut formatted_strings: Vec<String> = vec![];

        for sys_field in &self.data {
            let name = self.get_name(&sys_field);
            match sys_field {
                SysField::HostName(field) | SysField::Underline(field) => {
                    formatted_strings.push(format!("{}", field.clone().with(primary)));
                }
                SysField::OsName(field)
                | SysField::Host(field)
                | SysField::KernelVersion(field)
                | SysField::Uptime(field)
                | SysField::Shell(field)
                | SysField::WM(field)
                | SysField::Terminal(field)
                | SysField::Memory(field) => formatted_strings.push(format!(
                    "{}: {}",
                    name.with(primary),
                    field.clone().with(secondary)
                )),
                SysField::Resolutions(fields) | SysField::CPUs(fields) | SysField::GPUs(fields) => {
                    let len = fields.len();
                    if len == 1 {
                        formatted_strings.push(format!(
                            "{}: {}",
                            name.clone().with(primary),
                            fields[0].clone().with(secondary)
                        ));
                    } else {
                        for (i, field) in fields.iter().enumerate() {
                            formatted_strings.push(format!(
                                "{} ({}): {}",
                                name.clone().with(primary),
                                (i + 1).to_string().with(primary),
                                field.clone().with(secondary)
                            ));
                        }
                    }
                }
            };
        }

        formatted_strings
    }
}
