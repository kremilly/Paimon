extern crate chrono;

use regex::Regex;
use chrono::Local;
use once_cell::sync::Lazy;
use dirs_next::config_dir;

use std::{
    error::Error,
    path::PathBuf,

    process::{
        Stdio,
        Command,
    },
};

use crate::{
    consts::global::Global,
    regexp::regex_core::CoreRegExp,
    ui::errors_commands_alerts::ErrorsCommandsAlerts,
};

pub struct System;

impl System {

    pub const APP_FOLDER: Lazy<PathBuf> = Lazy::new(|| {
        let mut path = config_dir().expect("No config directory");
        path.push(Global::APP_NAME);
        path
    });
    
    pub const SETTINGS_FILE: Lazy<PathBuf> = Lazy::new(|| {
        let mut path = config_dir().expect("No config directory");
        path.push(Global::APP_NAME);
        path.push(
            format!("{}.yml", Global::APP_NAME)
        );

        path
    });
    
    pub const README_FOLDER: Lazy<PathBuf> = Lazy::new(|| {
        let mut path = System::APP_FOLDER.clone();
        path.push("readme");
        path
    });

    pub fn date_time() -> String {
        let local_time = Local::now();
    
        let date_formated = local_time.format("%Y-%m-%d").to_string();
        let hour_formated = local_time.format("%H:%M:%S").to_string();
    
        format!("{} {}", date_formated, hour_formated)
    }
    
    pub fn exec_script(line: &str, program: &str) -> Result<(), Box<dyn Error>> {
        let line_cleanned = Regex::new(
            CoreRegExp::CLEAN_LINE
        ).unwrap().replace_all(
            &line, ""
        ).to_string();

        let output = Command::new(&program)
            .arg(line_cleanned)
            .stdout(Stdio::piped())
            .output()?;
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("{}", stdout);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            ErrorsCommandsAlerts::executing(&stderr);
        }

        Ok(())
    }

}
