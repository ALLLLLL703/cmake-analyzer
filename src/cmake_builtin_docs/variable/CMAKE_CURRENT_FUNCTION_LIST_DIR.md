CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_FUNCTION_LIST_DIR.html)

# CMAKE_CURRENT_FUNCTION_LIST_DIR

Added in version 3.17.

When executing code inside a [`function()`](https://cmake.org/cmake/help/v4.4/command/function.html#command:function), this variable contains the full directory of the listfile that defined the current function.

It is quite common practice in CMake for modules to use some additional files, such as templates to be copied in after substituting CMake variables. In such cases, a function needs to know where to locate those files in a way that doesn't depend on where the function is called. Without `CMAKE_CURRENT_FUNCTION_LIST_DIR`, the code to do that would typically use the following pattern:

~~~
set(_THIS_MODULE_BASE_DIR "${CMAKE_CURRENT_LIST_DIR}")

function(foo)
  configure_file(
    "${_THIS_MODULE_BASE_DIR}/some.template.in"
    some.output
  )
endfunction()
~~~

Using `CMAKE_CURRENT_FUNCTION_LIST_DIR` inside the function instead eliminates the need for the extra variable which would otherwise be visible outside the function's scope. The above example can be written in the more concise and more robust form:

~~~
function(foo)
  configure_file(
    "${CMAKE_CURRENT_FUNCTION_LIST_DIR}/some.template.in"
    some.output
  )
endfunction()
~~~

See also [`CMAKE_CURRENT_FUNCTION`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_FUNCTION.html#variable:CMAKE_CURRENT_FUNCTION), [`CMAKE_CURRENT_FUNCTION_LIST_FILE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_FUNCTION_LIST_FILE.html#variable:CMAKE_CURRENT_FUNCTION_LIST_FILE), [`CMAKE_CURRENT_FUNCTION_LIST_LINE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_FUNCTION_LIST_LINE.html#variable:CMAKE_CURRENT_FUNCTION_LIST_LINE) and [`CMAKE_CURRENT_LIST_DIR`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_LIST_DIR.html#variable:CMAKE_CURRENT_LIST_DIR).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
