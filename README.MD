# gltch - A pixel sorting and glitch art tool

Ever wanted to use pixel sorting or glitch art techniques on your art but didn't want to wait 30 seconds for every modification to your python script to run? This is the tool for you! Gltch is a fast and completely customizable pixel sorting and glitch art tool written in Rust. It uses the power of compute shaders to do pixel sorting in real time, allowing you to see the results of your changes instantly.


## Planned Features

- GPU-accelerated pixel sorting using headless [WGPU](https://wgpu.rs/) and compute shaders
- A nice [Iced](https://iced.rs/) GUI for tweaking parameters and loading/saving images
- Presets for common pixel sorting and glitch art styles
- Support for loading and saving common image formats using [image-rs](https://github.com/image-rs/image)
- A file format to encode everything above for easy sharing of presets and images
- All done using cross-platform Rust libraries for maximum compatibility

## Distant Future Features

- Animation/video support
    - Will probably use [ffmpeg](https://ffmpeg.org/) and an actual fragment shader pipeline for this.
    - Probably not bundle the video with the before mentioned file format, handle it closer to premiere project files.
- On the-fly shader compilation for custom pixel sorting algorithms
    - Should we save a text version of the shader in the file format or compile it to SPIR-V and save that? Does it just need to be SPIR-V or would we have to save multiple compiled versions for different backends?
    - Have the ability to export a opaque shader blob that is just the compiled shader + metadata + maybe the source code that can be distributed separately from the main program.
    - How does sandboxing work here? Can shader code be malicious?
- Plugin system for user-created effects