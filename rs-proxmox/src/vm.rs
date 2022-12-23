mod clone;
mod agent;
mod template;
mod status;
mod pending;
mod config;

const API_PATH_FORMAT: &str = "nodes/{node}/qemu/{vm_id}";

pub struct GetRequest {}
pub struct GetResponse {}
pub struct PostRequest {}
pub struct PostResponse {}
pub struct PutRequest {}
pub struct PutResponse {}
pub struct DeleteRequest {}
pub struct DeleteResponse {}
