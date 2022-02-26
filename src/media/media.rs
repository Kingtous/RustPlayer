
pub enum Source {
    Http(String),
    Local(String),
}

pub struct Media{
    pub src: Source
}