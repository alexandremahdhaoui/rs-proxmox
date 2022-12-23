use reqwest::Method;
use serde::{Deserialize, Serialize};
use rs_proxmox_client::Builder;

const API_PATH_FORMAT: &str = "nodes/{node}/qemu/{vm_id}/clone";

enum Format {
    Raw,
    Qcow2,
    Vmdk
}

#[derive(Serialize, Deserialize)]
pub struct PostRequest {
    // Required
    /// Vm ID for the clone.
    new_id: u32,
    /// The cluster node name.
    node: String,
    /// The ID of the VM to clone.
    vm_id: u32,

    // Optional
    /// Override I/O bandwidth limit (in KiB/s).
    bw_limit: Option<usize>,
    /// Description for the new vm
    description: Option<String>,
    /// Target format for file storage. Only valid for full clone
    format: Option<Format>,
    /// Create a full copy of all disks. This is always done when you clone a normal VM.
    /// For VM templates, we try to create a linked clone by default.
    full: Option<boolean>,
    /// Set a name for the new VM.
    name: Option<String>,
    /// Add the new VM to the specified pool
    pool: Option<String>,
    /// The name of the snapshot
    snap_name: Option<String>,
    /// Target storage for full clone
    storage: Option<String>,
    /// Target node. Only allowed if the original VM is on shared storage.
    target: Option<String>,
}

pub struct PostResponse {}

pub async fn post(node: &str, vmid: &str, payload: PostReq) -> PostResp {
    let c = Builder::new().build()?;
    let response = c.request(
        Method::POST,
        &format!(API_PATH_FORMAT, &node, &vmid),
        payload
    ).await?.text().await?;
    PostResp{}
}
