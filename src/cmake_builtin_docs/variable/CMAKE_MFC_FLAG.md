CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MFC_FLAG.html)

# CMAKE_MFC_FLAG

Use the MFC library for an executable or dll.

Enables the use of the Microsoft Foundation Classes (MFC). It should be set to `1` for the static MFC library, and `2` for the shared MFC library. This is used in Visual Studio project files.

Contents of `CMAKE_MFC_FLAG` may use [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)).

## Examples

Usage example:

~~~
set(CMAKE_MFC_FLAG 2)

add_executable(CMakeSetup WIN32 ${SRCS})

# Visual Studio generators add this flag automatically based on the
# CMAKE_MFC_FLAG value, but generators matching "Make" require it:
target_compile_definitions(CMakeSetup PRIVATE _AFXDLL)
~~~

## See Also

- The [`FindMFC`](https://cmake.org/cmake/help/v4.4/module/FindMFC.html#module:FindMFC) module to check whether MFC is installed and available.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
