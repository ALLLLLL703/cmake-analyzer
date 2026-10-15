CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_BYTE_ORDER.html)

# CMAKE_\<LANG\>_BYTE_ORDER

Added in version 3.20.

Byte order of `<LANG>` compiler target architecture, if known. If defined and not empty, the value is one of:

**`BIG_ENDIAN`**

The target architecture is Big Endian.

**`LITTLE_ENDIAN`**

The target architecture is Little Endian.

This is defined for languages `C`, `CXX`, `OBJC`, `OBJCXX`, and `CUDA`.

If [`CMAKE_OSX_ARCHITECTURES`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_OSX_ARCHITECTURES.html#variable:CMAKE_OSX_ARCHITECTURES) specifies multiple architectures, the value of `CMAKE_<LANG>_BYTE_ORDER` is non-empty only if all architectures share the same byte order.

## Examples

### Example: Checking Endianness

Checking endianness (byte order) of the target architecture in a CMake project, where `C` language is one of the enabled languages, and storing the result in a variable `WORDS_BIGENDIAN`:

~~~
if(CMAKE_C_BYTE_ORDER STREQUAL "BIG_ENDIAN")
  set(WORDS_BIGENDIAN TRUE)
elseif(CMAKE_C_BYTE_ORDER STREQUAL "LITTLE_ENDIAN")
  set(WORDS_BIGENDIAN FALSE)
else()
  set(WORDS_BIGENDIAN FALSE)
  message(WARNING "Endianness could not be determined.")
endif()
~~~

Or, if the project doesn't have `C` language enabled, it can be replaced with some other enabled language. For example, if `CXX` is enabled:

~~~
if(CMAKE_CXX_BYTE_ORDER STREQUAL "BIG_ENDIAN")
  set(WORDS_BIGENDIAN TRUE)
elseif(CMAKE_CXX_BYTE_ORDER STREQUAL "LITTLE_ENDIAN")
  set(WORDS_BIGENDIAN FALSE)
else()
  set(WORDS_BIGENDIAN FALSE)
  message(WARNING "Endianness could not be determined.")
endif()
~~~

Note, that in most cases this can be simplified by only checking for a big-endian target:

~~~
if(CMAKE_C_BYTE_ORDER STREQUAL "BIG_ENDIAN")
  set(WORDS_BIGENDIAN TRUE)
else()
  set(WORDS_BIGENDIAN FALSE)
endif()
~~~

### Example: Per-language Endianness Check

Most of the time, architectures used today are consistent in endianness across compilers. But here's when per-language endianness check can matter:

- Cross-compilation to different architectures (e.g., big-endian embedded system).

- Heterogeneous toolchains where one target architecture is for C language and another target is for different language.

- Static libraries or binaries reused across platforms (e.g., distributing precompiled CUDA kernels).

~~~
if(CMAKE_C_BYTE_ORDER)
  message(STATUS "C byte order: ${CMAKE_C_BYTE_ORDER}")
endif()

if(CMAKE_CXX_BYTE_ORDER)
  message(STATUS "C++ byte order: ${CMAKE_CXX_BYTE_ORDER}")
endif()

if(CMAKE_CUDA_BYTE_ORDER)
  message(STATUS "CUDA byte order: ${CMAKE_CUDA_BYTE_ORDER}")
endif()
~~~

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
