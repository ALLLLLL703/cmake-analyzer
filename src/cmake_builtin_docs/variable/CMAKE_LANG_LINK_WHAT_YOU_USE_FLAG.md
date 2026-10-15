CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_WHAT_YOU_USE_FLAG.html)

# CMAKE_\<LANG\>_LINK_WHAT_YOU_USE_FLAG

Added in version 3.22.

Linker flag used by [`LINK_WHAT_YOU_USE`](https://cmake.org/cmake/help/v4.4/prop_tgt/LINK_WHAT_YOU_USE.html#prop_tgt:LINK_WHAT_YOU_USE) to tell the linker to link all shared libraries specified on the command line even if none of their symbols is needed. This is an implementation detail used so that the command in [`CMAKE_LINK_WHAT_YOU_USE_CHECK`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LINK_WHAT_YOU_USE_CHECK.html#variable:CMAKE_LINK_WHAT_YOU_USE_CHECK) can check the binary for unnecessarily-linked shared libraries.

Note

Do not rely on this abstraction to intentionally link to shared libraries whose symbols are not needed.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
