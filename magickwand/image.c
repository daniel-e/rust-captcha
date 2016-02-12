#include "image.h"
#include <pthread.h>

// http://members.shaw.ca/el.supremo/MagickWand/text_effects.htm

void draw_text(
  MagickWand* w, DrawingWand* d, int posx, int posy, double angle,
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

void draw_on_buf(
  void* buf, int x, int y, size_t width, size_t height, double angle,
  size_t font_size,
  const char* fgcolor,
  const char* font,
  const char* txt) {

  MagickWand* w = NewMagickWand();
  DrawingWand* d = NewDrawingWand();

  MagickConstituteImage(w, width, height, "RGB", CharPixel, buf);
  draw_text(w, d, x, y, angle, font, font_size, fgcolor, txt);
  MagickExportImagePixels(w, 0, 0, width, height, "RGB", CharPixel, buf);

  DestroyMagickWand(w);
  DestroyDrawingWand(d);
}

int save_buf(void* buf, size_t width, size_t height, const char* filename) {

  MagickBooleanType r = MagickFalse;
  MagickWand* w = NewMagickWand();
  MagickConstituteImage(w, width, height, "RGB", CharPixel, buf);
  r = MagickWriteImage(w, filename);
  DestroyMagickWand(w);
  return (r == MagickTrue ? 0 : 1);
}

static volatile int initialized = 0;
static pthread_mutex_t lock = PTHREAD_MUTEX_INITIALIZER;

void init_image() {
  pthread_mutex_lock(&lock);
  if (initialized++ == 0) {
    MagickWandGenesis();
  }
  pthread_mutex_unlock(&lock);
}

void done_image() {
  pthread_mutex_lock(&lock);
  if (--initialized == 0) {
    MagickWandTerminus();
  }
  pthread_mutex_unlock(&lock);
}
