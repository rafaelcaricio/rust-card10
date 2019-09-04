Draw Image
==========

Images need to be converted to a bitmap of 16BPP for inclusion with `include_bytes!()`. You can convert an image using:

```
convert image.png -flip -type truecolor -define bmp:subtype=RGB565 -depth 16 -strip image.bmp
```

then

```
tail -c $bytes image.bmp > image.raw // where $bytes is w * h * 2
```

This will remove the BMP header leaving the raw pixel data E.g 160x80 image will have 160 * 80 * 2 bytes of raw data.