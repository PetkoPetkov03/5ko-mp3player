use serde::Serialize;

#[derive(Serialize)]

pub enum FileType {
    FOLDER,
    MP3,    
}

#[derive(Serialize)]
pub struct FileStruct {
    pub file_path: String,
    pub file_type: FileType
}