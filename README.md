# TermViewer
Image viewer in Rust  
The goal of this project is for me to learn images and videos decoding  
It is not to be able to display every image or every video in the terminal  
For now, the code support
- non-progressive jpeg
- OS/2 1.x BITMAPCOREHEADER bitmap header
- the images in `tests` folder

Progressive jpeg or other bitmap headers might be implemented in the future  
The current goal is to decode and display videos  

# Compiling
```bash
cargo build --release
```

# Usage
Print help
```bash
./target/release/term_viewer --help
```

Display an image in the terminal
```bash
./target/release/term_viewer <file> <image type>

# Example
./target/release/term_viewer tests/jpeg/cat.jpg jpeg
```

Save the intermediate BMP file created
```bash
./target/release/term_viewer --save-bmp <file> <image type>
```
