CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_GLOBAL_AUTOGEN_TARGET_NAME.html)

# CMAKE_GLOBAL_AUTOGEN_TARGET_NAME

Added in version 3.14.

Change the name of the global `autogen` target.

When [`CMAKE_GLOBAL_AUTOGEN_TARGET`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_GLOBAL_AUTOGEN_TARGET.html#variable:CMAKE_GLOBAL_AUTOGEN_TARGET) is enabled, a global custom target named `autogen` is created. `CMAKE_GLOBAL_AUTOGEN_TARGET_NAME` allows to set a different name for that target.

By default `CMAKE_GLOBAL_AUTOGEN_TARGET_NAME` is unset.

See the [`cmake-qt(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-qt.7.html#manual:cmake-qt(7)) manual for more information on using CMake with Qt.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
