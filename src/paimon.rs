use clap::Parser;
use std::error::Error;

use crate::{
    args_cli::Flags,
    configs::env::Env,
    cmd::read_list::ReadList,
    prime_down::pd_core::PrimeDown,

    ui::{
        ui_base::UI,
        errors_alerts::ErrorsAlerts,
    },

    addons::{
        scrape::Scrape,
        ravenlib::Ravenlib, 
    },
};

pub struct Paimon;

impl Paimon {
    
    async fn options(value: &str) -> Result<(), Box<dyn Error>> {
        if value == "open-env" {
            Env::open_env_file()?;
        } else if value == "force-download-env" {
            Env::download_env_file(true, true).await?;
        }
        
        Ok(())
    }

    pub async fn init() {
        if let Err(err) = Env::download_env_file(false, false).await {
            ErrorsAlerts::generic(&err.to_string());
        }

        let flags = Flags::parse();
        let url = flags.url.as_deref().unwrap_or_default();
        let run = flags.run.as_deref().unwrap_or_default();
        let options = flags.options.as_deref().unwrap_or_default();

        UI::header();
        
        if !run.is_empty() {
            if !Ravenlib::check_is_user(run) {
                let _ = ReadList::read_dataset(
                    run, flags.no_ignore, flags.no_comments, flags.no_open_link
                ).await;

                PrimeDown::render_and_save_file(run, flags.no_open_link);
            } else {
                let _ = Ravenlib::get(
                    run, flags.no_ignore, flags.no_comments
                ).await;
            }
        }

        let _ = Scrape::get(
            flags.scrape, url, flags.no_ignore, flags.no_comments
        ).await;

        let _ = Self::options(options).await;
    }

}
