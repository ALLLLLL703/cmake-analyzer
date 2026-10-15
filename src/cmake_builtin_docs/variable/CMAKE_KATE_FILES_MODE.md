CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_KATE_FILES_MODE.html)

# CMAKE_KATE_FILES_MODE

Added in version 3.27.

This cache variable is used by the Kate project generator and controls to what mode the `files` entry in the project file will be set. See [`cmake-generators(7)`](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#manual:cmake-generators(7)).

Possible values are `AUTO`, `SVN`, `GIT`, `HG`, `FOSSIL` and `LIST`.

When set to `LIST`, CMake will put the list of source files known to CMake in the project file. When set to `SVN`, `GIT`, `HG` or `FOSSIL`, CMake will set the generated project accordingly to Subversion, git, Mercurial or Fossil, and Kate will then use the respective command line tool to retrieve the list of files in the project. When unset or set to `AUTO`, CMake will try to detect whether the source directory is part of a git or svn checkout or not, and put the respective entry into the project file.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
