# Inspired by https://github.com/porglezomp/pixel-canvas/blob/develop/gen-readme.sh

# Replace local links with links to docs.rs
s#\[(.+)\]\(\.\/(.*)\)#[\1](https://docs.rs/embedded-graphics-core/latest/embedded_graphics_core/\2)#g
s#\[(.+)\]\: +\.\/(.*)#[\1]: https://docs.rs/embedded-graphics-core/latest/embedded_graphics_core/\2#g

