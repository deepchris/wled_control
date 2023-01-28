<div align="center">

```
  
888       888 888      8888888888 8888888b.        .d8888b.  88888888888 8888888b.  888     
888   o   888 888      888        888  "Y88b      d88P  Y88b     888     888   Y88b 888     
888  d8b  888 888      888        888    888      888    888     888     888    888 888     
888 d888b 888 888      8888888    888    888      888            888     888   d88P 888     
888d88888b888 888      888        888    888      888            888     8888888P"  888     
88888P Y88888 888      888        888    888      888    888     888     888 T88b   888     
8888P   Y8888 888      888        888  .d88P      Y88b  d88P     888     888  T88b  888     
888P     Y888 88888888 8888888888 8888888P"        "Y8888P"      888     888   T88b 88888888

```

</div>

------
<div align="center">

[![GitHub Follow](https://img.shields.io/github/stars/deepchris/wled_control?label=Github+Stars&amp;logo=Github&amp;style=social)](https://github.com/deepchris) 
[![GitHub last commit](https://img.shields.io/github/last-commit/deepchris/wled_control?style=flat-square)](https://github.com/deepchris) 
[![GitHub Fork](https://img.shields.io/github/forks/deepchris/wled_control?label=Fork%20Me%21&style=social)](https://github.com/deepchris/wled_control/fork) 

</div>


`wled-control` is a CLI for [WLED](https://github.com/Aircoookie/WLED)'s JSON [API](https://kno.wled.ge/interfaces/json-api/) that sends fully formed image commands (and more!) directly to your WLED-powered device.

After watching [this](https://www.youtube.com/watch?v=WSex5f1qzH8) youtube video, I built the panel, 3D printed the frame (in a beautiful wood filament, no less!). I appreciate the work the creator did, but his method for converting images for WLED requires a full license to the non-office365 version of Excel. 

My goal was to make this program easy to integrate into Home Assistant, as well as make updating WLED values via integrations or the CLI effortless.

***Contributors welcome! Feel free to fork this project, and I'll review any push requests that come its way.*** I'm still a beginner with rust, so if you would like to suggest improvements to the overall structure of the project, send your push requests my way! I'm also more than happy to just take feedback on the structure of the code, if you're interested in teaching!

## ⚠️ THIS PROJECT IS UNDER CONSTRUCTION! ⚠️

This project still needs a LOT of work!

## TODO

- [x] exactly 16x16 pixels image loading
- [x] `off` function (turns the device off, only takes the IP address)
- [x] a builder function called `from_args`
- [x] Conversion logic from pixels, to WLED JSON API
- [x] Image resizing
- [x] LED panel resizing
- [x] Command line argument handling
- [ ] Saving settings to reduce arg typing
- [ ] A more prettified terminal interface (perhaps with [tui-rs](https://github.com/fdehau/tui-rs)?)

