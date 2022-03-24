# Inspired by https://github.com/porglezomp/pixel-canvas/blob/develop/gen-readme.sh

# Replace local links with links to docs.rs
s#\[(.+)\]\(\.\/(.*)\)#[\1](https://docs.rs/embedded-graphics-core/latest/embedded_graphics_core/\2)#g
s#\[(.+)\]\: +\.\/(.*)#[\1]: https://docs.rs/embedded-graphics-core/latest/embedded_graphics_core/\2#g

# Delete lines containing reference links like `[Foo]: bar::Foo`
/\[([^]]+)\]\: ([A-Za-z_]+::)*[A-Za-z_]+$/d

/<!-- README-LINKS/d
/README-LINKS -->/d
