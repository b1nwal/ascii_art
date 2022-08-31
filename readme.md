# ASCII Art

Simple test project for converting input images into ASCII art.

## How it works

Not user friendly yet; change the url variable inside the main.rs file to a url with a png of reasonably small size (for scaling), build and run.
The program scans each pixel and averages the RGB value for a total brightness score (consider that 255,255,255 is white, the brightest color and thus as you approach these values you must be getting brighter,) and then outputs a character to represent that pixel. The higher brightness the pixel is the more "dense" the character will be (e.g. "@" is more dense than "-").

## V3

V3 may include:  
* Dynamic resolution scaling,
* Local file input,  
and is not guarunteed to ever release (I might stop the project after some fixes to V2).

## Contributors

Reilley Pfrimmer (b1nwal)
