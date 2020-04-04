#[derive(Copy, Clone, PartialEq)]
pub enum VywrsMode {
    List,
    Tile,
}

#[derive(Copy, Clone, PartialEq)]
pub enum VywrsTheme {
    Dark,
    Light,
}

impl std::ops::Deref for VywrsTheme {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            VywrsTheme::Dark => "dark",
            VywrsTheme::Light => "light",
        }
    }
}
