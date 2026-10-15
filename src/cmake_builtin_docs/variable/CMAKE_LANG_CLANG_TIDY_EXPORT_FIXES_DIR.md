CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_CLANG_TIDY_EXPORT_FIXES_DIR.html)

# CMAKE_\<LANG\>_CLANG_TIDY_EXPORT_FIXES_DIR

Added in version 3.26.

Default value for [`<LANG>_CLANG_TIDY_EXPORT_FIXES_DIR`](https://cmake.org/cmake/help/v4.4/prop_tgt/LANG_CLANG_TIDY_EXPORT_FIXES_DIR.html#prop_tgt:%3CLANG%3E_CLANG_TIDY_EXPORT_FIXES_DIR) target property when `<LANG>` is `C`, `CXX`, `OBJC` or `OBJCXX`.

This variable is used to initialize the property on each target as it is created. For example:

~~~
set(CMAKE_CXX_CLANG_TIDY_EXPORT_FIXES_DIR clang-tidy-fixes)
add_executable(foo foo.cxx)
~~~

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
