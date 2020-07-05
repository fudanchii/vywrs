use crate::services::Config;
use web_sys::window;

pub struct TitleSetter;

impl TitleSetter {
    pub fn set(title: &str) -> Result<(), &str> {
        window()
            .ok_or("cannot invoke window")?
            .document()
            .ok_or("cannot invoke document")?
            .set_title(&Config::url_decode(title));
        Ok(())
    }
}
