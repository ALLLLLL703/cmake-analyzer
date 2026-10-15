CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_PACKAGE_SORT_ORDER.html)

# CMAKE_FIND_PACKAGE_SORT_ORDER

Added in version 3.7.

Changed in version 4.2: The default sort order has changed from `NONE` to `NATURAL`.

The default order for sorting directories which match a search path containing a glob expression found using [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package). It can assume one of the following values:

**`NONE`**

No attempt is done to sort directories. The first valid package found will be selected.

**`NAME`**

Sort directories lexicographically before searching.

**`NATURAL`**

Default. Sort directories using natural order (see `strverscmp(3)` manual), i.e. such that contiguous digits are compared as whole numbers.

Natural sorting is employed by default to return the highest version when multiple versions of the same library are available to be found by [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package). For example suppose that the following libraries have package configuration files on disk, in a directory of the same name, with all such directories residing in the same parent directory:

- `libX-1.1.0`

- `libX-1.2.9`

- `libX-1.2.10`

The default order of `NATURAL` will select the one with the highest version number, i.e. `libX-1.2.10`.

The sort direction can be controlled using the [`CMAKE_FIND_PACKAGE_SORT_DIRECTION`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_PACKAGE_SORT_DIRECTION.html#variable:CMAKE_FIND_PACKAGE_SORT_DIRECTION) variable (by default descending, i.e. `libX-1.2` will be tested before `libX-1.0` and `lib-B` will be tested before `lib-A`).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
