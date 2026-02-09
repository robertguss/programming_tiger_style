#[derive(Debug, Clone)]
pub struct Asset {
    pub relative_path: &'static str,
    pub contents: &'static [u8],
    pub executable: bool,
}

pub fn install_assets() -> Vec<Asset> {
    Vec::new()
}

pub fn agents_template() -> &'static [u8] {
    b""
}
