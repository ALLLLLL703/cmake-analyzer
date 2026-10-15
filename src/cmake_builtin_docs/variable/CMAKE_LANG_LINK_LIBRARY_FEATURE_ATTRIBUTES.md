CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_LIBRARY_FEATURE_ATTRIBUTES.html)

# CMAKE_\<LANG\>_LINK_LIBRARY_\<FEATURE\>_ATTRIBUTES

Added in version 3.30.

This variable defines the semantics of the specified link library `<FEATURE>` when linking with the link language `<LANG>`. It takes precedence over [`CMAKE_LINK_LIBRARY_<FEATURE>_ATTRIBUTES`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LINK_LIBRARY_FEATURE_ATTRIBUTES.html#variable:CMAKE_LINK_LIBRARY_%3CFEATURE%3E_ATTRIBUTES) if that variable is also defined for the same `<FEATURE>`, but otherwise has similar effects. See [`CMAKE_LINK_LIBRARY_<FEATURE>_ATTRIBUTES`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LINK_LIBRARY_FEATURE_ATTRIBUTES.html#variable:CMAKE_LINK_LIBRARY_%3CFEATURE%3E_ATTRIBUTES) for further details.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
