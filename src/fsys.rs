use std::fs::{File, OpenOptions};

pub(crate) fn get_file_handle(file_name: &str) -> std::io::Result<File> {
    OpenOptions::new().read(true).write(true).create(true).truncate(false).open(file_name)
}