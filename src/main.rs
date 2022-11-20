use sane_scan::{DeviceOption, DeviceOptionValue, Sane};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let sane = Sane::init_1_0().expect("Failed to initialize SANE");
    let mut devices = sane.get_devices().expect("Failed to enumerate devices");
    let device = devices.first_mut().expect("No scanner found");
    println!("{:?}", device);

    let mut handle = device.open().expect("Failed to open the device handle");

    let options = handle.get_options().expect("Failed to get options");
    for option in options.iter() {
        println!("{:?}", option);
    }

    let params = handle.get_parameters().expect("Failed to get parameters");
    println!("{:?}", params);

    // Set resolution.
    if let Some(option) = options.find_option("resolution") {
        let value = DeviceOptionValue::Int(1);
        handle
            .set_option(&option, value)
            .expect("Failed to set resolution");
    }

    // Set depth to color scan.
    if let Some(option) = options.find_option("depth") {
        let value = DeviceOptionValue::Int(16);
        handle
            .set_option(&option, value)
            .expect("Failed to set depth");
    }

    // Set top-left X.
    if let Some(option) = options.find_option("tl-x") {
        let value = DeviceOptionValue::Fixed(0);
        handle
            .set_option(&option, value)
            .expect("Failed to set top-left X coordinate");
    }

    // Set top-left Y.
    if let Some(option) = options.find_option("tl-y") {
        let value = DeviceOptionValue::Fixed(0);
        handle
            .set_option(&option, value)
            .expect("Failed to set top-left Y coordinate");
    }

    // Set bottom-right X.
    if let Some(option) = options.find_option("br-x") {
        let value = DeviceOptionValue::Fixed(14090240);
        handle
            .set_option(&option, value)
            .expect("Failed to set bottom-right X coordinate");
    }

    // Set bottom-right Y.
    if let Some(option) = options.find_option("br-y") {
        let value = DeviceOptionValue::Fixed(19464192);
        handle
            .set_option(&option, value)
            .expect("Failed to set bottom-right Y coordinate");
    }

    // Disable red lamp.
    if let Some(option) = options.find_option("redlamp-off") {
        let value = DeviceOptionValue::Int(0);
        handle
            .set_option(&option, value)
            .expect("Failed to set red lamp off");
    }

    // Disable green lamp.
    if let Some(option) = options.find_option("greenlamp-off") {
        let value = DeviceOptionValue::Int(0);
        handle
            .set_option(&option, value)
            .expect("Failed to set green lamp off");
    }

    // Disable blue lamp.
    if let Some(option) = options.find_option("bluelamp-off") {
        let value = DeviceOptionValue::Int(0);
        handle
            .set_option(&option, value)
            .expect("Failed to set blue lamp off");
    }

    // Enable preview.
    if let Some(option) = options.find_option("preview") {
        let value = DeviceOptionValue::Int(0);
        handle
            .set_option(&option, value)
            .expect("Failed to set preview mode");
    }

    let params = handle.get_parameters().expect("Failed to get parameters");
    println!("{:?}", params);

    let params = handle.start_scan().expect("Failed to start scan");
    println!("{:?}", params);

    let mut file = File::create("test.ppm").expect("Failed to create output file");
    writeln!(
        file,
        "P6\n{} {}\n65535\n",
        params.pixels_per_line, params.lines
    )
    .expect("Failed to write to output file");

    // TODO: Read the number of bytes per line (params.pixels_per_line, params.bytes_per_line, params.lines, params.depth)
    let mut buffer = vec![0u8; 1024];
    let mut bytes_read = 0;
    loop {
        match handle.read(&mut buffer) {
            Err(e) => {
                println!("{:?}", e);
                break;
            }
            Ok(None) => break,
            Ok(Some(count)) => {
                bytes_read += count;
                file.write_all(&buffer[0..count])
                    .expect("Failed to write to the output file");
            }
        }
    }

    println!("Total bytes read: {}", bytes_read);
}

trait FindOption<'a> {
    /// Finds a [`sane_scan::DeviceOption`] by name.
    ///
    /// ## Panics
    /// Panics if the option name cannot be represented as a [`std::ffi::CString`].
    ///
    /// ## Returns
    /// A reference to the found [`sane_scan::DeviceOption`] or
    /// [`Option::None`] if the option was not found.
    fn find_option<S: AsRef<str>>(&'a self, name: S) -> Option<&'a DeviceOption>;
}

impl<'a> FindOption<'a> for Vec<DeviceOption> {
    fn find_option<S: AsRef<str>>(&'a self, name: S) -> Option<&'a DeviceOption> {
        let name = name.as_ref().c_string();
        self.iter().find(|opt| opt.name.eq(&name))
    }
}

trait AsCString {
    /// Convert to a [`std::ffi::CString`] instance.
    ///
    /// ## Panics
    /// Panics if the input cannot be represented as a [`std::ffi::CString`].
    ///
    /// ## Returns
    /// The [`std::ffi::CString`].
    fn c_string(self) -> std::ffi::CString;
}

impl<S: AsRef<str>> AsCString for S {
    fn c_string(self) -> std::ffi::CString {
        std::ffi::CString::new(self.as_ref()).expect("CString::new failed")
    }
}
