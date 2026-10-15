CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/qt_wrap_cpp.html)

# qt_wrap_cpp

Deprecated since version 3.14: This command was originally added to support Qt 3 before the [`add_custom_command()`](https://cmake.org/cmake/help/v4.4/command/add_custom_command.html#command:add_custom_command) command was sufficiently mature. The [`FindQt4`](https://cmake.org/cmake/help/v4.4/module/FindQt4.html#module:FindQt4) module provides the `qt4_wrap_cpp()` macro, which should be used instead for Qt 4 projects. For projects using Qt 5 or later, use the equivalent macro provided by Qt itself (e.g. Qt 5 provides [qt5_wrap_cpp()](https://doc.qt.io/archives/qt-5.15/qtcore-cmake-qt5-wrap-cpp.html)).

Manually create Qt Wrappers.

~~~
qt_wrap_cpp(resultingLibraryName DestName SourceLists ...)
~~~

Produces moc files for all the .h files listed in the SourceLists. The moc files will be added to the library using the `DestName` source list.

Consider updating the project to use the [`AUTOMOC`](https://cmake.org/cmake/help/v4.4/prop_tgt/AUTOMOC.html#prop_tgt:AUTOMOC) target property instead for a more automated way of invoking the `moc` tool.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
