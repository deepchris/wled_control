#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt::format;
use std::net;
use std::net::IpAddr;
use futures::future::err;
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgba, RgbImage};
use image::codecs::png::CompressionType::Default;
use reqwest::*;

/// # wled-control
/// `wled-control` is a (for-now) JSON API CLI that sends fully formed image
/// commands (and more!) directly to your WLED-powered device.
///
/// ## To Implement:
/// - [x] exactly 16x16 pixels image loading
/// - [x] `off` function (turns the device off, only needs the IP address)
/// - [x] a builder function called `new`
/// - [ ] Image resizing (Currently the image must be exactly 16x16)
/// - [ ] Conversion logic from pixels, to WLED JSON API (WIP: fn change_image)
/// - [ ] Command line arguments
/// - [ ] Saving settings
/// - [ ] A more prettified terminal interface (perhaps with tui-rs?)
///
///
/// ## Arguments
/// All arguments should be prepended with a `-`
/// ```
/// -b[0-255] : Pass a brightness value, range from 0 - 255. Keep in mind that
///             very low brightness values may cause the light strip to not turn on.
/// ```

#[derive(Default)]
struct WledValue {
    brightness: Option<u8>,
    on: bool,
    ip: IpAddr,
    data: HashMap<&'static str, &'static str>,
}

impl WledValue {
    pub fn new(brightness: u8,
               on: bool,
               ip: IpAddr,
               data: HashMap<&'static str, &'static str>) -> WledValue {
        WledValue {
            brightness,
            on,
            ip,
            data,
        }
    }

    pub async fn off(&self) {
        let request = format!("http://{:?}/json", self.ip);
        let mut map = HashMap::new();

        map.insert("on", "false");
        map.insert("bri", "0");

        let client = reqwest::Client::new();
        let res = client.post(request)
            .json(&map)
            .send()
            .await?;
    }

    pub async fn change_image(&mut self, img: DynamicImage) {
        let request = format!("http://{:?}/json", self.ip);
        let mut map: HashMap<&str, &str> = HashMap::new();

        let mut start_counter: u32 = 0;
        let mut current_counter: u32 = 0;

        for data_chunk in img.pixels() {
            let pixel = (data_chunk.2.0);
            println!("{:?}", pixel);
            current_counter += 1;
        }

        self.data = map;
    }
}

// TODO: Add argument handling.
#[tokio::main]
async fn main() -> Result<()> {
    // let json_msg = json!({"on":true, "bri":255, "seg":{"i":[
    // 0,23,[0,176,240], 23,25,[255,255,255], 25,38,[0,176,240], 38,42,[255,255,255], 42,53,[0,176,240], 53,59,[255,255,255], 59,68,[0,176,240], 68,72,[255,255,255], 72,[0,176,240], 73,76,[255,255,255], 76,83,[0,176,240], 83,87,[255,255,255], 87,[0,176,240], 88,[255,255,255], 89,[0,176,240], 90,93,[255,255,255], 93,98,[0,176,240], 98,104,[255,255,255], 104,[0,176,240], 105,110,[255,255,255], 110,113,[0,176,240], 113,116,[255,255,255], 116,[0,176,240], 117,120,[255,255,255], 120,[0,176,240], 121,127,[255,255,255], 127,[0,176,240], 128,131,[255,255,255], 131,[0,176,240], 132,[255,255,255], 133,[0,176,240], 134,136,[255,255,255], 136,[0,176,240], 137,139,[255,255,255], 139,[0,176,240], 140,144,[255,255,255], 144,146,[0,176,240], 146,148,[255,255,255], 148,[0,176,240], 149,152,[255,255,255], 152,[0,176,240], 153,[255,255,255], 154,[0,176,240], 155,[255,255,255], 156,[0,176,240], 157,[255,255,255], 158,162,[0,176,240], 162,165,[255,255,255], 165,167,[0,176,240], 167,[255,255,255], 168,[0,176,240], 169,171,[255,255,255], 171,[0,176,240], 172,174,[255,255,255], 174,178,[0,176,240], 178,183,[255,255,255], 183,185,[0,176,240], 185,[255,255,255], 186,[0,176,240], 187,190,[255,255,255], 190,194,[0,176,240], 194,200,[255,255,255], 200,202,[0,176,240], 202,206,[255,255,255], 206,210,[0,176,240], 210,216,[255,255,255], 216,[0,176,240], 217,222,[255,255,255], 222,256,[0,176,240]
    // ]}});
    let path = "/Users/alex/rust-projects/wled_control/src/example_cat.png";
    let img = load_image(path).unwrap();

    use std::default::*;

    let wled = WledValue::new(255, true, ..Default::default());

    Ok(())
}

fn load_image(path: &str) -> Option<DynamicImage> {
    let path_err: &str = "Image path incorrect, or file does not exist.";

    // open the image, return none if error.
    if let img = image::open(path).expect(path_err) {

        // TODO: Implement automatic image resizing, with CLI args available for whatever settings seem relevant.
        return match img.dimensions() {
            (16, 16) => Some(img),
            _ => {
                println!("Image is not 16x16. Please re-run program with correct size.");
                None
            }
        }
    }
    return None;
}