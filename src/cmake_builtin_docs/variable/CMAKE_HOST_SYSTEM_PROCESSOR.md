CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_HOST_SYSTEM_PROCESSOR.html)

# CMAKE_HOST_SYSTEM_PROCESSOR

The name of the CPU CMake is running on.

## Windows Platforms

On Windows, this variable is set to the value of the environment variable `PROCESSOR_ARCHITECTURE`.

## Unix Platforms

On systems that support `uname`, this variable is set to the output of:

- `uname -m` on GNU, Linux, Cygwin, Android, or

- `arch` on OpenBSD, or

- on other systems,

   

  - `uname -p` if its exit code is nonzero, or

  - `uname -m` otherwise.

## macOS Platforms

The value of `uname -m` is used by default.

On Apple Silicon hosts, the architecture printed by `uname -m` may vary based on CMake's own architecture and that of the invoking process tree.

Added in version 3.19.2: On Apple Silicon hosts:

- The [`CMAKE_APPLE_SILICON_PROCESSOR`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_APPLE_SILICON_PROCESSOR.html#variable:CMAKE_APPLE_SILICON_PROCESSOR) variable or the [`CMAKE_APPLE_SILICON_PROCESSOR`](https://cmake.org/cmake/help/v4.4/envvar/CMAKE_APPLE_SILICON_PROCESSOR.html#envvar:CMAKE_APPLE_SILICON_PROCESSOR) environment variable may be set to specify the host architecture explicitly.

- If [`CMAKE_OSX_ARCHITECTURES`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_OSX_ARCHITECTURES.html#variable:CMAKE_OSX_ARCHITECTURES) is not set, CMake adds explicit flags to tell the compiler to build for the host architecture so the toolchain does not have to guess based on the process tree's architecture.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
