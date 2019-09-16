## penlib
A collection of characteristics for various pen brands and products.
This library was created to assist in creating SVG images for use by pen
plotters. Pen sizes and colors can be used to more carefully plan spacing
between paths, or to produce SVG images which will more closely resemble
the result of plotting to real paper.

All Pen types implement the `penlib::Pen` trait. Beyond that, each brand may
provide its own traits or generic interfaces in order to specify the nib sizes
and various colors available.