#![allow(dead_code)]

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::str;
use std::default::Default as std_default;
use image::{DynamicImage, GenericImageView};
use reqwest::*;
use reqwest::header::{CONTENT_TYPE, HeaderValue, USER_AGENT};

/// # wled-control
/// `wled-control` is a (for-now) JSON API CLI that sends fully formed image
/// commands (and more!) directly to your WLED-powered device.
///
/// ## To Implement:
/// - [x] exactly 16x16 pixels image loading
/// - [x] `off` function (turns the device off, only needs the IP address)
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
}

impl WledValue {

    // this is the builder function that lets you create an instance of WledValue
    pub fn new(brightness: Option<u8>, on: Option<bool>, ip: IpAddr) -> WledValue {
        WledValue {
            brightness,
            on,
            client: None,
            ip,
            data: String::default(),
        }
    }

    // `off` is an async function that turns off the controller. It only reads
    // the `ip` field from the WledValue instance, and sends the correct
    // command. It doesn't care about the rest of the fields.
    pub async fn off(&self) -> std::result::Result<Response, reqwest::Error> {
        // creates a string called url that contains the correct URL
        let url = format!("http://{:?}/json/state", self.ip);
        let mut map = HashMap::new();

        map.insert("on", "false");
        map.insert("bri", "0");

        let client = Client::new();
        let res = client.post(url)
            .json(&map)
            .send()
            .await?;
        Ok(res)
    }

    pub async fn change_image(&mut self, img: DynamicImage)
        -> std::result::Result<Response, reqwest::Error> {

        // loop_buffer will store our correctly formatted pixel values as a String
        let mut loop_buffer: String = std_default::default();

        // start_counter will track the first occurrence of a pixel. Once the loop detects
        // a new color, it will use this value to determine the first and last occurrence
        // of the adjacent color block.
        let mut start_counter: u32 = 0;

        // pixel will store the last pixel value, so we can compare the pixel we're working
        // with in the loop (data_chunk).
        let mut pixel: [u8;4] = [0, 0, 0, 0];

        // despite the fact that img_len can only ever be 255 right now, I have included
        // the logic, so that in the future, it work with different sized images, and
        // different sized arrays of physical LEDs.
        let (img_x, img_y) = img.dimensions();
        let img_len = img_x * img_y - 1;

        for (current_counter, data_chunk) in (0_u32..).zip(img.pixels()) {

            // should only ever be true if we are on the last iteration of the loop.
            let end_of_loop = {current_counter >= img_len};

            // TODO: Implement an end_of_loop if statement after the one directly below this
            // todo message, to include the last pixel
            if pixel != (data_chunk.2.0) && current_counter > 0 {
                // cast the rgb values to their respective variables, and ignore the fourth value
                // in data_chunk. A pixel in a DynamicImage is actually represented by XY
                // coordinates, and RGBA values, e.g. `0, 1, [255, 255, 255, 255]`. We can choose to
                // only grab the element in position 2, the list with 4 values.
                let [r, g, b, _] = pixel;

                let pixel_to_json = format!("{},{},[{},{},{}], ", start_counter, current_counter, r, g, b);
                start_counter = current_counter;
                // println!("{}", pixel_to_json);

                loop_buffer.push_str(&pixel_to_json);

            } else if end_of_loop {

                let [r, g, b, _] = pixel;
                let pixel_to_json = format!("{},{},[{},{},{}], ", start_counter, { current_counter + 1 }, r, g, b);

                loop_buffer.push_str(&pixel_to_json);
            }

            // at the end of every iteration, we want to update the pixel value to compare next
            // time
            pixel = data_chunk.2.0;
        }

        let loop_buffer_len = loop_buffer.len();
        loop_buffer = loop_buffer.chars().take(loop_buffer_len - 2).collect();

        self.data = loop_buffer.to_string();

        let url = format!("http://{:?}/json/state", self.ip);

        let json_header: String = "{\"on\":true, \"bri\":100, \"seg\":{\"i\":[".to_owned();
        let json_tail: String = "]}}".to_owned();

        let mut json_full: String = json_header + &loop_buffer;
        json_full.push_str(&*json_tail);

        let mut headers = http::HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_str("curl/7.85.0").unwrap());
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap());

        let client = Client::new();
        let res = client
            .post(url)
            .headers(headers)
            .body(json_full)
            .send()
            .await?;
        Ok(res)
    }
}


#[tokio::main]
async fn main() {
    // TODO: Add argument handling.

    let path = "/Users/alex/rust-projects/wled_control/src/HA_logo.png";
    let new_img = load_image(path).unwrap();

    let device_ip: IpAddr = IpAddr::V4(Ipv4Addr::new(192, 168, 5, 102));

    // creating a new WledValue instance named wled. After this, methods can be called like
    // `wled.load_image(foo, bar, etc)`
    let mut wled = WledValue::new(Some(255), Some(true), device_ip);

    let response = wled.change_image(new_img).await;
    println!("{:#?}", response)

}

fn load_image(path: &str) -> Option<DynamicImage> {
    // TODO: Implement automatic image resizing, with CLI args available for whatever settings
    //  seem relevant.

    // here we define the error message we want to provide if image::open()
    // fails for whatever reason.
    let path_err: &str = "Image path incorrect, or file does not exist.";

    // open() is a common name for a method, so the :: is a way to tell the
    // compiler that we need this one for our code to work!
    let img = image::open(path).expect(path_err) ;

    // `return match` returns the result of the match statement. In this
    // case, match will evaluate the dimensions and check to see if it's
    // exactly 16x16, or ANYTHING ELSE (that's what the _ means)
    // if it's anything else, our program can't handel it right now, and
    // needs to return None, representing no valid return value.
    return match img.dimensions() {
        (16, 16) => Some(img),
        _ => {
            // not necessary, but the println here is functioning like an error log.
            println!("Image is not 16x16. Please re-run program with correct size.");
            // note that None has no ; after it. This is because it is the return
            // value of this arm of the match expression. (if the match value is ever
            // NOT 16x16, then it will end up here, and always return None).
            None
        }
    }
}