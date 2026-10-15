CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_EXPORT_SARIF.html)

# CMAKE_EXPORT_SARIF

Added in version 4.0.

Enable or disable CMake diagnostics output in SARIF format for a project.

If enabled, CMake will generate a SARIF log file containing diagnostic messages output by CMake when running in a project. By default, the log file is written to `.cmake/sarif/cmake.sarif`, but the location can be changed by setting the command-line option [`cmake --sarif-output`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-sarif-output) to the desired path.

The Static Analysis Results Interchange Format (SARIF) is a JSON-based standard format for static analysis tools (including build tools like CMake) to record and communicate diagnostic messages. CMake generates a SARIF log entry for warnings and errors produced while running CMake on a project (e.g. [`message()`](https://cmake.org/cmake/help/v4.4/command/message.html#command:message) calls). Each log entry includes the message, severity, and location information if available.

An example of CMake's SARIF output is:

~~~
{
  "version" : "2.1.0",
  "$schema" : "https://schemastore.azurewebsites.net/schemas/json/sarif-2.1.0-rtm.4.json",
  "runs" :
  [
    {
      "tool" :
      {
        "driver" :
        {
          "name" : "CMake",
          "rules" :
          [
            {
              "id" : "CMake.Warning",
              "messageStrings" :
              {
                "default" :
                {
                  "text" : "CMake Warning: {0}"
                }
              },
              "name" : "CMake Warning"
            }
          ]
        }
      },
      "results" :
      [
        {
          "level" : "warning",
          "locations" :
          [
            {
              "physicalLocation" :
              {
                "artifactLocation" :
                {
                  "uri" : "/home/user/development/project/CMakeLists.txt"
                },
                "region" :
                {
                  "startLine" : 5
                }
              }
            }
          ],
          "message" :
          {
            "text" : "An example warning"
          },
          "ruleId" : "CMake.Warning",
          "ruleIndex" : 0
        }
      ]
    }
  ]
}
~~~

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
