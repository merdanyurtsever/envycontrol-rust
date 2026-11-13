pub const VERSION: &str = "3.5.3";
pub const CACHE_FILE_PATH: &str = "/var/cache/envycontrol/cache.json";
pub const BLACKLIST_PATH: &str = "/etc/modprobe.d/blacklist-nvidia.conf";
pub const UDEV_INTEGRATED_PATH: &str = "/etc/udev/rules.d/50-remove-nvidia.rules";
pub const UDEV_PM_PATH: &str = "/etc/udev/rules.d/80-nvidia-pm.rules";
pub const XORG_PATH: &str = "/etc/X11/xorg.conf";
pub const EXTRA_XORG_PATH: &str = "/etc/X11/xorg.conf.d/10-nvidia.conf";
pub const MODESET_PATH: &str = "/etc/modprobe.d/nvidia.conf";
pub const SDDM_XSETUP_PATH: &str = "/usr/share/sddm/scripts/Xsetup";
pub const LIGHTDM_SCRIPT_PATH: &str = "/etc/lightdm/nvidia.sh";
pub const LIGHTDM_CONFIG_PATH: &str = "/etc/lightdm/lightdm.conf.d/20-nvidia.conf";

pub const BLACKLIST_CONTENT: &str = include_str!("../constants/blacklist_content.txt");
pub const UDEV_INTEGRATED: &str = include_str!("../constants/udev_integrated.txt");
pub const UDEV_PM_CONTENT: &str = include_str!("../constants/udev_pm_content.txt");
pub const XORG_INTEL: &str = include_str!("../constants/xorg_intel.txt");
pub const XORG_AMD: &str = include_str!("../constants/xorg_amd.txt");
pub const EXTRA_XORG_CONTENT: &str = include_str!("../constants/extra_xorg_content.txt");
pub const FORCE_COMP: &str = "    Option \"ForceCompositionPipeline\" \"true\"\n";
pub const COOLBITS: &str = "    Option \"Coolbits\" \"{}\"\n";
pub const MODESET_CONTENT: &str = include_str!("../constants/modeset_content.txt");
pub const MODESET_RTD3: &str = include_str!("../constants/modeset_rtd3.txt");
pub const MODESET_CURRENT_RTD3: &str = include_str!("../constants/modeset_current_rtd3.txt");
pub const MODESET_CURRENT_CONTENT: &str = include_str!("../constants/modeset_current_content.txt");
pub const LIGHTDM_CONFIG_CONTENT: &str = include_str!("../constants/lightdm_config_content.txt");
pub const NVIDIA_XRANDR_SCRIPT: &str = include_str!("../constants/nvidia_xrandr_script.txt");

