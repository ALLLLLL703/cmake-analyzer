CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_COMPILER_IS_GNUCXX.html)

# CMAKE_COMPILER_IS_GNUCXX

Deprecated since version 3.24: Use the [`CMAKE_CXX_COMPILER_ID`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_COMPILER_ID.html#variable:CMAKE_%3CLANG%3E_COMPILER_ID) variable instead.

The `CMAKE_COMPILER_IS_*` variables were used in early CMake versions before the introduction of [`CMAKE_<LANG>_COMPILER_ID`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_COMPILER_ID.html#variable:CMAKE_%3CLANG%3E_COMPILER_ID) variables in CMake 2.6.

The `CMAKE_COMPILER_IS_GNUCXX` variable is set to boolean true if the C++ (`CXX`) compiler is GNU.

## Examples

In earlier versions of CMake, the `CMAKE_COMPILER_IS_GNUCXX` variable was used to check if the `CXX` compiler was GNU:

~~~
if(CMAKE_COMPILER_IS_GNUCXX)
  # GNU C++ compiler-specific logic.
endif()
~~~

Starting with CMake 2.6, the `CMAKE_CXX_COMPILER_ID` variable should be used instead:

~~~
if(CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
  # GNU C++ compiler-specific logic.
endif()
~~~

When upgrading code, consider whether additional `CXX` compilers should be targeted beyond just `GNU`. In the past, the `LCC` and `QCC` compilers also had this variable set (see [`CMP0047`](https://cmake.org/cmake/help/v4.4/policy/CMP0047.html#policy:CMP0047) and [`CMP0129`](https://cmake.org/cmake/help/v4.4/policy/CMP0129.html#policy:CMP0129) policies for details). To account for this, the following approach can be used:

~~~
# Targeting GNU, LCC, and QCC compilers for CXX:
if(CMAKE_CXX_COMPILER_ID MATCHES "^(GNU|LCC|QCC)$")
  # ...
endif()
~~~

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
