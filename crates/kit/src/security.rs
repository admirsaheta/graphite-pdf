#[derive(Clone, Debug, Default)]
pub struct SecurityOptions {
    pub user_password: Option<String>,
    pub owner_password: Option<String>,
    pub permissions: Permissions,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Permissions {
    pub print: bool,
    pub modify: bool,
    pub copy: bool,
    pub annotate: bool,
}

impl Permissions {
    pub fn all() -> Self {
        Self {
            print: true,
            modify: true,
            copy: true,
            annotate: true,
        }
    }

    pub fn none() -> Self {
        Self::default()
    }
}
