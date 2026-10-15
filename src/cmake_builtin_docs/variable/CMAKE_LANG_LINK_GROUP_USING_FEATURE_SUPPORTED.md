CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_GROUP_USING_FEATURE_SUPPORTED.html)

# CMAKE_\<LANG\>_LINK_GROUP_USING_\<FEATURE\>_SUPPORTED

Added in version 3.24.

This variable specifies whether the `<FEATURE>` is supported for the link language `<LANG>`. If this variable is true, then the `<FEATURE>` must be defined by [`CMAKE_<LANG>_LINK_GROUP_USING_<FEATURE>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_GROUP_USING_FEATURE.html#variable:CMAKE_%3CLANG%3E_LINK_GROUP_USING_%3CFEATURE%3E), and the more generic [`CMAKE_LINK_GROUP_USING_<FEATURE>_SUPPORTED`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LINK_GROUP_USING_FEATURE_SUPPORTED.html#variable:CMAKE_LINK_GROUP_USING_%3CFEATURE%3E_SUPPORTED) and [`CMAKE_LINK_GROUP_USING_<FEATURE>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LINK_GROUP_USING_FEATURE.html#variable:CMAKE_LINK_GROUP_USING_%3CFEATURE%3E) variables are not used.

If `CMAKE_<LANG>_LINK_GROUP_USING_<FEATURE>_SUPPORTED` is false or is not set, then the [`CMAKE_LINK_GROUP_USING_<FEATURE>_SUPPORTED`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LINK_GROUP_USING_FEATURE_SUPPORTED.html#variable:CMAKE_LINK_GROUP_USING_%3CFEATURE%3E_SUPPORTED) variable will determine whether `<FEATURE>` is deemed to be supported.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
