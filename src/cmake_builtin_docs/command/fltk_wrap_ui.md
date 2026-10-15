CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/fltk_wrap_ui.html)

# fltk_wrap_ui

Create FLTK user interfaces Wrappers.

~~~
fltk_wrap_ui(resultingLibraryName source1
             source2 ... sourceN)
~~~

Produce .h and .cxx files for all the .fl and .fld files listed. The resulting .h and .cxx files will be added to a variable named `resultingLibraryName_FLTK_UI_SRCS` which should be added to your library.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
