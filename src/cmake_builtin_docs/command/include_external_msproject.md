CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/include_external_msproject.html)

# include_external_msproject

Include an external Microsoft project file in the solution file produced by [Visual Studio Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#visual-studio-generators). Ignored on other generators.

~~~
include_external_msproject(projectname location
                           [TYPE projectTypeGUID]
                           [GUID projectGUID]
                           [PLATFORM platformName]
                           dep1 dep2 ...)
~~~

Includes an external Microsoft project in the generated solution file. This will create a target named `[projectname]`. This can be used in the [`add_dependencies()`](https://cmake.org/cmake/help/v4.4/command/add_dependencies.html#command:add_dependencies) command to make things depend on the external project.

`TYPE`, `GUID` and `PLATFORM` are optional parameters that allow one to specify the type of project, id (`GUID`) of the project and the name of the target platform. This is useful for projects requiring values other than the default (e.g. WIX projects).

Added in version 3.9: If the imported project has different configuration names than the current project, set the [`MAP_IMPORTED_CONFIG_<CONFIG>`](https://cmake.org/cmake/help/v4.4/prop_tgt/MAP_IMPORTED_CONFIG_CONFIG.html#prop_tgt:MAP_IMPORTED_CONFIG_%3CCONFIG%3E) target property to specify the mapping.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
