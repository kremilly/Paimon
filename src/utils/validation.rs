use reqwest;
use is_url::is_url;

use std::{
    io,
    path::Path,
    error::Error
};

pub struct Validate;

impl Validate {
  
    pub fn validate_url(url_line: &str) -> io::Result<()> {
        if !is_url(url_line) {
            return Err(
                io::Error::new(
                    io::ErrorKind::InvalidInput, format!(
                        "The url {} is not a valid", url_line
                    )
                )
            );
        }
    
        Ok(())
    }
    
    pub fn validate_file(file: &str) -> io::Result<()> {
        if !Path::new(file).exists() {
            return Err(
                io::Error::new(
                    io::ErrorKind::NotFound, format!(
                        "The file {} does not exist.", file
                    )
                )
            );
        }
    
        if !file.ends_with(".txt") {
            return Err(
                io::Error::new(
                    io::ErrorKind::InvalidData, format!(
                        "The file {} is not a .txt", file
                    )
                )
            );
        }
    
        Ok(())
    }
    
    pub fn validate_file_type(file: &str, file_type: &str) -> io::Result<()> {
        if !file.ends_with(file_type) {
            return Err(
                io::Error::new(
                    io::ErrorKind::InvalidData, format!(
                        "The file {} is not a {}", file, file_type
                    )
                )
            );
        }
    
        Ok(())
    }
    
    pub async fn check_url_status(url: &str) -> Result<(), Box<dyn Error>> {
        let response = reqwest::get(url).await?;
    
        if response.status().as_u16() == 200 {
            Ok(())
        } else {
            Err(
                format!(
                    "Received a non-200 status: {}", 
                    response.status()
                ).into()
            )
        }
    }
    
}
