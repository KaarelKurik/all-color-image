Uses a Hilbert curve to generate a 256x65536 image with all RGB colors varying continuously.
Kind of a shitty method, but I don't presently know of a generic way to lay out a 3D block of values continuously without repeats in 2D with a nice aspect ratio.
https://link.springer.com/article/10.1007/PL00009375 offers some ideas on how to proceed, but it's more of a limit construction and is liable to leave gaps/have repeats in our case.
