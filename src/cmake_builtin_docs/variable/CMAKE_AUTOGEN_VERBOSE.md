CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_AUTOGEN_VERBOSE.html)

# CMAKE_AUTOGEN_VERBOSE

Added in version 3.13.

Sets the verbosity of [`AUTOMOC`](https://cmake.org/cmake/help/v4.4/prop_tgt/AUTOMOC.html#prop_tgt:AUTOMOC), [`AUTOUIC`](https://cmake.org/cmake/help/v4.4/prop_tgt/AUTOUIC.html#prop_tgt:AUTOUIC) and [`AUTORCC`](https://cmake.org/cmake/help/v4.4/prop_tgt/AUTORCC.html#prop_tgt:AUTORCC). A positive integer value or a true boolean value lets the `AUTO*` generators output additional processing information.

Setting `CMAKE_AUTOGEN_VERBOSE` has the same effect as setting the `VERBOSE` environment variable during generation (e.g. by calling `make VERBOSE=1`). The extra verbosity is limited to the `AUTO*` generators though.

By default `CMAKE_AUTOGEN_VERBOSE` is unset.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
