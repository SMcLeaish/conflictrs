use anyhow::Result;
use std::fs::File;
use std::io::Write;
use tempfile::Builder;

pub fn fetch_gdelt() -> Result<()> {
    let tmp_dir = Builder::new().prefix("gdelt").tempdir()?;
    let target = "http://data.gdeltproject.org/gdeltv2/masterfilelist.txt"
    response = reqwest::blocking::get
}
