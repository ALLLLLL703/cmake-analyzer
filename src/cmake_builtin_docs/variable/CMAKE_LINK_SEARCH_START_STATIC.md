CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LINK_SEARCH_START_STATIC.html)

# CMAKE_LINK_SEARCH_START_STATIC

Added in version 3.4.

Assume the linker looks for static libraries by default.

Some linkers support switches such as `-Bstatic` and `-Bdynamic` to determine whether to use static or shared libraries for `-lXXX` options. CMake uses these options to set the link type for libraries whose full paths are not known or (in some cases) are in implicit link directories for the platform. By default the linker search type is assumed to be `-Bdynamic` at the beginning of the library list. This property switches the assumption to `-Bstatic`. It is intended for use when linking an executable statically (e.g. with the GNU `-static` option).

This variable is used to initialize the target property [`LINK_SEARCH_START_STATIC`](https://cmake.org/cmake/help/v4.4/prop_tgt/LINK_SEARCH_START_STATIC.html#prop_tgt:LINK_SEARCH_START_STATIC) for all targets. If set, its value is also used by the [`try_compile()`](https://cmake.org/cmake/help/v4.4/command/try_compile.html#command:try_compile) command.

See also [`CMAKE_LINK_SEARCH_END_STATIC`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LINK_SEARCH_END_STATIC.html#variable:CMAKE_LINK_SEARCH_END_STATIC).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
