CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_IMPLICIT_LINK_LIBRARIES.html)

# CMAKE_\<LANG\>_IMPLICIT_LINK_LIBRARIES

Implicit link libraries and flags detected for language `<LANG>`.

Compilers typically pass language runtime library names and other flags when they invoke a linker. These flags are implicit link options for the compiler's language.

For each language enabled by the [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) or [`enable_language()`](https://cmake.org/cmake/help/v4.4/command/enable_language.html#command:enable_language) command, CMake automatically detects these libraries and flags and reports the results in this variable. The [`CMAKE_<LANG>_IMPLICIT_LINK_LIBRARIES_EXCLUDE`](https://cmake.org/cmake/help/v4.4/envvar/CMAKE_LANG_IMPLICIT_LINK_LIBRARIES_EXCLUDE.html#envvar:CMAKE_%3CLANG%3E_IMPLICIT_LINK_LIBRARIES_EXCLUDE) environment variable may be set to exclude specific libraries from the automatically detected results.

When linking to a static library, CMake adds the implicit link libraries and flags from this variable for each language used in the static library (except the language whose compiler is used to drive linking). In the case of an imported static library, the [`IMPORTED_LINK_INTERFACE_LANGUAGES`](https://cmake.org/cmake/help/v4.4/prop_tgt/IMPORTED_LINK_INTERFACE_LANGUAGES.html#prop_tgt:IMPORTED_LINK_INTERFACE_LANGUAGES) target property lists the languages whose implicit link information is needed. If any of the languages is not enabled, its value for the `CMAKE_<LANG>_IMPLICIT_LINK_LIBRARIES` variable may instead be provided by the project. Or, a [`toolchain file`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TOOLCHAIN_FILE.html#variable:CMAKE_TOOLCHAIN_FILE) may set the variable to a value known for the specified toolchain. It will either be overridden when the language is enabled, or used as a fallback.

See also the [`CMAKE_<LANG>_IMPLICIT_LINK_DIRECTORIES`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_IMPLICIT_LINK_DIRECTORIES.html#variable:CMAKE_%3CLANG%3E_IMPLICIT_LINK_DIRECTORIES) variable.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
