use crate::brightness_transition::BrightnessTransition;
use crate::LightPoint;
use log::LevelFilter;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Daemonize {
    pub log_to: String,
    pub log_level: LevelFilter,
    pub pid_file: String,
}

#[derive(Debug, Deserialize)]
struct General {
    pub light_steps: u32,
    pub min_backlight: u32,
    pub step_barrier: f32,
    pub check_period_in_seconds: u64,
    pub backlight_transition: BrightnessTransition,
    pub backlight_transition_step_count: u32,
    pub backlight_transition_time_seconds: u64,
    pub event_device_name: String,
    pub event_device_mask: String,
    pub enable_max_brightness_mode: bool,
    pub max_backlight_file: String,
    pub backlight_file: String,
    pub illuminance_file: String,
    pub switch_key_code: u16,
}

#[derive(Debug, Deserialize)]
struct Kalman {
    pub q: f32,
    pub r: f32,
    pub covariance: f32,
}

#[derive(Debug, Deserialize)]
struct Light {
    pub light_points: Option<Vec<LightPoint>>,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    daemonize: Daemonize,
    general: General,
    kalman: Kalman,
    light: Light,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            daemonize: Daemonize {
                log_to: "/var/log/illuminanced.log".into(),
                log_level: LevelFilter::Warn,
                pid_file: "/run/illuminanced.pid".into(),
            },
            general: General {
                light_steps: 10,
                min_backlight: 70,
                step_barrier: 0.1,
                check_period_in_seconds: 1,
                backlight_transition: BrightnessTransition::Linear,
                backlight_transition_step_count: 20,
                backlight_transition_time_seconds: 1,
                event_device_name: "/dev/input/event/*".into(),
                event_device_mask: "Asus WMI hotkeys".into(),
                enable_max_brightness_mode: true,
                max_backlight_file: "/sys/class/backlight/intel_backlight/max_brightness".into(),
                backlight_file: "/sys/class/backlight/intel_backlight/brightness".into(),
                illuminance_file: "/sys/bus/acpi/devices/ACPI0008:00/iio:device0/in_illuminance_raw".into(),
                switch_key_code: 0x230, /* KEY_ALS_TOGGLE */
            },
            kalman: Kalman {
                q: 1.0,
                r: 20.0,
                covariance: 10.0,
            },
            light: Light {
                light_points: None
            },
        }
    }
}

impl Config {
    pub fn log_to_syslog(&self) -> bool {
        self.daemonize.log_to == "syslog"
    }

    pub fn log_filename(&self) -> &str {
        &self.daemonize.log_to
    }

    pub fn log_level(&self) -> LevelFilter {
        self.daemonize.log_level
    }

    pub fn pid_filename(&self) -> &str {
        &self.daemonize.pid_file
    }

    pub fn light_steps(&self) -> u32 {
        self.general.light_steps
    }

    pub fn min_backlight(&self) -> u32 {
        self.general.min_backlight
    }

    pub fn step_barrier(&self) -> f32 {
        self.general.step_barrier
    }

    pub fn check_period_in_seconds(&self) -> u64 {
        self.general.check_period_in_seconds
    }

    pub fn event_device_name(&self) -> &str {
        &self.general.event_device_name
    }

    pub fn event_device_mask(&self) -> &str {
        &self.general.event_device_mask
    }

    pub fn is_max_brightness_mode(&self) -> bool {
        self.general.enable_max_brightness_mode
    }

    pub fn kalman_q(&self) -> f32 {
        self.kalman.q
    }

    pub fn kalman_r(&self) -> f32 {
        self.kalman.r
    }

    pub fn kalman_covariance(&self) -> f32 {
        self.kalman.covariance
    }

    pub fn max_backlight_filename(&self) -> &str {
        &self.general.max_backlight_file
    }

    pub fn backlight_filename(&self) -> &str {
        &self.general.backlight_file
    }

    pub fn illuminance_filename(&self) -> &str {
        &self.general.illuminance_file
    }

    pub fn backlight_transition(&self) -> BrightnessTransition {
        self.general.backlight_transition
    }

    pub fn backlight_transition_step_count(&self) -> u32 {
        self.general.backlight_transition_step_count
    }

    pub fn backlight_transition_time_seconds(&self) -> u64 {
        self.general.backlight_transition_time_seconds
    }

    pub fn light_points(&self) -> Box<[LightPoint]> {
        let light_steps = self.light_steps();
        self.light.light_points.as_ref().and_then(|a| Some(a.clone().into_boxed_slice())).unwrap_or_else(
            || Box::new([LightPoint {
                illuminance: 700,
                light: light_steps - 1,
            }])
        )
    }

    pub fn switch_key_code(&self) -> u16 {
        self.general.switch_key_code
    }
}
