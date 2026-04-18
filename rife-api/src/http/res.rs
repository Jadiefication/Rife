pub(crate) enum Body<'a> {
    Bytes(Vec<u8>),
    Str(&'a str),
}

pub struct Res<'a> {
    pub(crate) status: u16,
    pub(crate) s_text: &'a str,
    pub(crate) body: Body<'a>
}