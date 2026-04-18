use std::collections::HashMap;

pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch
}

pub struct Req<'a> {
    pub(crate) method: Method,
    pub(crate) target: &'a str,
    pub headers: HashMap<&'a str, &'a str>,
    pub payload: &'a str
}