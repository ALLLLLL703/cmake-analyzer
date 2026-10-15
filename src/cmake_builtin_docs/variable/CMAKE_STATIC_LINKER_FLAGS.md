CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_STATIC_LINKER_FLAGS.html)

# CMAKE_STATIC_LINKER_FLAGS

Flags to be used to create static libraries. These flags will be passed to the archiver when creating a static library.

See also [`CMAKE_STATIC_LINKER_FLAGS_<CONFIG>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_STATIC_LINKER_FLAGS_CONFIG.html#variable:CMAKE_STATIC_LINKER_FLAGS_%3CCONFIG%3E).

Note

Static libraries do not actually link. They are essentially archives of object files. The use of the name "linker" in the name of this variable is kept for compatibility.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
