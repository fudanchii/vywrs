use js_sys::{Array, JsString, Object, Reflect};
use web_sys::window;

macro_rules! path {
    ($p:ident) => {
        if $p == "" {
            "/"
        } else {
            &$p
        }
    };
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Config {
    list_endpoint: String,
    file_endpoint: String,
    thumbnailer: String,
    max_size: String,
    supported_image_type: Vec<String>,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let vyw = window()
            .ok_or("Cannot invoke window")?
            .get("vyw")
            .ok_or("Cannot invoke vyw object")?;

        Ok(Config {
            list_endpoint: Config::field_get_string(&vyw, "listEndpoint"),
            file_endpoint: Config::field_get_string(&vyw, "fileEndpoint"),
            thumbnailer: Config::field_get_string(&vyw, "thumbnailer"),
            max_size: Config::field_get_string(&vyw, "maxSize"),
            supported_image_type: Config::field_get_vec_string(
                &vyw,
                "supportedImageType",
                vec!["jpg"],
            ),
        })
    }

    fn field_get_string(vyw: &Object, key: &str) -> String {
        Reflect::get(vyw, &JsString::from(key))
            .ok()
            .and_then(|val| val.as_string())
            .unwrap()
    }

    fn field_get_vec_string(vyw: &Object, key: &str, default: Vec<&str>) -> Vec<String> {
        Reflect::get(vyw, &JsString::from(key))
            .ok()
            .map(|val| Array::from(&val).to_vec())
            .map(|val| {
                val.iter()
                    .map(|elt| elt.as_string().unwrap_or_else(|| "".to_string()))
                    .collect::<Vec<String>>()
            })
            .unwrap_or_else(|| {
                default
                    .iter()
                    .map(|elt| (*elt).to_string())
                    .collect::<Vec<_>>()
            })
    }

    pub fn url_decode(url: &str) -> String {
        js_sys::decode_uri(url).unwrap().into()
    }

    pub fn url_encode(url: &str) -> String {
        js_sys::encode_uri(url).into()
    }

    pub fn list_endpoint(&self, path: &str) -> String {
        Config::url_encode(&self.list_endpoint.replace("/<PATHNAME>", path))
    }

    pub fn directory_endpoint(&self, path: &str, name: &str) -> String {
        let mut endpoint = String::from("#");
        endpoint.push_str(path);
        if path != "/" {
            endpoint.push('/');
        }
        endpoint.push_str(name);
        endpoint
    }

    pub fn file_endpoint(&self, path: &str, name: &str) -> String {
        let mut path = String::from(path);
        if path != "/" {
            path.push('/');
        }
        path.push_str(name);
        Config::url_encode(&self.file_endpoint.replace("/<PATHNAME>", &path))
    }

    pub fn thumbnailer(&self, path: &str, name: &str) -> String {
        let mut path = String::from(path);
        path.push('/');
        path.push_str(name);
        Config::url_encode(&self.thumbnailer.replace("/<PATHNAME>", &path))
    }

    pub fn supported_image_type(&self) -> &Vec<String> {
        &self.supported_image_type
    }
}
