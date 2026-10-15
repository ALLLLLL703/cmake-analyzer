CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FASTBUILD_TRACK_BYPRODUCTS_AS_OUTPUT.html)

# CMAKE_FASTBUILD_TRACK_BYPRODUCTS_AS_OUTPUT

Added in version 4.2.

By default, custom commands declaring only `BYPRODUCTS` will always run unconditionally. You can use this variable to make FASTBuild rerun the command only when its inputs have changed or the byproduct file is missing.

Note

When this variable is `OFF` (the default), `BYPRODUCTS` are treated similarly to how Ninja handles them - as opaque side effects - and the generator emits `ExecAlways` nodes to ensure they always run.

Defaults to `OFF`.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
