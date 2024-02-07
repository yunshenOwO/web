use std::collections::HashMap;
use std::io::{Result, Write};


#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a>{
    version:&'a str,

}