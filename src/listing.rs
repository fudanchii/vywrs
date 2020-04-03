use bytesize::ByteSize;
use serde::Deserialize;
use std::path::Path;

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

impl File {
    pub fn name(&self) -> String {
        js_sys::decode_uri_component(&self.name)
            .map(|jsstring| jsstring.into())
            .unwrap_or(self.name.clone())
    }

    pub fn size(&self) -> String {
        ByteSize::b(self.size).to_string()
    }

    pub fn mtime(&self) -> String {
        self.mtime.clone()
    }

    pub fn location(&self, path: &str) -> String {
        let mut path = String::from(path);
        path.push('/');
        path.push_str(&self.name());
        path
    }

    pub fn file_type(&self) -> FileType {
        if self._type == "directory" {
            return FileType::Directory;
        }

        let file = Path::new(&self.name);
        match file.extension() {
            Some(ext) if File::is_image(ext) => FileType::Image,
            _ => FileType::File,
        }
    }

    fn is_image<T>(ext: &T) -> bool
    where
        T: PartialEq<str> + ?Sized,
    {
        vec!["jpg", "jpeg", "png"].iter().any(|&iex| ext.eq(iex))
    }
}
