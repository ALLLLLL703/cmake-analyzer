CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_ARCHIVE_FINISH.html)

# CMAKE_\<LANG\>_ARCHIVE_FINISH

Rule variable to finish an existing static archive.

This is a rule variable that tells CMake how to finish a static archive. It is used in place of [`CMAKE_<LANG>_CREATE_STATIC_LIBRARY`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_CREATE_STATIC_LIBRARY.html#variable:CMAKE_%3CLANG%3E_CREATE_STATIC_LIBRARY) on some platforms in order to support large object counts. See also [`CMAKE_<LANG>_ARCHIVE_CREATE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_ARCHIVE_CREATE.html#variable:CMAKE_%3CLANG%3E_ARCHIVE_CREATE) and [`CMAKE_<LANG>_ARCHIVE_APPEND`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_ARCHIVE_APPEND.html#variable:CMAKE_%3CLANG%3E_ARCHIVE_APPEND).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
