# ASCII Art

Simple test project for converting input images into ASCII art.

## What is ASCII Art?
ASCII art is a graphic design technique that uses computers for presentation and consists of pictures pieced together from the 95 printable characters defined by the ASCII Standard from 1963 and ASCII compliant character sets with proprietary extended characters.
> From [ASCII art - Wikipedia](https://en.wikipedia.org/wiki/ASCII_art)

## How it works

Run ascii_art.exe, and provide a URL to a reasonably small png file (128x128 or smaller is ideal). The result will appear in result/result.txt  
The program takes each individual pixel and averages the RGB values for a total brightness score (the closer to white (255,255,255) a color is, the brighter it will be). Higher brightness pixels get higher "density" characters (higher density characters cover more of their allotted space, e.x. "@" is more dense than "/")
<br>
## V3

V3 may include:  
* Dynamic resolution scaling,
* Local file input,  
and is not guarunteed to ever release (I might stop the project after some fixes to V2).

## Contributors

Reilley Pfrimmer (b1nwal)
