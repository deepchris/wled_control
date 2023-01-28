#![allow(dead_code)]

use image::{GenericImageView};
use reqwest::header::{HeaderValue, CONTENT_TYPE, USER_AGENT};
use reqwest::*;
use clap::{arg, Parser};
use std::default::{Default as std_default};
use std::net::{IpAddr};
use std::path::{PathBuf};
use std::str;
use image::imageops::Lanczos3;

/// # wled-control
/// `wled-control` is a (for-now) JSON API CLI that sends fully formed image
/// commands (and more!) directly to your WLED-powered device.

struct WledValue {
    brightness: Option<u8>,
    on: Option<bool>,
    ip: Option<IpAddr>,
    data: String,
    height: Option<u32>,
    width: Option<u32>,
}

impl WledValue {

    // TODO: Make ip, height, and width `Option<>`s, so that I can get rid of these pesky expects.
    //  Instead of crashing, I would like the program to prompt the user for the missing values.
    fn from_args(args: &Args) -> WledValue {
        WledValue {
            brightness: args.brightness,
            on: args.on,
            ip: args.ip,
            data: std_default::default(),
            height: args.height,
            width: args.width,
        }
    }

    /// send_updates sends the information contained in the WledValue to the WLED device via
    /// the JSON API.
    async fn send_updates(&mut self) -> std::result::Result<Response, Error> {

        let url = format!("http://{:?}/json/state", self.ip);

        let json_header: String = "{\"on\":true, \"bri\":100, \"seg\":{\"i\":[".to_owned();
        let json_tail: String = "]}}".to_owned();

        let mut json_full: String = json_header + &self.data;
        json_full.push_str(&json_tail);

        // TODO: Remove unwrap, handle errors in more robust manner.
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

    // TODO: Return errors instead of panicking.
    /// load_image takes a path, and loads the image into
    fn load_image(&mut self, path: &PathBuf, args: &Args) -> std::result::Result<(), Error> {
        // here we define the error message we want to provide if image::open()
        // fails for whatever reason.
        let path_err: &str = "Image path incorrect, or file does not exist.";

        // Open the image using the path supplied in the command-line arguments. Panic and throw an
        // error if the image cannot be found.
        // TODO: Remove .expect() call
        let img = image::open(path).expect(path_err);

        // cast the image we open's dimensions to variables to make later user easier.
        let (img_w, img_h) = img.dimensions();

        // cast the LED panel's dimensions to variables, also for convenience later on.
        // TODO: Remove .unwrap() call, emit error message if width and height aren't given.
        let (led_w, led_h) = (self.width.unwrap(), self.height.unwrap());

        // this is the auto-resizing logic. It checks if the number of pixels in the image is the
        // same as the LED panel. If they're different, but have the same aspect ratio, just do a
        // high quality resize. If the aspect ratio is different, the image can either just be re
        // -sized (which will result in a warped image), or the image can be cropped first. The
        // crop flag is how the user decides whether to crop first, or just embrace the squish.
        let img = if (led_w != img_w) || (led_h != img_h) {
                // the logic above checks if the aspect ratio of the image is different from the
                // LED panel.
            if (img_w as f32 / img_h as f32) != (led_w as f32 / led_h as f32) && args.crop {
                // resize without distorting the image. crop first, then resize.
                img.resize_to_fill(led_w, led_h, Lanczos3)
            } else {
                // this is the arm where we resize without constraining proportions
                img.resize_exact(led_w, led_h, Lanczos3)
            }
        } else {
            // this arm can only be reached if the image is already the same size as the LEDs
            // if the image is already the same size as the LEDs, don't change it.
            img
        };

        // loop_buffer will store our correctly formatted pixel values as a String
        let mut converted_value_stack: String = std_default::default();

        // start_counter will track the first occurrence of a pixel. Once the loop detects
        // a new color, it will use this value to determine the first and last occurrence
        // of the adjacent color block.
        let mut sequence_start_counter: u32 = 0;

        // pixel will store the last pixel value, so we can compare the pixel we're working
        // with in the loop (data_chunk).
        let mut last_pixel_value: [u8; 4] = [0, 0, 0, 0];

        // despite the fact that img_len can only ever be 255 right now, I have included
        // the logic, so that in the future, it work with different sized images, and
        // different sized arrays of physical LEDs.
        let (img_x, img_y) = img.dimensions();
        let img_len = img_x * img_y - 1;

        for (current_counter, color_array) in (0_u32..).zip(img.pixels()) {
            // should only ever be true if we are on the last iteration of the loop.
            let end_of_loop = { current_counter >= img_len };


            if last_pixel_value != (color_array.2 .0) && current_counter > 0 {
                // cast the rgb values to their respective variables, and ignore the fourth value
                // in data_chunk. A pixel in a DynamicImage is actually represented by XY
                // coordinates, and RGBA values, e.g. `0, 1, [255, 255, 255, 255]`. We can choose to
                // only grab the element in position 2, the list with 4 values.
                let [r, g, b, _] = last_pixel_value;

                let pixel_to_json = format!(
                    "{},{},[{},{},{}], ",
                    sequence_start_counter, current_counter, r, g, b
                );
                sequence_start_counter = current_counter;
                // println!("{}", pixel_to_json);

                converted_value_stack.push_str(&pixel_to_json);
            } else if end_of_loop {
                let [r, g, b, _] = last_pixel_value;
                let pixel_to_json = format!(
                    "{},{},[{},{},{}], ",
                    sequence_start_counter,
                    { current_counter + 1 },
                    r,
                    g,
                    b
                );

                // append the value built up for this pixel to converted_value_stack
                converted_value_stack.push_str(&pixel_to_json);
            }

            // at the end of every iteration, we want to update the pixel value to compare next
            // time
            last_pixel_value = color_array.2 .0;
        }

        // as a side effect of our pure string conversion above, we have an extraneous `,` that is
        // removed below, by recasting converted_value_stack to itself minus the very last char.
        converted_value_stack = converted_value_stack.chars().take(converted_value_stack.len() - 2).collect();
        // set self.data to our final converted string of values for later use.
        self.data = converted_value_stack.to_string();

        Ok(())
    }

    // TODO: Do something with the data in the device state. Turn it into a visualization or
    //  something.
    async fn request_device_state(&mut self) -> std::result::Result<Response, Error> {

        let url = format!("http://{:?}/json/state", self.ip);

        // TODO: Remove unwrap, handle errors in more robust manner.
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
            .send()
            .await?;
        Ok(res)
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

    /// If you enable this flag, the image will be cropped instead of being squished, during resizing.
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    crop: bool,

}

#[tokio::main]
async fn main() {

    // Cast the command line arguments to a variable
    let args = Args::parse();

    // Instantiate a new WledValue, fill in whatever arguments are available.
    let mut wled = WledValue::from_args(&args);

    // If a path is provided, load the image. The error isn't handled right now.
    // Else, print an error, and return early
    if let Some(x) = &args.path {
        // TODO: Handle this error instead of casting it to `_`
        let _ = wled.load_image(x, &args);
        // after the image is parsed, send the new WledValue values to the LED controller
        let response = wled.send_updates().await;
        println!("Response was:\n{:?}", response);
    } else {
        eprintln!("Error: A path to an image is required for what you are trying to do.");
    }

}
