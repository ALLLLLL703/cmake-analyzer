CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_INSTALL_DEFAULT_DIRECTORY_PERMISSIONS.html)

# CMAKE_INSTALL_DEFAULT_DIRECTORY_PERMISSIONS

Added in version 3.11.

Default permissions for directories created implicitly during installation of files by [`install()`](https://cmake.org/cmake/help/v4.4/command/install.html#command:install) and [`file(INSTALL)`](https://cmake.org/cmake/help/v4.4/command/file.html#install).

If `make install` is invoked and directories are implicitly created they get permissions set by `CMAKE_INSTALL_DEFAULT_DIRECTORY_PERMISSIONS` variable or platform specific default permissions if the variable is not set.

Implicitly created directories are created if they are not explicitly installed by [`install()`](https://cmake.org/cmake/help/v4.4/command/install.html#command:install) command but are needed to install a file on a certain path. Example of such locations are directories created due to the setting of [`CMAKE_INSTALL_PREFIX`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_INSTALL_PREFIX.html#variable:CMAKE_INSTALL_PREFIX).

Expected content of the `CMAKE_INSTALL_DEFAULT_DIRECTORY_PERMISSIONS` variable is a list of permissions that can be used by [`install()`](https://cmake.org/cmake/help/v4.4/command/install.html#command:install) command `PERMISSIONS` section.

Example usage:

~~~
set(CMAKE_INSTALL_DEFAULT_DIRECTORY_PERMISSIONS
     OWNER_READ
     OWNER_WRITE
     OWNER_EXECUTE
     GROUP_READ
   )
~~~

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
