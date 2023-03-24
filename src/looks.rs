use yew::{classes, Classes};

#[derive(Copy, Clone, Default, PartialEq)]
pub enum VywrsTheme {
    #[default]
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

impl From<VywrsTheme> for Classes {
    fn from(item: VywrsTheme) -> Self {
        match item {
            VywrsTheme::Dark => classes!("dark"),
            VywrsTheme::Light => classes!("light"),
        }
    }
}

#[derive(Copy, Clone, Default, PartialEq)]
pub enum VywrsMode {
    List,
    #[default]
    Tile,
}

impl std::ops::Deref for VywrsMode {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            VywrsMode::Tile => "tile",
            VywrsMode::List => "list",
        }
    }
}

impl From<VywrsMode> for Classes {
    fn from(item: VywrsMode) -> Self {
        match item {
            VywrsMode::Tile => classes!("tile"),
            VywrsMode::List => classes!("list"),
        }
    }
}

impl From<&str> for VywrsTheme {
    fn from(item: &str) -> Self {
        match item {
            "light" => VywrsTheme::Light,
            _ => VywrsTheme::Dark,
        }
    }
}

impl From<&str> for VywrsMode {
    fn from(item: &str) -> Self {
        match item {
            "list" => VywrsMode::List,
            _ => VywrsMode::Tile,
        }
    }
}

impl From<Result<String, anyhow::Error>> for VywrsTheme {
    fn from(item: Result<String, anyhow::Error>) -> Self {
        item.map(|val| (val.as_ref() as &str).into())
            .unwrap_or(VywrsTheme::Dark)
    }
}

impl From<Result<String, anyhow::Error>> for VywrsMode {
    fn from(item: Result<String, anyhow::Error>) -> Self {
        item.map(|val| (val.as_ref() as &str).into())
            .unwrap_or(VywrsMode::Tile)
    }
}
