CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_COMPILER_IS_GNUG77.html)

# CMAKE_COMPILER_IS_GNUG77

Deprecated since version 3.24: Use the [`CMAKE_Fortran_COMPILER_ID`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_COMPILER_ID.html#variable:CMAKE_%3CLANG%3E_COMPILER_ID) variable instead.

The `CMAKE_COMPILER_IS_*` variables were used in early CMake versions before the introduction of [`CMAKE_<LANG>_COMPILER_ID`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_COMPILER_ID.html#variable:CMAKE_%3CLANG%3E_COMPILER_ID) variables in CMake 2.6.

The `CMAKE_COMPILER_IS_GNUG77` variable is set to boolean true if the `Fortran` compiler is GNU.

## Examples

In earlier versions of CMake, the `CMAKE_COMPILER_IS_GNUG77` variable was used to check if the `Fortran` compiler was GNU:

~~~
if(CMAKE_COMPILER_IS_GNUG77)
  # GNU Fortran compiler-specific logic.
endif()
~~~

Starting with CMake 2.6, the `CMAKE_Fortran_COMPILER_ID` variable should be used instead:

~~~
if(CMAKE_Fortran_COMPILER_ID STREQUAL "GNU")
  # GNU Fortran compiler-specific logic.
endif()
~~~

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
