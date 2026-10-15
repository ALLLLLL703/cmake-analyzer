CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LINK_GROUP_USING_FEATURE_SUPPORTED.html)

# CMAKE_LINK_GROUP_USING_\<FEATURE\>_SUPPORTED

Added in version 3.24.

This variable specifies whether the `<FEATURE>` is supported regardless of the link language. If this variable is true, then the `<FEATURE>` must be defined by [`CMAKE_LINK_GROUP_USING_<FEATURE>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LINK_GROUP_USING_FEATURE.html#variable:CMAKE_LINK_GROUP_USING_%3CFEATURE%3E).

Note that this variable has no effect if [`CMAKE_<LANG>_LINK_GROUP_USING_<FEATURE>_SUPPORTED`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_GROUP_USING_FEATURE_SUPPORTED.html#variable:CMAKE_%3CLANG%3E_LINK_GROUP_USING_%3CFEATURE%3E_SUPPORTED) is true for the link language of the target.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
