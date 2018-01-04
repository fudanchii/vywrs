use yew::html::*;
use stdweb::private::from_value;
use std::path::Path;
use std::path::PathBuf;
use bytesize::ByteSize;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ViewMode {
    Thumbnail,
    List,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct File {
    name: String,
    #[serde(rename = "type")]
    type_: String,
    mtime: String,
    #[serde(default)]
    size: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FileListing {
    listing: Vec<File>,
    view_mode: ViewMode,
    config: VywConfig,
    location: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VywConfig {
    list_endpoint: String,
    file_endpoint: String,
    thumbnailer: String,
    max_size: String,
    supported_image_type: Vec<String>,
}

pub fn get_location_hash() -> String {
    from_value(js!(return window.location.hash.slice(1) || "/"))
        .unwrap_or("/".to_owned())
}

impl FileListing {
    pub fn create() -> FileListing {
        FileListing {
            listing: Vec::new(),
            view_mode: ViewMode::Thumbnail,
            location: "/".to_owned(),
            config: from_value(js!(return window.vyw))
                .unwrap_or(VywConfig::create()),
        }
    }

    pub fn parent_dir_endpoint() -> String {
        let lochash = PathBuf::from(get_location_hash());
        let lochash = lochash.parent()
            .unwrap_or(Path::new("/"))
            .to_str()
            .unwrap();
        format!("#{}", lochash)
    }

    pub fn listing_endpoint(&self) -> String {
        let hashloc = get_location_hash();
        self.config.list_endpoint.replace("/<PATHNAME>", &hashloc)
    }

    pub fn file_endpoint(&self, f: &File) -> String {
        match f.file_type() {
            ref s if s == "directory" => format!("#{}", f.endpoint()),
            _ => {
                self.config.file_endpoint.replace("/<PATHNAME>", &f.endpoint())
            },
        }
    }

    pub fn thumbnail_endpoint(&self, f: &File) -> String {
        self.config.thumbnailer.replace("/<PATHNAME>", &f.endpoint())
    }

    pub fn set_listing(&mut self, lst: &[File]) {
        self.listing = lst.to_vec()
    }

    pub fn get_view_mode(&self) -> ViewMode {
        self.view_mode.clone()
    }

    pub fn set_view_mode(&mut self, v: &ViewMode) {
        self.view_mode = v.clone();
    }

    pub fn map_listing<MSG, F>(&self, f: F) -> Vec<Html<MSG>>
    where
        MSG: 'static,
        F: Fn(&FileListing, &File) -> Html<MSG> + 'static,
    {
        (&self.listing).iter().map(|item| f(self, item)).collect()
    }
}

fn is_image(s: &str) -> bool {
    vec!["jpg", "jpeg", "png", "gif", "webp"]
        .iter()
        .any(|&x| x == s)
}

impl File {
    pub fn file_type(&self) -> String {
        let fname = Path::new(&self.name);
        match fname.extension() {
            Some(ext) if is_image(ext.to_str().unwrap()) => "image".to_owned(),
            Some(_) => self.type_.clone(),
            None => "directory".to_owned(), 
        }
    }

    pub fn endpoint(&self) -> String {
        match get_location_hash() {
            ref s if s == "/" => format!("{}{}", s, self.name()),
            ref s => format!("{}/{}", s, self.name()),
        }
    }

    pub fn mtime(&self) -> String {
        self.mtime.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn size(&self) -> String {
        ByteSize::b(self.size as usize).to_string(true)
    }
}

impl VywConfig {
    pub fn create() -> VywConfig {
        VywConfig {
            list_endpoint: "/<PATHNAME>".to_owned(),
            file_endpoint: "/<PATHNAME>".to_owned(),
            thumbnailer: "/<PATHNAME>".to_owned(),
            max_size: "100K".to_owned(),
            supported_image_type: Vec::new(),
        }
    }
}