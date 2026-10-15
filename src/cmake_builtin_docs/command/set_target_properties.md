CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/set_target_properties.html)

# set_target_properties

Targets can have properties that affect how they are built.

~~~
set_target_properties(<targets> ...
                      PROPERTIES <prop1> <value1>
                      [<prop2> <value2>] ...)
~~~

Sets properties on targets. The syntax for the command is to list all the targets you want to change, and then provide the values you want to set next. You can use any prop value pair you want and extract it later with the [`get_property()`](https://cmake.org/cmake/help/v4.4/command/get_property.html#command:get_property) or [`get_target_property()`](https://cmake.org/cmake/help/v4.4/command/get_target_property.html#command:get_target_property) command.

[Alias Targets](https://cmake.org/cmake/help/v4.4/manual/cmake-buildsystem.7.html#alias-targets) do not support setting target properties.

## See Also

- [`define_property()`](https://cmake.org/cmake/help/v4.4/command/define_property.html#command:define_property)

- [`get_target_property()`](https://cmake.org/cmake/help/v4.4/command/get_target_property.html#command:get_target_property)

- the more general [`set_property()`](https://cmake.org/cmake/help/v4.4/command/set_property.html#command:set_property) command

- [Properties on Targets](https://cmake.org/cmake/help/v4.4/manual/cmake-properties.7.html#target-properties) for the list of properties known to CMake

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
