CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LINK_WHAT_YOU_USE_CHECK.html)

# CMAKE_LINK_WHAT_YOU_USE_CHECK

Added in version 3.22.

Command executed by [`LINK_WHAT_YOU_USE`](https://cmake.org/cmake/help/v4.4/prop_tgt/LINK_WHAT_YOU_USE.html#prop_tgt:LINK_WHAT_YOU_USE) after the linker to check for unnecessarily-linked shared libraries. This check is currently only defined on `ELF` platforms with value `ldd -u -r`.

See also [`CMAKE_<LANG>_LINK_WHAT_YOU_USE_FLAG`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_WHAT_YOU_USE_FLAG.html#variable:CMAKE_%3CLANG%3E_LINK_WHAT_YOU_USE_FLAG) variables.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
