# Inspired by https://github.com/porglezomp/pixel-canvas/blob/develop/gen-readme.sh

# Replace local links with links to docs.rs
s#\[(.+)\]\(\.\/(.*)\)#[\1](https://docs.rs/embedded-graphics/latest/embedded_graphics/\2)#g
s#\[(.+)\]\: +\.\/(.*)#[\1]: https://docs.rs/embedded-graphics/latest/embedded_graphics/\2#g

