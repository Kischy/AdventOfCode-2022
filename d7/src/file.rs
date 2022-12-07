
#[derive(Clone)]
pub struct File {
    pub name: String,
    pub size: u128,
}

impl File {
    pub fn new(name: String, size: u128) -> File {
        File {
            name: name,
            size: size,
        }
    }
}
