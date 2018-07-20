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

    // Load Info about a Ressource
    pub fn load_information<T, U>(&self, handles: &[T]) -> Vec<U>
        where
            T: ResourceHandle,
            U: ResourceInfo<Handle = T>,
    {
        handles
            .iter()
            .map(|&h| self::resource_info(h).expect("Could not load resource info"))
            .collect()
    }

    pub fn print_connector_info(&self) {
        // Get a set of all modesetting resource handles (excluding planes):
        let res_handles = self.resource_handles().unwrap();
        // Load the information.
        let res = self.resource_handles().expect("Could not load normal resource ids.");
        let coninfo: Vec<connector::Info> = card.load_information(res.connectors());
        for info in coninfo.iter() {
            println!("Connector info:");
            println!("Type: {:#?}", info.connector_type());
            println!("Connected: {:#?}", info.connection_state());
            println!("Supported Modes:");
            for mode in info.modes().iter() {
                println!("{} x {} @ {}", mode.size().0, mode.size().1,  mode.vrefresh());
            }
        }
    }

    pub fn print_crtc_info(&self) {
        // Get a set of all modesetting resource handles (excluding planes):
        let res_handles = self.resource_handles().unwrap();
        // Load the information.
        let res = self.resource_handles().expect("Could not load normal resource ids.");
        let crtcinfo: Vec<crtc::Info> = card.load_information(res.crtcs());

        for info in crtcinfo.iter() {
            println!("CRTC info:");
            println!("Position: {},{}", info.position().0, info.position().1);
            match info.mode() {
                Some(m) =>  println!("Mode: {} x {} @ {} ", m.size().0, m.size().1, m.vrefresh()),
                None => println!("No mode set.")
            }
        }
    }

}

