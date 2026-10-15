CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/qt_wrap_ui.html)

# qt_wrap_ui

Deprecated since version 3.14: This command was originally added to support Qt 3 before the [`add_custom_command()`](https://cmake.org/cmake/help/v4.4/command/add_custom_command.html#command:add_custom_command) command was sufficiently mature. The [`FindQt4`](https://cmake.org/cmake/help/v4.4/module/FindQt4.html#module:FindQt4) module provides the `qt4_wrap_ui()` macro, which should be used instead for Qt 4 projects. For projects using Qt 5 or later, use the equivalent macro provided by Qt itself (e.g. Qt 5 provides `qt5_wrap_ui()`).

Manually create Qt user interfaces Wrappers.

~~~
qt_wrap_ui(resultingLibraryName HeadersDestName
           SourcesDestName SourceLists ...)
~~~

Produces .h and .cxx files for all the .ui files listed in the `SourceLists`. The .h files will be added to the library using the `HeadersDestNamesource` list. The .cxx files will be added to the library using the `SourcesDestNamesource` list.

Consider updating the project to use the [`AUTOUIC`](https://cmake.org/cmake/help/v4.4/prop_tgt/AUTOUIC.html#prop_tgt:AUTOUIC) target property instead for a more automated way of invoking the `uic` tool.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
