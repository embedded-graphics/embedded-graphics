# Delete lines containing reference links like `[Foo]: bar::Foo`
/\[([^]]+)\]\: ([A-Za-z_]+::)*[A-Za-z_]+$/d

/<!-- README-LINKS/d
/README-LINKS -->/d
