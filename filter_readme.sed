# Taken from https://github.com/porglezomp/pixel-canvas/blob/develop/gen-readme.sh

# Remove footer-reference-style doc links like "[`Foo`]: ./foo/trait.Foo.html"
/\[.+\]: .*(struct|enum|trait|type|fn|index)\./d

# Remove inline-style doc links like "[`Foo`](./foo/trait.Foo.html)",
# leaving just "`Foo`" in its place
s/\[(.+)\]\(.*(struct|enum|trait|type|fn|index).*\)/\1/g

# Remove square braces from footer-reference-style inline links like "[`Foo`]",
# leaving "`Foo`" in its place
s/\[(`[^]]*`)\]([^\(:]|$)/\1\2/g
