#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt::format;
use std::net;
use std::net::{IpAddr, Ipv4Addr};
use std::str;
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

    // this is the builder function that lets you create an instance of WledValue
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

    // `off` is an async function that turns off the controller. It only reads
    // the `ip` field from the WledValue instance, and sends the correct
    // command. It doesn't care about the rest of the fields.
    pub async fn off(&self) {
        // creates a string called url that contains the correct URL
        let url = format!("http://{:?}/json", self.ip);
        let mut map = HashMap::new();

        map.insert("on", "false");
        map.insert("bri", "0");

        let client = reqwest::Client::new();
        let res = client.post(url)
            .json(&map)
            .send()
            .await?;
    }


    pub async fn change_image(&mut self, img: DynamicImage) {

        // change_image is the function that updates the image. It takes one value, a DynamicImage
        // (a way of representing an image as a 2D array of pixels). The other value `&mut self`
        // means that the method can mess with the data inside the WledValue struct that you call
        // the method on.

        // this is the method that needs its logic implemented. The for loop is already set up to
        // iterate across every pixel in the DynamicImage, and I already created two variables you
        // can use as counters.

        let url = format!("http://{:?}/json", self.ip);
        let mut map: HashMap<&str, &str> = HashMap::new();

        let mut loop_buffer = "";

        let mut start_counter: u32 = 0;
        let mut current_counter: u32 = 0;

        // currently, the for loop will iterate across every single pixel, print the value,
        // and then increment `current_counter.` Use method `.append()` on string `loop_buffer`
        // as in the commented example below to add more values to the buffer.
        for data_chunk in img.pixels() {
            // to figure out why I used the strange expression data_chunk.2.0, try changing
            // the value in println!() from `pixel` to `data_chunk`, and commenting out the
            // line below.
            let pixel = (data_chunk.2.0);

            println!("{:?}", pixel);

            loop_buffer = loop_buffer.append("value".to_owned());

            current_counter += 1;
        }
        // after you convert the logic from an array of pixels to the compressed form, this
        // expression below will set the WledValue's `data` field to the value you computed
        // in the for loop, and stored in `map`.
        self.data = map;
    }
}

// TODO: Add argument handling.
#[tokio::main]
// the return type `Result<()>` is in place because async functions do not know
// when they will be done, and the function may fail (since you're reaching out
// to the internet.
async fn main() -> Result<()> {
    // let json_msg = json!({"on":true, "bri":255, "seg":{"i":[
    // 0,23,[0,176,240], 23,25,[255,255,255], 25,38,[0,176,240], 38,42,[255,255,255], 42,53,[0,176,240], 53,59,[255,255,255], 59,68,[0,176,240], 68,72,[255,255,255], 72,[0,176,240], 73,76,[255,255,255], 76,83,[0,176,240], 83,87,[255,255,255], 87,[0,176,240], 88,[255,255,255], 89,[0,176,240], 90,93,[255,255,255], 93,98,[0,176,240], 98,104,[255,255,255], 104,[0,176,240], 105,110,[255,255,255], 110,113,[0,176,240], 113,116,[255,255,255], 116,[0,176,240], 117,120,[255,255,255], 120,[0,176,240], 121,127,[255,255,255], 127,[0,176,240], 128,131,[255,255,255], 131,[0,176,240], 132,[255,255,255], 133,[0,176,240], 134,136,[255,255,255], 136,[0,176,240], 137,139,[255,255,255], 139,[0,176,240], 140,144,[255,255,255], 144,146,[0,176,240], 146,148,[255,255,255], 148,[0,176,240], 149,152,[255,255,255], 152,[0,176,240], 153,[255,255,255], 154,[0,176,240], 155,[255,255,255], 156,[0,176,240], 157,[255,255,255], 158,162,[0,176,240], 162,165,[255,255,255], 165,167,[0,176,240], 167,[255,255,255], 168,[0,176,240], 169,171,[255,255,255], 171,[0,176,240], 172,174,[255,255,255], 174,178,[0,176,240], 178,183,[255,255,255], 183,185,[0,176,240], 185,[255,255,255], 186,[0,176,240], 187,190,[255,255,255], 190,194,[0,176,240], 194,200,[255,255,255], 200,202,[0,176,240], 202,206,[255,255,255], 206,210,[0,176,240], 210,216,[255,255,255], 216,[0,176,240], 217,222,[255,255,255], 222,256,[0,176,240]
    // ]}});
    let path = "/Users/alex/rust-projects/wled_control/src/example_cat.png";
    let img = load_image(path).unwrap();

    use std::default::*;

    let wled_ip: IpAddr = IpAddr::V4(Ipv4Addr::new(192, 168, 5, 52));

    // creating a new WledValue instance named wled. After this, methods can be called like
    // `wled.load_image(foo, bar, etc)`
    let wled = WledValue::new(255, true, wled_ip, HashMap::new());

    Ok(())
}

fn load_image(path: &str) -> Option<DynamicImage> {

    // here we define the error message we want to provide if image::open()
    // fails for whatever reason.
    let path_err: &str = "Image path incorrect, or file does not exist.";


    // open the image, return none if error. `.expect()` prints an error
    // message if the code fails to do something (in this case, open an
    // image based on the path we provided). The image:: part of image::open
    // refers to the crate it came from, image! In this case, open is the
    // name of a method, but we want to make sure that the compiler knows we
    // want the open() specifically provided by the crate image.

    // open() is a common name for a method, so the :: is a way to tell the
    // compiler that we need this one for our code to work!
    if let img = image::open(path).expect(path_err) {

        // TODO: Implement automatic image resizing, with CLI args available for whatever settings seem relevant.
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
    // if we ended up here, something went wrong. We should return None, since the program
    // can only succeed if it flows through the logic above.
    return None;
}