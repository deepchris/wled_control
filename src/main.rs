#![allow(dead_code)]

use image::{DynamicImage, GenericImageView};
use reqwest::header::{HeaderValue, CONTENT_TYPE, USER_AGENT};
use reqwest::*;
use clap::Parser;
use std::default::Default as std_default;
use std::net::{IpAddr};
use std::path::PathBuf;
use std::str;

/// # wled-control
/// `wled-control` is a (for-now) JSON API CLI that sends fully formed image
/// commands (and more!) directly to your WLED-powered device.
///
/// ## To Implement:
/// - [x] exactly 16x16 pixels image loading
/// - [ ] `off` function (turns the device off, only needs the IP address)
/// - [x] a builder function called `new`
/// - [x] Conversion logic from pixels, to WLED JSON API (WIP: fn change_image)
/// - [ ] Image resizing (Currently the image must be exactly 16x16)
/// - [ ] Led panel resizing
/// - [ ] Command line arguments
/// - [ ] Saving settings
/// - [ ] A more prettified terminal interface (perhaps with tui-rs?)
///
///
/// ## Arguments
/// All arguments should be prepended with a `-`
///
/// -b\[0-255] : Pass a brightness value, range from 0 - 255. Keep in mind that
///             very low brightness values may cause the light strip to not turn on.
///

struct WledValue {
    brightness: Option<u8>,
    on: Option<bool>,
    client: Option<Client>,
    ip: IpAddr,
    data: String,
    height: u32,
    width: u32,
}

impl WledValue {

    fn from_args(args: Args) -> WledValue {


        return WledValue;
    }

    async fn send_updates(&mut self) -> std::result::Result<Response, Error> {

        let url = format!("http://{:?}/json/state", self.ip);

        let json_header: String = "{\"on\":true, \"bri\":100, \"seg\":{\"i\":[".to_owned();
        let json_tail: String = "]}}".to_owned();

        let mut json_full: String = json_header + &self.data;
        json_full.push_str(&json_tail);

        let mut headers = http::HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_str("curl/7.85.0").unwrap());
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_str("application/json").unwrap(),
        );

        let client = Client::new();
        let res = client
            .post(url)
            .headers(headers)
            .body(json_full)
            .send()
            .await?;
        Ok(res)
    }

    fn load_image(&mut self, path: &str) {
        // here we define the error message we want to provide if image::open()
        // fails for whatever reason.
        let path_err: &str = "Image path incorrect, or file does not exist.";

        let img = image::open(path).expect(path_err);

        // loop_buffer will store our correctly formatted pixel values as a String
        let mut loop_buffer: String = std_default::default();

        // start_counter will track the first occurrence of a pixel. Once the loop detects
        // a new color, it will use this value to determine the first and last occurrence
        // of the adjacent color block.
        let mut start_counter: u32 = 0;

        // pixel will store the last pixel value, so we can compare the pixel we're working
        // with in the loop (data_chunk).
        let mut pixel: [u8; 4] = [0, 0, 0, 0];

        // despite the fact that img_len can only ever be 255 right now, I have included
        // the logic, so that in the future, it work with different sized images, and
        // different sized arrays of physical LEDs.
        let (img_x, img_y) = img.dimensions();
        let img_len = img_x * img_y - 1;

        for (current_counter, data_chunk) in (0_u32..).zip(img.pixels()) {
            // should only ever be true if we are on the last iteration of the loop.
            let end_of_loop = { current_counter >= img_len };


            if pixel != (data_chunk.2 .0) && current_counter > 0 {
                // cast the rgb values to their respective variables, and ignore the fourth value
                // in data_chunk. A pixel in a DynamicImage is actually represented by XY
                // coordinates, and RGBA values, e.g. `0, 1, [255, 255, 255, 255]`. We can choose to
                // only grab the element in position 2, the list with 4 values.
                let [r, g, b, _] = pixel;

                let pixel_to_json = format!(
                    "{},{},[{},{},{}], ",
                    start_counter, current_counter, r, g, b
                );
                start_counter = current_counter;
                // println!("{}", pixel_to_json);

                loop_buffer.push_str(&pixel_to_json);
            } else if end_of_loop {
                let [r, g, b, _] = pixel;
                let pixel_to_json = format!(
                    "{},{},[{},{},{}], ",
                    start_counter,
                    { current_counter + 1 },
                    r,
                    g,
                    b
                );

                loop_buffer.push_str(&pixel_to_json);
            }

            // at the end of every iteration, we want to update the pixel value to compare next
            // time
            pixel = data_chunk.2 .0;
        }

        loop_buffer = loop_buffer.chars().take(loop_buffer.len() - 2).collect();
        self.data = loop_buffer.to_string();
    }

    async fn request_device_state(&mut self) {

    }

}

/// wled-control is a CLI for controlling your WLED-powered devices!
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

    /// Brightness to set for the panel. Must be a number 0-255. NOTE: Very small numbers may cause
    /// the panel to not light up.
    #[arg(short, long="bright")]
    brightness: Option<u8>,

    /// Whether to turn the panel off or not.
    #[arg(short, long)]
    on: Option<bool>,

    /// IP address or hostname of the WLED device to control.
    #[arg(short, long)]
    ip: Option<IpAddr>,

    /// Path to the image you would like to send.
    #[arg(short, long)]
    path: Option<PathBuf>,

    /// Width of the panel, in number of LEDs.
    #[arg(short='W', long)]
    width: Option<u32>,

    #[arg(short='H', long)]
    /// Height of the panel, in number of LEDs.
    height: Option<u32>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("{:#?}", args);


}
