use web_sys::window;

pub struct BodyClassSetter;

impl BodyClassSetter {
    pub fn set(class: &str) -> Result<(), &str> {
        window()
            .ok_or("cannot invoke window")?
            .document()
            .ok_or("cannot invoke document")?
            .body()
            .ok_or("cannot invoke body")?
            .set_class_name(class);
        Ok(())
    }
}
