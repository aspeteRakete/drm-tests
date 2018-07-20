extern crate drm;

use std::fs::{OpenOptions, File};
use std::os::unix::io::{AsRawFd, RawFd};

use drm::control::ResourceInfo;
use drm::control::ResourceHandle;
use drm::control::{connector, crtc};

use drm::Device as BasicDevice;
use drm::control::Device as ControlDevice;



// The drm crate does not provide a method of opening the device.
// It is expected to be implemented by the user.
struct DrmCard(File);

// Required to implement drm::Device
impl AsRawFd for DrmCard {
    fn as_raw_fd(&self) -> RawFd { self.0.as_raw_fd() }
}

// Required to implement drm::control::Device
impl BasicDevice for DrmCard { }

// Allows modesetting functionality to be performed.
impl ControlDevice for DrmCard { }

impl  DrmCard {
    pub fn open(path: &str) -> Self {
        let mut options = OpenOptions::new();
        options.read(true);
        options.write(true);
        DrmCard(options.open(path).unwrap())
    }

    pub fn open_global() -> Self {
        Self::open("/dev/dri/card0")
    }

    pub fn open_control() -> Self {
        Self::open("/dev/dri/controlD64")
    }
}

fn load_information<T, U>(card: &DrmCard, handles: &[T]) -> Vec<U>
    where
        T: ResourceHandle,
        U: ResourceInfo<Handle = T>,
{
    handles
        .iter()
        .map(|&h| card.resource_info(h).expect("Could not load resource info"))
        .collect()
}

fn main() {
    println!("Hello World");
    let card = DrmCard::open_global();
    // Get a set of all modesetting resource handles (excluding planes):
    let res_handles = card.resource_handles().unwrap();

    // Load the information.
    let res = card.resource_handles()
        .expect("Could not load normal resource ids.");
    let coninfo: Vec<connector::Info> = load_information(&card, res.connectors());
    //let crtcinfo: Vec<crtc::Info> = load_information(&card, res.crtcs());

    for info in coninfo.iter() {
        println!("Connector info:");
        println!("Type: {}", info.connector_type());
        println!("Connected: {:#?}", info.connection_state());
        println!("Supported Modes:");
        for mode in info.modes().iter() {
            println!("{} x {} @ {}", mode.size().0, mode.size().1,  mode.vrefresh());
        }
    }

    // Filter each connector until we find one that's connected.
//    let con = coninfo
//        .iter()
//        .filter(|&i| i.connection_state() == connector::State::Connected)
//        .next()
//        .expect("No Connected Connectors");
//
//    for &info in con {
//        println!(":?",info);
//    }

    // Print all connector information
//    for &con in res_handles.connectors() {
//        let info : drm::control::connector::Info = card.resource_info(con).unwrap();
//        for info.modes
//        println!("{:#?}", info)
//    }
//
//    // Print all CRTC information
//    for &crtc in res_handles.crtcs() {
//        let info : drm::control::crtc::Info = card.resource_info(crtc).unwrap();
//
//        println!("{:#?}", info)
//    }


}