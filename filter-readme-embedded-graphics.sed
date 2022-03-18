# Inspired by https://github.com/porglezomp/pixel-canvas/blob/develop/gen-readme.sh

# Replace local links with links to docs.rs
s#\[(.+)\]\(\.\/(.*)\)#[\1](https://docs.rs/embedded-graphics/latest/embedded_graphics/\2)#g
s#\[(.+)\]\: +\.\/(.*)#[\1]: https://docs.rs/embedded-graphics/latest/embedded_graphics/\2#g

# Remove intra-doc links, originally from https://github.com/livioribeiro/cargo-readme/issues/70#issuecomment-907867904
# Unwrap inline links, e.g. `[The thing](the::Thing)` -> `The Thing`
s#\[([^]]+)\]\(([A-Za-z_]+::)*[A-Za-z_]+\)#\1#g

# Delete lines containing reference links like `[Foo]: bar::Foo`
/\[([^]]+)\]\: ([A-Za-z_]+::)*[A-Za-z_]+$/d
