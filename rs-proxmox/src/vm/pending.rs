use serde::{Deserialize, Serialize};

const API_PATH_FORMAT: &str = "nodes/{node}/qemu/{vm_id}/pending";

#[derive(Serialize, Deserialize)]
pub struct GetRequest {
    /// The cluster node name.
    node: String,
    /// The unique ID of the VM.
    vmid: u32
}

/// Pending returns the VirtualMachine configuration with both current & pending values.
#[derive(Serialize, Deserialize)]
pub struct GetResponse {
    /// Configuration option name.
    key: String,

    // Optional
    /// Indicates a pending delete request if present and not 0.
    /// The value 2 indicates a force-delete request
    delete: u8,
    /// Pending value.
    pending: String,
    /// Current value.
    value: String
}