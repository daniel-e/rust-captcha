#include "image.h"

// http://members.shaw.ca/el.supremo/MagickWand/text_effects.htm

void draw_text(
  MagickWand* w, DrawingWand* d, size_t posx, size_t posy, double angle,
  const char* font, size_t siz, const char* color, const char* text) {

  PixelWand* p = NewPixelWand();
  DrawSetFont (d, font);
  DrawSetFontSize(d, siz);
  DrawSetTextAntialias(d, MagickTrue);
  PixelSetColor(p, color);
  DrawSetFillColor(d, p);
  MagickAnnotateImage(w, d, posx, posy, angle, text);
  DestroyPixelWand(p);
}

MagickWand* draw_on_buf(
  void* buf, size_t width, size_t height, double angle,
  size_t font_size,
  const char* fgcolor,
  const char* font,
  const char* txt) {

  MagickWand* w = NewMagickWand();
  DrawingWand* d = NewDrawingWand();

  MagickConstituteImage(w, width, height, "RGB", CharPixel, buf);
  draw_text(w, d, 50, 100, angle, font, font_size, fgcolor, txt);
  MagickExportImagePixels(w, 0, 0, width, height, "RGB", CharPixel, buf);

  DestroyMagickWand(w);
  DestroyDrawingWand(d);
}

void init_image() {
  MagickWandGenesis();
}

void done_image() {
  MagickWandTerminus();
}
