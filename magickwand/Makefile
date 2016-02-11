all: example

example: libimage.a example.c
	gcc example.c -o $@ libimage.a `pkg-config --cflags --libs MagickWand`

libimage.a: image.h image.c
	gcc -c -fPIC image.c `pkg-config --cflags --libs MagickWand`
	ar crs libimage.a image.o

clean:
	rm -f image.o libimage.a example
