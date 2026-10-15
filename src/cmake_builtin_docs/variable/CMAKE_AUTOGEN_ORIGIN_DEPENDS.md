CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_AUTOGEN_ORIGIN_DEPENDS.html)

# CMAKE_AUTOGEN_ORIGIN_DEPENDS

Added in version 3.14.

Switch for forwarding origin target dependencies to the corresponding [\<ORIGIN\>_autogen](https://cmake.org/cmake/help/v4.4/manual/cmake-qt.7.html#origin-autogen) targets.

Note

If the [\<ORIGIN\>_autogen_timestamp_deps](https://cmake.org/cmake/help/v4.4/manual/cmake-qt.7.html#origin-autogen-timestamp-deps) target is created, additional target dependencies are added to it instead of the [\<ORIGIN\>_autogen](https://cmake.org/cmake/help/v4.4/manual/cmake-qt.7.html#origin-autogen) target. When using the [Visual Studio Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#visual-studio-generators), the `moc` and `uic` step may be part of the `<ORIGIN>` target itself, in which case the dependencies are added to `<ORIGIN>`. See the [`cmake-qt(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-qt.7.html#manual:cmake-qt(7)) manual for details.

This variable is used to initialize the [`AUTOGEN_ORIGIN_DEPENDS`](https://cmake.org/cmake/help/v4.4/prop_tgt/AUTOGEN_ORIGIN_DEPENDS.html#prop_tgt:AUTOGEN_ORIGIN_DEPENDS) property on all the targets. See that target property for additional information.

By default `CMAKE_AUTOGEN_ORIGIN_DEPENDS` is `ON`.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
