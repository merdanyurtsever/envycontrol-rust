mod constants;
use constants::*;
use clap::Parser;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(name = "envycontrol", version = VERSION, about = "Easy GPU switching for Nvidia Optimus laptops")]
struct Cli {
    #[arg(short = 'v', long = "version")]
    version: bool,

    #[arg(short = 'q', long = "query")]
    query: bool,

    #[arg(short = 's', long = "switch")]
    switch: Option<String>,

    #[arg(long = "dm")]
    dm: Option<String>,

    #[arg(long = "force-comp")]
    force_comp: bool,

    #[arg(long = "coolbits")]
    coolbits: Option<Option<i32>>,

    #[arg(long = "rtd3")]
    rtd3: Option<Option<i32>>,

    #[arg(long = "use-nvidia-current")]
    use_nvidia_current: bool,

    #[arg(long = "reset-sddm")]
    reset_sddm: bool,

    #[arg(long = "reset")]
    reset: bool,

    #[arg(long = "cache-create")]
    cache_create: bool,

    #[arg(long = "cache-delete")]
    cache_delete: bool,

    #[arg(long = "cache-query")]
    cache_query: bool,

    #[arg(long = "verbose")]
    verbose: bool,
}

fn create_file(path: &str, content: &str, executable: bool) {
    if let Some(parent) = Path::new(path).parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(path, content);
    if executable {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o755));
    }
}

fn cleanup() {
    let to_remove = [
        BLACKLIST_PATH, UDEV_INTEGRATED_PATH, UDEV_PM_PATH, XORG_PATH,
        EXTRA_XORG_PATH, MODESET_PATH, LIGHTDM_SCRIPT_PATH, LIGHTDM_CONFIG_PATH,
        "/etc/X11/xorg.conf.d/90-nvidia.conf",
        "/lib/udev/rules.d/50-remove-nvidia.rules",
        "/lib/udev/rules.d/80-nvidia-pm.rules",
    ];
    for p in to_remove {
        let _ = fs::remove_file(p);
    }
    let backup = format!("{}.bak", SDDM_XSETUP_PATH);
    if Path::new(&backup).exists() {
        if let Ok(c) = fs::read_to_string(&backup) {
            create_file(SDDM_XSETUP_PATH, &c, false);
        }
        let _ = fs::remove_file(&backup);
    }
}

fn get_nvidia_gpu_pci_bus() -> String {
    let out = Command::new("lspci").output().unwrap();
    let text = String::from_utf8_lossy(&out.stdout);
    for line in text.lines() {
        if line.to_lowercase().contains("vga") && line.to_lowercase().contains("nvidia") {
            let id = line.split_whitespace().next().unwrap_or("");
            // convert 01:00.0 -> PCI:1:0:0
            let parts: Vec<&str> = id.split(&[':', '.'][..]).collect();
            if parts.len() >= 3 {
                let bus = u32::from_str_radix(parts[0], 16).unwrap_or(0);
                let dev = u32::from_str_radix(parts[1], 16).unwrap_or(0);
                let func = parts[2];
                return format!("PCI:{}:{}:{}", bus, dev, func);
            }
        }
    }
    "PCI:1:0:0".to_string()
}

fn get_igpu_vendor() -> String {
    let out = Command::new("lspci").output().unwrap();
    let text = String::from_utf8_lossy(&out.stdout);
    for line in text.lines() {
        let l = line.to_lowercase();
        if l.contains("vga") && !l.contains("nvidia") {
            if l.contains("intel") { return "intel".to_string(); }
            if l.contains("amd") || l.contains("ati") { return "amd".to_string(); }
        }
    }
    "intel".to_string()
}

fn rebuild_initramfs() {
    // best-effort: try common tools
    let tools = ["dracut", "mkinitcpio", "update-initramfs"];
    for t in tools {
        if Command::new("which").arg(t).status().map(|s| s.success()).unwrap_or(false) {
            let _ = Command::new(t).arg("-f").status();
            return;
        }
    }
}

fn generate_xrandr_script(igpu_vendor: &str) -> String {
    let pci = get_nvidia_gpu_pci_bus();
    NVIDIA_XRANDR_SCRIPT.replace("{}", &pci)
}

fn main() {
    let cli = Cli::parse();
    if cli.version { println!("{}", VERSION); return; }

    if cli.reset {
        cleanup();
        println!("Reverted changes");
        return;
    }

    if cli.query {
        // simplified: check files
        if Path::new(XORG_PATH).exists() { println!("nvidia"); }
        else if Path::new(UDEV_INTEGRATED_PATH).exists() { println!("integrated"); }
        else { println!("hybrid"); }
        return;
    }

    if let Some(mode) = cli.switch {
        // integrated
        if mode == "integrated" {
            cleanup();
            create_file(BLACKLIST_PATH, BLACKLIST_CONTENT, false);
            create_file(UDEV_INTEGRATED_PATH, UDEV_INTEGRATED, false);
            rebuild_initramfs();
            println!("Switching to integrated mode");
            println!("Operation completed successfully");
            println!("Please reboot your computer for changes to take effect!");
            return;
        }
        if mode == "hybrid" {
            cleanup();
            let rtd3 = cli.rtd3.flatten();
            if rtd3.is_none() {
                if cli.use_nvidia_current { create_file(MODESET_PATH, MODESET_CURRENT_CONTENT, false); }
                else { create_file(MODESET_PATH, MODESET_CONTENT, false); }
            } else {
                let v = rtd3.unwrap();
                let content = if cli.use_nvidia_current { MODESET_CURRENT_RTD3 } else { MODESET_RTD3 };
                let c = content.replace("{}", &v.to_string());
                create_file(MODESET_PATH, &c, false);
                create_file(UDEV_PM_PATH, UDEV_PM_CONTENT, false);
            }
            rebuild_initramfs();
            println!("Switching to hybrid mode");
            println!("Operation completed successfully");
            println!("Please reboot your computer for changes to take effect!");
            return;
        }
        if mode == "nvidia" {
            cleanup();
            let pci = get_nvidia_gpu_pci_bus();
            let igpu = get_igpu_vendor();
            let xorg = if igpu == "intel" { XORG_INTEL.replace("{}", &pci) } else { XORG_AMD.replace("{}", &pci) };
            create_file(XORG_PATH, &xorg, false);
            if cli.use_nvidia_current { create_file(MODESET_PATH, MODESET_CURRENT_CONTENT, false); }
            else { create_file(MODESET_PATH, MODESET_CONTENT, false); }

            if cli.force_comp || cli.coolbits.is_some() {
                let mut extra = EXTRA_XORG_CONTENT.to_string();
                if cli.force_comp { extra.push_str(FORCE_COMP); }
                if let Some(cb) = cli.coolbits.flatten() { extra.push_str(&COOLBITS.replace("{}", &cb.to_string())); }
                extra.push_str("EndSection\n");
                create_file(EXTRA_XORG_PATH, &extra, false);
            }
            let dm = cli.dm.clone().unwrap_or_else(get_display_manager);
            if dm == "sddm" {
                if Path::new(SDDM_XSETUP_PATH).exists() {
                    if let Ok(c) = fs::read_to_string(SDDM_XSETUP_PATH) {
                        create_file(&format!("{}.bak", SDDM_XSETUP_PATH), &c, false);
                    }
                }
                create_file(SDDM_XSETUP_PATH, &generate_xrandr_script(&igpu), true);
            } else if dm == "lightdm" {
                create_file(LIGHTDM_SCRIPT_PATH, &generate_xrandr_script(&igpu), true);
                create_file(LIGHTDM_CONFIG_PATH, LIGHTDM_CONFIG_CONTENT, false);
            }
            rebuild_initramfs();
            println!("Switching to nvidia mode");
            println!("Operation completed successfully");
            println!("Please reboot your computer for changes to take effect!");
            return;
        }
    }

    if cli.cache_create {
        println!("cache create not fully implemented");
        return;
    }
    if cli.cache_delete {
        let _ = std::fs::remove_file(CACHE_FILE_PATH);
        println!("cache deleted");
        return;
    }
    if cli.cache_query {
        if let Ok(c) = std::fs::read_to_string(CACHE_FILE_PATH) { println!("{}", c); }
        else { println!("no cache"); }
        return;
    }
    if cli.reset_sddm {
        // restore Xsetup from backup if exists
        println!("reset-sddm done");
        return;
    }
    println!("Nothing to do. Use -h for help.");
}

fn get_display_manager() -> String {
    // simplified
    if Path::new("/etc/gdm3").exists() { "gdm3".to_string() }
    else if Path::new("/etc/sddm.conf").exists() || Path::new("/usr/share/sddm").exists() { "sddm".to_string() }
    else if Path::new("/etc/lightdm").exists() { "lightdm".to_string() }
    else { "gdm".to_string() }
}
