CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/find_program.html)

# find_program

A short-hand signature is:

~~~
find_program (<VAR> <name> [<path>...])
~~~

The general signature is:

~~~
find_program (
          <VAR>
          {<name> | NAMES <name>... [NAMES_PER_DIR]}
          [HINTS {<path> | ENV <var>}...]
          [PATHS {<path> | ENV <var>}...]
          [REGISTRY_VIEW {64|32|64_32|32_64|HOST|TARGET|BOTH}]
          [PATH_SUFFIXES <suffix>...]
          [VALIDATOR <function>]
          [DOC "cache documentation string"]
          [NO_CACHE]
          [REQUIRED|OPTIONAL]
          [NO_DEFAULT_PATH]
          [NO_PACKAGE_ROOT_PATH]
          [NO_CMAKE_PATH]
          [NO_CMAKE_ENVIRONMENT_PATH]
          [NO_SYSTEM_ENVIRONMENT_PATH]
          [NO_CMAKE_SYSTEM_PATH]
          [NO_CMAKE_INSTALL_PREFIX]
          [CMAKE_FIND_ROOT_PATH_BOTH |
           ONLY_CMAKE_FIND_ROOT_PATH |
           NO_CMAKE_FIND_ROOT_PATH]
         )
~~~

This command is used to find a program.

Prior to searching, find_program checks if variable `<VAR>` is defined. If the variable is not defined, the search will be performed. If the variable is defined and its value is `NOTFOUND`, or ends in `-NOTFOUND`, the search will be performed. If the variable contains any other value the search is not performed.

Note

`VAR` is considered defined if it is available in the current scope. See the [cmake-language(7) variables](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-variables) documentation for details on scopes, and the interaction of normal variables and cache entries.

The results of the search will be stored in a cache entry named `<VAR>`. Future calls to find_program will inspect this cache entry when specifying the same `<VAR>`. This optimization ensures successful searches will not be repeated unless the cache entry is [`unset()`](https://cmake.org/cmake/help/v4.4/command/unset.html#command:unset).

If the program is found the recorded value in cache entry `<VAR>` will be the result of the search. If nothing is found, the recorded value will be `<VAR>-NOTFOUND`.

Options include:

**`NAMES`**

Specify one or more possible names for the program.

When using this to specify names with and without a version suffix, we recommend specifying the unversioned name first so that locally-built packages can be found before those provided by distributions.

**`HINTS`, `PATHS`**

Specify directories to search in addition to the default locations. The `ENV var` sub-option reads paths from a system environment variable.

Changed in version 3.24: On `Windows` platform, it is possible to include registry queries as part of the directories, using a [dedicated syntax](https://cmake.org/cmake/help/v4.4/manual/cmake-developer.7.html#find-using-windows-registry). Such specifications will be ignored on all other platforms.

**`REGISTRY_VIEW`**

Added in version 3.24.

Specify which registry views must be queried. This option is only meaningful on `Windows` platforms and will be ignored on other ones. When not specified, the `BOTH` view is used when the [`CMP0134`](https://cmake.org/cmake/help/v4.4/policy/CMP0134.html#policy:CMP0134) policy is `NEW`. Refer to [`CMP0134`](https://cmake.org/cmake/help/v4.4/policy/CMP0134.html#policy:CMP0134) for the default view when the policy is `OLD`.

**`64`**

Query the 64-bit registry. On 32-bit Windows, it always returns the string `/REGISTRY-NOTFOUND`.

**`32`**

Query the 32-bit registry.

**`64_32`**

Query both views (`64` and `32`) and generate a path for each.

**`32_64`**

Query both views (`32` and `64`) and generate a path for each.

**`HOST`**

Query the registry matching the architecture of the host: `64` on 64-bit Windows and `32` on 32-bit Windows.

**`TARGET`**

Query the registry matching the architecture specified by the [`CMAKE_SIZEOF_VOID_P`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SIZEOF_VOID_P.html#variable:CMAKE_SIZEOF_VOID_P) variable. If not defined, fall back to `HOST` view.

**`BOTH`**

Query both views (`32` and `64`). The order depends on the following rules: If the [`CMAKE_SIZEOF_VOID_P`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SIZEOF_VOID_P.html#variable:CMAKE_SIZEOF_VOID_P) variable is defined, use the following view depending on the content of this variable:

- `8`: `64_32`

- `4`: `32_64`

If the [`CMAKE_SIZEOF_VOID_P`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SIZEOF_VOID_P.html#variable:CMAKE_SIZEOF_VOID_P) variable is not defined, rely on the architecture of the host:

- 64-bit: `64_32`

- 32-bit: `32`

**`PATH_SUFFIXES`**

Specify additional subdirectories to check below each directory location otherwise considered.

**`VALIDATOR`**

Added in version 3.25.

Specify a [`function()`](https://cmake.org/cmake/help/v4.4/command/function.html#command:function) to be called for each candidate item found (a [`macro()`](https://cmake.org/cmake/help/v4.4/command/macro.html#command:macro) cannot be provided, that will result in an error). Two arguments will be passed to the validator function: the name of a result variable, and the absolute path to the candidate item. The item will be accepted and the search will end unless the function sets the value in the result variable to false in the calling scope. The result variable will hold a true value when the validator function is entered.

~~~
function(my_check validator_result_var item)
  if(NOT item MATCHES ...)
    set(${validator_result_var} FALSE PARENT_SCOPE)
  endif()
endfunction()

find_program (result NAMES ... VALIDATOR my_check)
~~~

Note that if a cached result is used, the search is skipped and any `VALIDATOR` is ignored. The cached result is not required to pass the validation function.

**`DOC`**

Specify the documentation string for the `<VAR>` cache entry.

**`NO_CACHE`**

Added in version 3.21.

The result of the search will be stored in a normal variable rather than a cache entry.

Note

find_program will still check for `<VAR>` as usual, checking first for a variable, and then a cache entry. If either indicate a previous successful search, the search will not be performed.

Warning

This option should be used with caution because it can greatly increase the cost of repeated configure steps.

**`REQUIRED`**

Added in version 3.18.

Stop processing with an error message if nothing is found, otherwise the search will be attempted again the next time find_program is invoked with the same variable.

Added in version 4.1: Every find_program command will be treated as `REQUIRED` when the [`CMAKE_FIND_REQUIRED`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_REQUIRED.html#variable:CMAKE_FIND_REQUIRED) variable is enabled.

**`OPTIONAL`**

Added in version 4.1.

Ignore the value of [`CMAKE_FIND_REQUIRED`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_REQUIRED.html#variable:CMAKE_FIND_REQUIRED) and continue without an error message if nothing is found. Incompatible with `REQUIRED`.

If `NO_DEFAULT_PATH` is specified, then no additional paths are added to the search. If `NO_DEFAULT_PATH` is not specified, the search process is as follows:

- If called from within a find module or any other script loaded by a call to [`find_package(<PackageName>)`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package), search prefixes unique to the current package being found. See policy [`CMP0074`](https://cmake.org/cmake/help/v4.4/policy/CMP0074.html#policy:CMP0074).

    

  Added in version 3.12.

    

  Specifically, search paths specified by the following variables, in order:

   

  - [`<PackageName>_ROOT`](https://cmake.org/cmake/help/v4.4/variable/PackageName_ROOT.html#variable:%3CPackageName%3E_ROOT) CMake variable, where `<PackageName>` is the case-preserved package name.

  - [`<PACKAGENAME>_ROOT`](https://cmake.org/cmake/help/v4.4/variable/PackageName_ROOT.html#variable:%3CPACKAGENAME%3E_ROOT) CMake variable, where `<PACKAGENAME>` is the upper-cased package name. See policy [`CMP0144`](https://cmake.org/cmake/help/v4.4/policy/CMP0144.html#policy:CMP0144).

      

    Added in version 3.27.

  - [`<PackageName>_ROOT`](https://cmake.org/cmake/help/v4.4/envvar/PackageName_ROOT.html#envvar:%3CPackageName%3E_ROOT) environment variable, where `<PackageName>` is the case-preserved package name.

  - [`<PACKAGENAME>_ROOT`](https://cmake.org/cmake/help/v4.4/envvar/PackageName_ROOT.html#envvar:%3CPACKAGENAME%3E_ROOT) environment variable, where `<PACKAGENAME>` is the upper-cased package name. See policy [`CMP0144`](https://cmake.org/cmake/help/v4.4/policy/CMP0144.html#policy:CMP0144).

      

    Added in version 3.27.

   

  The package root variables are maintained as a stack, so if called from nested find modules or config packages, root paths from the parent's find module or config package will be searched after paths from the current module or package. In other words, the search order would be `<CurrentPackage>_ROOT`, `ENV{<CurrentPackage>_ROOT}`, `<ParentPackage>_ROOT`, `ENV{<ParentPackage>_ROOT}`, etc. This can be skipped if `NO_PACKAGE_ROOT_PATH` is passed or by setting the [`CMAKE_FIND_USE_PACKAGE_ROOT_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_USE_PACKAGE_ROOT_PATH.html#variable:CMAKE_FIND_USE_PACKAGE_ROOT_PATH) to `FALSE`.

   

  - `<prefix>/[s]bin` for each `<prefix>` in the [`<PackageName>_ROOT`](https://cmake.org/cmake/help/v4.4/variable/PackageName_ROOT.html#variable:%3CPackageName%3E_ROOT) CMake variable and the [`<PackageName>_ROOT`](https://cmake.org/cmake/help/v4.4/envvar/PackageName_ROOT.html#envvar:%3CPackageName%3E_ROOT) environment variable if called from within a find module loaded by [`find_package(<PackageName>)`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package)

- Search paths specified in cmake-specific cache variables. These are intended to be used on the command line with a `-DVAR=value`. The values are interpreted as [semicolon-separated lists](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-lists). This can be skipped if `NO_CMAKE_PATH` is passed or by setting the [`CMAKE_FIND_USE_CMAKE_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_USE_CMAKE_PATH.html#variable:CMAKE_FIND_USE_CMAKE_PATH) to `FALSE`.

   

  - `<prefix>/[s]bin` for each `<prefix>` in [`CMAKE_PREFIX_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PREFIX_PATH.html#variable:CMAKE_PREFIX_PATH)

  - [`CMAKE_PROGRAM_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PROGRAM_PATH.html#variable:CMAKE_PROGRAM_PATH)

  - [`CMAKE_APPBUNDLE_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_APPBUNDLE_PATH.html#variable:CMAKE_APPBUNDLE_PATH)

- Search paths specified in cmake-specific environment variables. These are intended to be set in the user's shell configuration, and therefore use the host's native path separator (`;` on Windows and `:` on UNIX). This can be skipped if `NO_CMAKE_ENVIRONMENT_PATH` is passed or by setting the [`CMAKE_FIND_USE_CMAKE_ENVIRONMENT_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_USE_CMAKE_ENVIRONMENT_PATH.html#variable:CMAKE_FIND_USE_CMAKE_ENVIRONMENT_PATH) to `FALSE`.

   

  - `<prefix>/[s]bin` for each `<prefix>` in [`CMAKE_PREFIX_PATH`](https://cmake.org/cmake/help/v4.4/envvar/CMAKE_PREFIX_PATH.html#envvar:CMAKE_PREFIX_PATH)

  - [`CMAKE_PROGRAM_PATH`](https://cmake.org/cmake/help/v4.4/envvar/CMAKE_PROGRAM_PATH.html#envvar:CMAKE_PROGRAM_PATH)

  - [`CMAKE_APPBUNDLE_PATH`](https://cmake.org/cmake/help/v4.4/envvar/CMAKE_APPBUNDLE_PATH.html#envvar:CMAKE_APPBUNDLE_PATH)

- Search the paths specified by the `HINTS` option. These should be paths computed by system introspection, such as a hint provided by the location of another item already found. Hard-coded guesses should be specified with the `PATHS` option.

- Search the standard system environment variables. This can be skipped if `NO_SYSTEM_ENVIRONMENT_PATH` is passed or by setting the [`CMAKE_FIND_USE_SYSTEM_ENVIRONMENT_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_USE_SYSTEM_ENVIRONMENT_PATH.html#variable:CMAKE_FIND_USE_SYSTEM_ENVIRONMENT_PATH) to `FALSE`.

   

  - The directories in `PATH` itself.

- Search cmake variables defined in the Platform files for the current system. The searching of `CMAKE_INSTALL_PREFIX` and `CMAKE_STAGING_PREFIX` can be skipped if `NO_CMAKE_INSTALL_PREFIX` is passed or by setting the [`CMAKE_FIND_USE_INSTALL_PREFIX`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_USE_INSTALL_PREFIX.html#variable:CMAKE_FIND_USE_INSTALL_PREFIX) to `FALSE`. All these locations can be skipped if `NO_CMAKE_SYSTEM_PATH` is passed or by setting the [`CMAKE_FIND_USE_CMAKE_SYSTEM_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_USE_CMAKE_SYSTEM_PATH.html#variable:CMAKE_FIND_USE_CMAKE_SYSTEM_PATH) to `FALSE`.

   

  - `<prefix>/[s]bin` for each `<prefix>` in [`CMAKE_SYSTEM_PREFIX_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_PREFIX_PATH.html#variable:CMAKE_SYSTEM_PREFIX_PATH)

  - [`CMAKE_SYSTEM_PROGRAM_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_PROGRAM_PATH.html#variable:CMAKE_SYSTEM_PROGRAM_PATH)

  - [`CMAKE_SYSTEM_APPBUNDLE_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_APPBUNDLE_PATH.html#variable:CMAKE_SYSTEM_APPBUNDLE_PATH)

   

  The platform paths that these variables contain are locations that typically include installed software. An example being `/usr/local` for UNIX based platforms.

- Search the paths specified by the PATHS option or in the short-hand version of the command. These are typically hard-coded guesses.

The [`CMAKE_IGNORE_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_IGNORE_PATH.html#variable:CMAKE_IGNORE_PATH), [`CMAKE_IGNORE_PREFIX_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_IGNORE_PREFIX_PATH.html#variable:CMAKE_IGNORE_PREFIX_PATH), [`CMAKE_SYSTEM_IGNORE_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_IGNORE_PATH.html#variable:CMAKE_SYSTEM_IGNORE_PATH) and [`CMAKE_SYSTEM_IGNORE_PREFIX_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_IGNORE_PREFIX_PATH.html#variable:CMAKE_SYSTEM_IGNORE_PREFIX_PATH) variables can also cause some of the above locations to be ignored.

Added in version 3.16: Added `CMAKE_FIND_USE_<CATEGORY>_PATH` variables to globally disable various search locations.

On macOS the [`CMAKE_FIND_FRAMEWORK`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_FRAMEWORK.html#variable:CMAKE_FIND_FRAMEWORK) and [`CMAKE_FIND_APPBUNDLE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_APPBUNDLE.html#variable:CMAKE_FIND_APPBUNDLE) variables determine the order of preference between Apple-style and unix-style package components.

The CMake variable [`CMAKE_FIND_ROOT_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_ROOT_PATH.html#variable:CMAKE_FIND_ROOT_PATH) specifies one or more directories to be prepended to all other search directories. This effectively "re-roots" the entire search under given locations. Paths which are descendants of the [`CMAKE_STAGING_PREFIX`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_STAGING_PREFIX.html#variable:CMAKE_STAGING_PREFIX) are excluded from this re-rooting, because that variable is always a path on the host system. By default the [`CMAKE_FIND_ROOT_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_ROOT_PATH.html#variable:CMAKE_FIND_ROOT_PATH) is empty.

The [`CMAKE_SYSROOT`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSROOT.html#variable:CMAKE_SYSROOT) variable can also be used to specify exactly one directory to use as a prefix. Setting [`CMAKE_SYSROOT`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSROOT.html#variable:CMAKE_SYSROOT) also has other effects. See the documentation for that variable for more.

These variables are especially useful when cross-compiling to point to the root directory of the target environment and CMake will search there too. By default at first the directories listed in [`CMAKE_FIND_ROOT_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_ROOT_PATH.html#variable:CMAKE_FIND_ROOT_PATH) are searched, then the [`CMAKE_SYSROOT`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSROOT.html#variable:CMAKE_SYSROOT) directory is searched, and then the non-rooted directories will be searched. The default behavior can be adjusted by setting [`CMAKE_FIND_ROOT_PATH_MODE_PROGRAM`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_ROOT_PATH_MODE_PROGRAM.html#variable:CMAKE_FIND_ROOT_PATH_MODE_PROGRAM). This behavior can be manually overridden on a per-call basis using options:

**`CMAKE_FIND_ROOT_PATH_BOTH`**

Search in the order described above.

**`NO_CMAKE_FIND_ROOT_PATH`**

Do not use the [`CMAKE_FIND_ROOT_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_ROOT_PATH.html#variable:CMAKE_FIND_ROOT_PATH) variable.

**`ONLY_CMAKE_FIND_ROOT_PATH`**

Search only the re-rooted directories and directories below [`CMAKE_STAGING_PREFIX`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_STAGING_PREFIX.html#variable:CMAKE_STAGING_PREFIX).

The default search order is designed to be most-specific to least-specific for common use cases. Projects may override the order by simply calling the command multiple times and using the `NO_*` options:

~~~
find_program (<VAR> NAMES name PATHS <paths>... NO_DEFAULT_PATH)
find_program (<VAR> NAMES name)
~~~

Once one of the calls succeeds the result variable will be set and stored in the cache so that no call will search again.

When more than one value is given to the `NAMES` option this command by default will consider one name at a time and search every directory for it. The `NAMES_PER_DIR` option tells this command to consider one directory at a time and search for all names in it.

The set of files considered to be programs is platform-specific:

- On Windows, filename suffixes are considered in order `.com`, `.exe`, and no suffix.

- On non-Windows systems, no filename suffix is considered, but files must have execute permission (see policy [`CMP0109`](https://cmake.org/cmake/help/v4.4/policy/CMP0109.html#policy:CMP0109)).

To search for scripts, specify an extension explicitly:

~~~
if(WIN32)
  set(_script_suffix .bat)
else()
  set(_script_suffix .sh)
endif()

find_program(MY_SCRIPT NAMES my_script${_script_suffix})
~~~

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
