#include <stdio.h>
#include <stdlib.h>
#include <wand/MagickWand.h>

// http://members.shaw.ca/el.supremo/MagickWand/text_effects.htm

int main() {
  size_t cols = 300;
  size_t rows = 300;
  MagickWand* image_wand;
  PixelWand* background;
  DrawingWand* drawing_wand;

  MagickWandGenesis();


  background = NewPixelWand();
  PixelSetColor(background, "green");
  drawing_wand = NewDrawingWand();

  image_wand = NewMagickWand();
  MagickNewImage(image_wand, cols, rows, background);

  MagickAnnotateImage(image_wand, drawing_wand, 10, 150, -45, "hello");
  MagickDrawImage(image_wand, drawing_wand);

  MagickWriteImages(image_wand, "a.jpg", MagickTrue);
  DestroyMagickWand(image_wand);


  MagickWandTerminus();
}
