CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_LIBRARY_USING_FEATURE_SUPPORTED.html)

# CMAKE_\<LANG\>_LINK_LIBRARY_USING_\<FEATURE\>_SUPPORTED

Added in version 3.24.

Set to `TRUE` if the `<FEATURE>`, as defined by variable [`CMAKE_<LANG>_LINK_LIBRARY_USING_<FEATURE>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_LIBRARY_USING_FEATURE.html#variable:CMAKE_%3CLANG%3E_LINK_LIBRARY_USING_%3CFEATURE%3E), is supported for the linker language `<LANG>`.

Note

This variable is evaluated before the more generic variable [`CMAKE_LINK_LIBRARY_USING_<FEATURE>_SUPPORTED`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LINK_LIBRARY_USING_FEATURE_SUPPORTED.html#variable:CMAKE_LINK_LIBRARY_USING_%3CFEATURE%3E_SUPPORTED).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
