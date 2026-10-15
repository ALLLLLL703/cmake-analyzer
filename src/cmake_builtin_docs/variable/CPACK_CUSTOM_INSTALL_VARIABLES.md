CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CPACK_CUSTOM_INSTALL_VARIABLES.html)

# CPACK_CUSTOM_INSTALL_VARIABLES

Added in version 3.21.

CPack variables (set via e.g. [`cpack -D`](https://cmake.org/cmake/help/v4.4/manual/cpack.1.html#cmdoption-cpack-D), `CPackConfig.cmake` or [`CPACK_PROJECT_CONFIG_FILE`](https://cmake.org/cmake/help/v4.4/module/CPack.html#variable:CPACK_PROJECT_CONFIG_FILE) scripts) are not directly visible in installation scripts. Instead, one can pass a list of `varName=value` pairs in the `CPACK_CUSTOM_INSTALL_VARIABLES` variable. At install time, each list item will result in a variable of the specified name (`varName`) being set to the given `value`. The `=` can be omitted for an empty `value`.

`CPACK_CUSTOM_INSTALL_VARIABLES` allows the packaging installation to be influenced by the user or driving script at CPack runtime without having to regenerate the install scripts.

## Example

~~~
install(FILES large.txt DESTINATION data)

install(CODE [[
  if(ENABLE_COMPRESSION)
    # "run-compressor" is a fictional tool that produces
    # large.txt.xz from large.txt and then removes the input file
    execute_process(COMMAND run-compressor $ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/large.txt)
  endif()
]])
~~~

With the above example snippet, [`cpack`](https://cmake.org/cmake/help/v4.4/manual/cpack.1.html#manual:cpack(1)) will by default run the installation script with `ENABLE_COMPRESSION` unset, resulting in a package containing the uncompressed `large.txt`. This can be overridden when invoking [`cpack`](https://cmake.org/cmake/help/v4.4/manual/cpack.1.html#manual:cpack(1)) like so:

~~~
cpack -D "CPACK_CUSTOM_INSTALL_VARIABLES=ENABLE_COMPRESSION=TRUE"
~~~

The installation script will then run with `ENABLE_COMPRESSION` set to `TRUE`, resulting in a package containing the compressed `large.txt.xz` instead.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
