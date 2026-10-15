CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_GLOBAL_AUTORCC_TARGET.html)

# CMAKE_GLOBAL_AUTORCC_TARGET

Added in version 3.14.

Switch to enable generation of a global `autorcc` target.

When `CMAKE_GLOBAL_AUTORCC_TARGET` is enabled, a custom target `autorcc` is generated. This target depends on all [`AUTORCC`](https://cmake.org/cmake/help/v4.4/prop_tgt/AUTORCC.html#prop_tgt:AUTORCC) generated `<ORIGIN>_arcc_<QRC>` targets in the project. By building the global `autorcc` target, all [`AUTORCC`](https://cmake.org/cmake/help/v4.4/prop_tgt/AUTORCC.html#prop_tgt:AUTORCC) files in the project will be generated.

The name of the global `autorcc` target can be changed by setting [`CMAKE_GLOBAL_AUTORCC_TARGET_NAME`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_GLOBAL_AUTORCC_TARGET_NAME.html#variable:CMAKE_GLOBAL_AUTORCC_TARGET_NAME).

By default `CMAKE_GLOBAL_AUTORCC_TARGET` is unset.

See the [`cmake-qt(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-qt.7.html#manual:cmake-qt(7)) manual for more information on using CMake with Qt.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
