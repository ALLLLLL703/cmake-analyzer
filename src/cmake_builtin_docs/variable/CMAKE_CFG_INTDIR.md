CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CFG_INTDIR.html)

# CMAKE_CFG_INTDIR

Deprecated since version 3.21: This variable has poor support on [`Ninja Multi-Config`](https://cmake.org/cmake/help/v4.4/generator/Ninja%20Multi-Config.html#generator:Ninja%20Multi-Config), and predates the existence of the [`$<CONFIG>`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#genex:CONFIG) generator expression. Use `$<CONFIG>` instead.

Build-time reference to per-configuration output subdirectory.

For native build systems supporting multiple configurations in the build tree (such as [Visual Studio Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#visual-studio-generators) and [`Xcode`](https://cmake.org/cmake/help/v4.4/generator/Xcode.html#generator:Xcode)), the value is a reference to a build-time variable specifying the name of the per-configuration output subdirectory. On [Makefile Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#makefile-generators) this evaluates to `.` because there is only one configuration in a build tree. Example values:

`$(Configuration)` |  Visual Studio |

`$(CONFIGURATION)` |  Xcode |

`.` |  Make-based tools |

`.` |  Ninja |

`${CONFIGURATION}` |  Ninja Multi-Config |

Since these values are evaluated by the native build system, this variable is suitable only for use in command lines that will be evaluated at build time. Example of intended usage:

~~~
add_executable(mytool mytool.c)
add_custom_command(
  OUTPUT out.txt
  COMMAND ${CMAKE_CURRENT_BINARY_DIR}/${CMAKE_CFG_INTDIR}/mytool
          ${CMAKE_CURRENT_SOURCE_DIR}/in.txt out.txt
  DEPENDS mytool in.txt
  )
add_custom_target(drive ALL DEPENDS out.txt)
~~~

Note that `CMAKE_CFG_INTDIR` is no longer necessary for this purpose but has been left for compatibility with existing projects. Instead [`add_custom_command()`](https://cmake.org/cmake/help/v4.4/command/add_custom_command.html#command:add_custom_command) recognizes executable target names in its `COMMAND` option, so `${CMAKE_CURRENT_BINARY_DIR}/${CMAKE_CFG_INTDIR}/mytool` can be replaced by just `mytool`.

This variable is read-only. Setting it is undefined behavior. In multi-configuration build systems the value of this variable is passed as the value of preprocessor symbol `CMAKE_INTDIR` to the compilation of all source files.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
