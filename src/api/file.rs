#[derive(Debug, PartialEq)]
pub struct AltObject {
    r#type: String,
    width: u32,
    height: u32,
    urls: Vec<String>
}

#[derive(Debug, PartialEq)]
pub struct FileObject {
    width: u32,  // According to the png standard, the maximum width and
    height: u32, // height is one that can be contained in 4 byte uint (u32)
    ext: String,
    size: Option<u32>,
    md5: Option<String>,
    url: Option<String>,
    alternatives: Option<AltObject>
}

impl FileObject {
    
}
