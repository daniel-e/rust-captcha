#include <stdlib.h>
#include <stdint.h>
#include <wand/MagickWand.h>

void draw_on_buf(
  void* buf, size_t x, size_t y, size_t width, size_t height, double angle,
  size_t font_size,
  const char* fgcolor,
  const char* font,
  const char* txt);

void init_image();
void done_image();
