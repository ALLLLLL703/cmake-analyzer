CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_AUTOMOC_RELAXED_MODE.html)

# CMAKE_AUTOMOC_RELAXED_MODE

Deprecated since version 3.15.

Switch between strict and relaxed automoc mode.

By default, [`AUTOMOC`](https://cmake.org/cmake/help/v4.4/prop_tgt/AUTOMOC.html#prop_tgt:AUTOMOC) behaves exactly as described in the documentation of the [`AUTOMOC`](https://cmake.org/cmake/help/v4.4/prop_tgt/AUTOMOC.html#prop_tgt:AUTOMOC) target property. When set to `TRUE`, it accepts more input and tries to find the correct input file for `moc` even if it differs from the documented behavior. In this mode it e.g. also checks whether a header file is intended to be processed by moc when a `"foo.moc"` file has been included.

Relaxed mode has to be enabled for KDE4 compatibility.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
