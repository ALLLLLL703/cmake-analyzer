CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/get_target_property.html)

# get_target_property

Get a property from a target.

~~~
get_target_property(<variable> <target> <property>)
~~~

Get a property from a target. The value of the property is stored in the specified `<variable>`. If the target property is not found, `<variable>` will be set to `<variable>-NOTFOUND`. If the target property was defined to be an `INHERITED` property (see [`define_property()`](https://cmake.org/cmake/help/v4.4/command/define_property.html#command:define_property)), the search will include the relevant parent scopes, as described for the [`define_property()`](https://cmake.org/cmake/help/v4.4/command/define_property.html#command:define_property) command.

Use [`set_target_properties()`](https://cmake.org/cmake/help/v4.4/command/set_target_properties.html#command:set_target_properties) to set target property values. Properties are usually used to control how a target is built, but some query the target instead. This command can get properties for any target so far created. The targets do not need to be in the current `CMakeLists.txt` file.

## See Also

- [`define_property()`](https://cmake.org/cmake/help/v4.4/command/define_property.html#command:define_property)

- the more general [`get_property()`](https://cmake.org/cmake/help/v4.4/command/get_property.html#command:get_property) command

- [`set_target_properties()`](https://cmake.org/cmake/help/v4.4/command/set_target_properties.html#command:set_target_properties)

- [Properties on Targets](https://cmake.org/cmake/help/v4.4/manual/cmake-properties.7.html#target-properties) for the list of properties known to CMake

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
