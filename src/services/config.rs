use js_sys::{Array, JsString, Object, Reflect};
use web_sys::window;

const LE_DEFAULT: &str = "http://localhost/listing/<PATHNAME>";
const FE_DEFAULT: &str = "http://localhost/file/<PATHNAME>";
const TE_DEFAULT: &str = "http://localhost/thumbnail/<PATHNAME>";
const MAX_DEFAULT: &str = "100M";

#[derive(Clone, Debug, PartialEq)]
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
            list_endpoint: Config::field_get_string(&vyw, "listEndpoint", LE_DEFAULT),
            file_endpoint: Config::field_get_string(&vyw, "fileEndpoint", FE_DEFAULT),
            thumbnailer: Config::field_get_string(&vyw, "thumbnailer", TE_DEFAULT),
            max_size: Config::field_get_string(&vyw, "maxSize", MAX_DEFAULT),
            supported_image_type: Config::field_get_vec_string(
                &vyw,
                "supportedImageType",
                vec!["jpg"],
            ),
        })
    }

    fn field_get_string(vyw: &Object, key: &str, default: &str) -> String {
        Reflect::get(vyw, &JsString::from(key))
            .ok()
            .and_then(|val| val.as_string())
            .unwrap_or(default.to_string())
    }

    fn field_get_vec_string(vyw: &Object, key: &str, default: Vec<&str>) -> Vec<String> {
        Reflect::get(vyw, &JsString::from(key))
            .ok()
            .map(|val| Array::from(&val).to_vec())
            .map(|val| {
                val.iter()
                    .map(|elt| elt.as_string().unwrap_or("".to_string()))
                    .collect::<Vec<String>>()
            })
            .unwrap_or(
                default
                    .iter()
                    .map(|elt| elt.to_string())
                    .collect::<Vec<_>>(),
            )
    }

    fn url_encode(url: &str) -> String {
        js_sys::encode_uri(url).into()
    }

    pub fn list_endpoint(&self, path: &str, name: &str) -> String {
        let mut path = String::from(path); 
        path.push('/');
        path.push_str(name);
        Config::url_encode(
            &self
                .list_endpoint
                .replace("/<PATHNAME>", &path),
        )
    }

    pub fn file_endpoint(&self, path: &str, name: &str) -> &str {
        &self.file_endpoint
    }

    pub fn thumbnailer(&self, path: &str, name: &str) -> &str {
        &self.thumbnailer
    }

    pub fn max_size(&self) -> &str {
        &self.max_size
    }

    pub fn supported_image_type(&self) -> &Vec<String> {
        &self.supported_image_type
    }
}
