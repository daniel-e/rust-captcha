#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include "image.h"

int main() {
  size_t width = 200;
  size_t height = 200;

  init_image();

  void* buffer = malloc(width * height * 3);
  memset(buffer, 255, width * height * 3);
  draw_on_buf(buffer, 50, 100, width, height, 10, 72, "blue", "Verdana-Bold-Italic", "Y");

  // save image to disk
  MagickWand* w = NewMagickWand();
  MagickConstituteImage(w, width, height, "RGB", CharPixel, buffer);
  MagickWriteImage(w, "/tmp/a.jpg");
  DestroyMagickWand(w);

  done_image();
}
