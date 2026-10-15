CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINKER_PREFERENCE.html)

# CMAKE_\<LANG\>_LINKER_PREFERENCE

An internal variable subject to change.

Preference value for linker language selection.

The "linker language" for executable, shared library, and module targets is the language whose compiler will invoke the linker. The [`LINKER_LANGUAGE`](https://cmake.org/cmake/help/v4.4/prop_tgt/LINKER_LANGUAGE.html#prop_tgt:LINKER_LANGUAGE) target property sets the language explicitly. Otherwise, the linker language is that whose linker preference value is highest among languages compiled and linked into the target. See also the [`CMAKE_<LANG>_LINKER_PREFERENCE_PROPAGATES`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINKER_PREFERENCE_PROPAGATES.html#variable:CMAKE_%3CLANG%3E_LINKER_PREFERENCE_PROPAGATES) variable.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
