CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_GLOBAL_AUTOGEN_TARGET.html)

# CMAKE_GLOBAL_AUTOGEN_TARGET

Added in version 3.14.

Switch to enable generation of a global `autogen` target.

When `CMAKE_GLOBAL_AUTOGEN_TARGET` is enabled, a custom target `autogen` is generated. This target depends on all [`AUTOMOC`](https://cmake.org/cmake/help/v4.4/prop_tgt/AUTOMOC.html#prop_tgt:AUTOMOC) and [`AUTOUIC`](https://cmake.org/cmake/help/v4.4/prop_tgt/AUTOUIC.html#prop_tgt:AUTOUIC) generated [\<ORIGIN\>_autogen](https://cmake.org/cmake/help/v4.4/manual/cmake-qt.7.html#origin-autogen) targets in the project. By building the global `autogen` target, all [`AUTOMOC`](https://cmake.org/cmake/help/v4.4/prop_tgt/AUTOMOC.html#prop_tgt:AUTOMOC) and [`AUTOUIC`](https://cmake.org/cmake/help/v4.4/prop_tgt/AUTOUIC.html#prop_tgt:AUTOUIC) files in the project will be generated.

The name of the global `autogen` target can be changed by setting [`CMAKE_GLOBAL_AUTOGEN_TARGET_NAME`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_GLOBAL_AUTOGEN_TARGET_NAME.html#variable:CMAKE_GLOBAL_AUTOGEN_TARGET_NAME).

By default `CMAKE_GLOBAL_AUTOGEN_TARGET` is unset.

See the [`cmake-qt(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-qt.7.html#manual:cmake-qt(7)) manual for more information on using CMake with Qt.

Note

[\<ORIGIN\>_autogen](https://cmake.org/cmake/help/v4.4/manual/cmake-qt.7.html#origin-autogen) targets by default inherit their origin target's dependencies. This might result in unintended dependency target builds when only [\<ORIGIN\>_autogen](https://cmake.org/cmake/help/v4.4/manual/cmake-qt.7.html#origin-autogen) targets are built. A solution is to disable [`AUTOGEN_ORIGIN_DEPENDS`](https://cmake.org/cmake/help/v4.4/prop_tgt/AUTOGEN_ORIGIN_DEPENDS.html#prop_tgt:AUTOGEN_ORIGIN_DEPENDS) on the respective origin targets.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
