CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/cmake_instrumentation.html)

# cmake_instrumentation

Added in version 4.3.

Enables interacting with the [`CMake Instrumentation API`](https://cmake.org/cmake/help/v4.4/manual/cmake-instrumentation.7.html#manual:cmake-instrumentation(7)).

This allows for configuring instrumentation at the project-level.

~~~
cmake_instrumentation(
  API_VERSION <version>
  DATA_VERSION <version>
  [HOOKS <hooks>...]
  [OPTIONS <options>...]
  [CALLBACK <callback>]
  [CUSTOM_CONTENT <name> <type> <content>]
)
~~~

The `API_VERSION` and `DATA_VERSION` must always be given.

`API_VERSION` is an integer. Currently, the only supported value is `1`. See [API v1](https://cmake.org/cmake/help/v4.4/manual/cmake-instrumentation.7.html#cmake-instrumentation-api-v1) for details.

`DATA_VERSION` is a version value of the form `major` or `major.minor`. Currently, the maximum supported version is `1.1`. See [Data Version](https://cmake.org/cmake/help/v4.4/manual/cmake-instrumentation.7.html#cmake-instrumentation-data-version) for details.

Each of the optional keywords `HOOKS`, `OPTIONS`, and `CALLBACK` correspond to one of the parameters to the [v1 Query Files](https://cmake.org/cmake/help/v4.4/manual/cmake-instrumentation.7.html#cmake-instrumentation-v1-query-files). The `CALLBACK` keyword can be provided multiple times to create multiple callbacks.

Whenever `cmake_instrumentation` is invoked, a query file is generated in `<build>/.cmake/instrumentation/v1/query/generated` to enable instrumentation with the provided arguments.

## Custom CMake Content

The `CUSTOM_CONTENT` argument specifies certain data from configure time to include in each [v1 CMake Content File](https://cmake.org/cmake/help/v4.4/manual/cmake-instrumentation.7.html#cmake-instrumentation-v1-cmake-content-file). This may be used to associate instrumentation data with certain information about its configuration, such as the optimization level or whether it is part of a coverage build.

`CUSTOM_CONTENT` expects `name`, `type` and `content` arguments.

`name` is a specifier to identify the content being reported.

**`type` specifies how the content should be interpreted. Supported values are:**

- `STRING` the content is a string.

- `BOOL` the content should be interpreted as a boolean. It will be `true` under the same conditions that `if()` would be true for the given value.

- `LIST` the content is a CMake `;` separated list that should be parsed.

- `JSON` the content should be parsed as a JSON string. This can be a number such as `1` or `5.0`, a quoted string such as `\"string\"`, a boolean value `true`/`false`, or a JSON object such as `{ \"key\" : \"value\" }` that may be constructed using `string(JSON ...)` commands.

`content` is the actual content to report.

## Example

The following example shows an invocation of the command and its equivalent JSON query file.

~~~
cmake_instrumentation(
  API_VERSION 1
  DATA_VERSION 1.0
  HOOKS postGenerate preCMakeBuild postCMakeBuild
  OPTIONS staticSystemInformation dynamicSystemInformation compileTrace trace
  CALLBACK ${CMAKE_COMMAND} -P /path/to/handle_data.cmake
  CALLBACK ${CMAKE_COMMAND} -P /path/to/handle_data_2.cmake
  CUSTOM_CONTENT myString STRING string
  CUSTOM_CONTENT myList   LIST   "item1;item2"
  CUSTOM_CONTENT myObject JSON   "{ \"key\" : \"value\" }"
)
~~~

~~~
{
  "version": 1,
  "hooks": [
    "postGenerate", "preCMakeBuild", "postCMakeBuild"
  ],
  "options": [
    "staticSystemInformation", "dynamicSystemInformation", "compileTrace", "trace"
  ],
  "callbacks": [
    "/path/to/cmake -P /path/to/handle_data.cmake",
    "/path/to/cmake -P /path/to/handle_data_2.cmake"
  ]
}
~~~

This will also result in the following content included in each [v1 CMake Content File](https://cmake.org/cmake/help/v4.4/manual/cmake-instrumentation.7.html#cmake-instrumentation-v1-cmake-content-file):

~~~
"custom": {
  "myString": "string",
  "myList": [
    "item1", "item2"
  ],
  "myObject": {
    "key": "value"
  }
}
~~~

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
