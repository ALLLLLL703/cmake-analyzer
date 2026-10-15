CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINKER_PREFERENCE_PROPAGATES.html)

# CMAKE_\<LANG\>_LINKER_PREFERENCE_PROPAGATES

An internal variable subject to change.

True if [`CMAKE_<LANG>_LINKER_PREFERENCE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINKER_PREFERENCE.html#variable:CMAKE_%3CLANG%3E_LINKER_PREFERENCE) propagates across targets.

This is used when CMake selects a linker language for a target. Languages compiled directly into the target are always considered. A language compiled into static libraries linked by the target is considered if this variable is true.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
