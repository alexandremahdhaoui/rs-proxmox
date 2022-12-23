use std::arch::aarch64::{ST, uint8x8_t};
use serde::de::value::BoolDeserializer;

const API_PATH_FORMAT: &str = "nodes/{node}/qemu/{vm_id}/config";

pub struct GetRequest {}

pub enum Bios {
    Seabios,
    Ovmf
}

pub enum CIType {
    ConfigDrive2,
    NoCloud,
    Opennebula
}

pub enum HugePageMemory {
    any,
    _2,
    _1024
}

pub enum OsType {
    Other,
    Wxp,
    W2k,
    W2k8,
    Wvista,
    Win7,
    Win8,
    Win10,
    Win11,
    L24,
    L26,
    Solaris
}

pub enum Lock {
    Backup,
    Clone,
    Create,
    Migrate,
    Rollback,
    Snapshot,
    SnapshotDelete,
    Suspending,
    Suspended
}

pub enum ScsiHw {
    Isi,
    Isi53c810,
    VirtioScsiPci,
    VirtioScsiSingle,
    Megasas,
    Pvscsi
}

/// Get the virtual machine configuration with pending configuration changes applied.
/// Set the "current" parameter to get the current configuration instead.
pub struct GetResponse {
    /// Enable/disable ACPI.
    acpi: bool,
    /// List of host cores used to execute guest processes, for example: 0,5,8-11
    affinity: String,
    /// Enable/disable communication with the Qemu Guest Agent and its properties.
    /// - enabled:
    agent: String,
    arch: String,
    args: String,
    audio0: String,
    autostart: bool,
    balloon: u32,
    bios: Bios,
    boot: String,
    boot_disk: String,
    cdrom: String,
    ci_custom: String,
    ci_password: String,
    ci_type: String,
    ci_user: String,
    cores: u32,
    cpu_limit: u32,
    cpu_units: u32,
    description: String,
    efi_disk0: String,
    freeze: bool,
    hook_script: String,
    hot_plug: String,
    huge_pages: HugePageMemory,
    ide0: String,
    ip_config_0: String,
    inter_vm_shared_memory: String,
    keep_huge_pages: bool,
    // Keyboard layout for VNC server. This option is generally not required and is often better
    // handled from within the guest OS.
    // keyboard: Keyboard,
    kvm: bool,
    localtime: bool,
    lock: Lock,
    machine: String,
    memory: u32,
    migrate_downtime: f32,
    migrate_speed: u32,
    name: String,
    nameserver: String,
    net0: String,
    numa: bool,
    on_boot: bool,
    os_type: OsType,
    // Experimental
    // parallel0: String,
    protection: bool,
    reboot: bool,
    rng0: String,
    sata0: String,
    scsi0: String,
    scsihw: ScsiHw,
    serial0: String,
    shares: u32,
    smbios1: String,
    smp: u32,
    sockets: u32,
    spice_enhancement: String,
    ssh_keys: String,
    startup: String,
    tablet: bool,
    tags: String,
    tdf: bool,
    template: bool,
    tpmstate0: String,
    unused0: String,
    usb0: String,
    vcpus: u32,
    vga: String,
    virtio0: String,
    vmgenid: String,
    vm_state_storage: String,
    watchdog: String,

}
pub struct PostRequest {}
pub struct PostResponse {}
pub struct PutRequest {}
pub struct PutResponse {}
pub struct DeleteRequest {}
pub struct DeleteResponse {}
