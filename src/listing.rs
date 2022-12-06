use bytesize::ByteSize;
use serde::Deserialize;
use std::ffi::OsStr;
use std::path::Path;

use crate::services::Config;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct File {
    name: String,
    #[serde(rename = "type")]
    _type: String,
    mtime: String,
    #[serde(default)]
    size: u64,
}

pub enum FileType {
    Directory,
    Image,
    File,
}

impl From<FileType> for String {
    fn from(ft: FileType) -> Self {
        match ft {
            FileType::Directory => "directory".to_string(),
            FileType::Image => "image".to_string(),
            FileType::File => "file".to_string(),
        }
    }
}

impl File {
    pub fn name(&self) -> String {
        js_sys::decode_uri_component(&self.name)
            .map(|jsstring| jsstring.into())
            .unwrap_or_else(|_| self.name.clone())
    }

    pub fn size(&self) -> String {
        ByteSize::b(self.size).to_string()
    }

    pub fn mtime(&self) -> String {
        self.mtime.clone()
    }

    pub fn file_type(&self, cfg: &Config) -> FileType {
        if self._type == "directory" {
            return FileType::Directory;
        }

        let file = Path::new(&self.name);
        match file.extension() {
            Some(ext) if File::is_image(ext, cfg.supported_image_type()) => FileType::Image,
            _ => FileType::File,
        }
    }

    fn is_image(ext: &OsStr, supported: &Vec<String>) -> bool {
        supported
            .iter()
            .any(|iex| ext.to_string_lossy().as_ref().to_lowercase().eq(iex))
    }
}
