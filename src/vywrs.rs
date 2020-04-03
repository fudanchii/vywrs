use crate::listing::File;

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

pub enum VywrsMessage {
    ChangeMode(VywrsMode),
    ChangeTheme(VywrsTheme),
    UpdateListing(Vec<File>),
    FetchFailed,
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
