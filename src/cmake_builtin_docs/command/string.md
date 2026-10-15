CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/command/string.html)

# string

String operations.

## Synopsis

~~~
Search and Replace
  string(FIND <string> <substring> <out-var> [...])
  string(REPLACE <match-string> <replace-string> <out-var> <input>...)
  string(REGEX MATCH <match-regex> <out-var> <input>...)
  string(REGEX MATCHALL <match-regex> <out-var> <input>...)
  string(REGEX REPLACE <match-regex> <replace-expr> <out-var> <input>...)

Manipulation
  string(APPEND <string-var> [<input>...])
  string(PREPEND <string-var> [<input>...])
  string(CONCAT <out-var> [<input>...])
  string(JOIN <glue> <out-var> [<input>...])
  string(TOLOWER <string> <out-var>)
  string(TOUPPER <string> <out-var>)
  string(LENGTH <string> <out-var>)
  string(SUBSTRING <string> <begin> <length> <out-var>)
  string(STRIP <string> <out-var>)
  string(GENEX_STRIP <string> <out-var>)
  string(REPEAT <string> <count> <out-var>)
  string(REGEX QUOTE <out-var> <input>...)

Comparison
  string(COMPARE <op> <string1> <string2> <out-var>)

Hashing
  string(<HASH> <out-var> <input>)

Generation
  string(ASCII <number>... <out-var>)
  string(HEX <string> <out-var>)
  string(CONFIGURE <string> <out-var> [...])
  string(MAKE_C_IDENTIFIER <string> <out-var>)
  string(RANDOM [<option>...] <out-var>)
  string(TIMESTAMP <out-var> [<format string>] [UTC])
  string(UUID <out-var> ...)

JSON
  string(JSON <out-var> [ERROR_VARIABLE <error-var>]
         {GET | GET_RAW | TYPE | LENGTH}
         <json-string> [<member|index> ...])
  string(JSON <out-var> [ERROR_VARIABLE <error-var>]
         REMOVE
         <json-string> <member|index> [<member|index> ...])
  string(JSON <out-var> [ERROR_VARIABLE <error-var>]
         MEMBER <json-string>
         [<member|index> ...] <index>)
  string(JSON <out-var> [ERROR_VARIABLE <error-var>]
         SET <json-string>
         <member|index> [<member|index> ...] <value>)
  string(JSON <out-var> [ERROR_VARIABLE <error-var>]
         EQUAL <json-string1> <json-string2>)
  string(JSON <out-var> [ERROR_VARIABLE <error-var>]
         PARTIAL_EQUAL <json-string1> <json-string2>)
  string(JSON <out-var> [ERROR_VARIABLE <error-var>]
         STRING_ENCODE <string>)
~~~

## Search and Replace

### Search and Replace With Plain Strings

**string(FIND \<string\> \<substring\> \<output_variable\> [REVERSE])**

Return the position where the given `<substring>` was found in the supplied `<string>`. If the `REVERSE` flag was used, the command will search for the position of the last occurrence of the specified `<substring>`. If the `<substring>` is not found, a position of -1 is returned.

The `string(FIND)` subcommand treats all strings as ASCII-only characters. The index stored in `<output_variable>` will also be counted in bytes, so strings containing multi-byte characters may lead to unexpected results.

**string(REPLACE \<match_string\> \<replace_string\> \<output_variable\> \<input\> [\<input\>...])**

Replace all occurrences of `<match_string>` in the `<input>` with `<replace_string>` and store the result in the `<output_variable>`.

### Search and Replace With Regular Expressions

**string(REGEX MATCH \<regular_expression\> \<output_variable\> \<input\> [\<input\>...])**

Match the `<regular_expression>` once and store the match in the `<output_variable>`. All `<input>` arguments are concatenated before matching. Regular expressions are specified in the subsection just below.

**string(REGEX MATCHALL \<regular_expression\> \<output_variable\> \<input\> [\<input\>...])**

Match the `<regular_expression>` as many times as possible and store the matches in the `<output_variable>` as a list. All `<input>` arguments are concatenated before matching.

**string(REGEX REPLACE \<regular_expression\> \<replacement_expression\> \<output_variable\> \<input\> [\<input\>...])**

Match the `<regular_expression>` as many times as possible and substitute the `<replacement_expression>` for the match in the output. All `<input>` arguments are concatenated before matching.

The `<replacement_expression>` may refer to parenthesis-delimited subexpressions of the match using `\1`, `\2`, ..., `\9`. Note that two backslashes (`\\1`) are required in CMake code to get a backslash through argument parsing.

Changed in version 4.1: The `^` anchor now matches only at the beginning of the input string instead of the beginning of each repeated search. See policy [`CMP0186`](https://cmake.org/cmake/help/v4.4/policy/CMP0186.html#policy:CMP0186).

Zero-length matches are allowed in `MATCHALL` and `REPLACE`. Previously, they triggered an error.

The replacement expression may contain references to subexpressions that didn't match anything. Previously, such references triggered an error.

### Regex Specification

The following characters have special meaning in regular expressions:

**`^`**

Matches at beginning of input

**`$`**

Matches at end of input

**`.`**

Matches any single character

**`\<char>`**

Matches the single character specified by `<char>`. Use this to match special regex characters, e.g. `\.` for a literal `.` or `\\` for a literal backslash `\`. Escaping a non-special character is unnecessary but allowed, e.g. `\a` matches `a`.

**`[ ]`**

Matches any character(s) inside the brackets. To match a literal `]`, make it the first character, e.g., `[]ab]`.

**`[^ ]`**

Matches any character(s) not inside the brackets. To not match a literal `]`, make it the first character, e.g., `[^]ab]`.

**`-`**

Inside brackets, specifies an inclusive range between characters on either side, e.g., `[a-f]` is `[abcdef]`. To match a literal `-` using brackets, make it the first or the last character, e.g., `[+*/-]` matches basic mathematical operators.

**`*`**

Matches preceding pattern zero or more times

**`+`**

Matches preceding pattern one or more times

**`?`**

Matches preceding pattern zero or once only

**`|`**

Matches a pattern on either side of the `|`

**`()`**

Saves a matched subexpression, which can be referenced in the `REGEX REPLACE` operation.

Added in version 3.9: All regular expression-related commands, including e.g. [`if(MATCHES)`](https://cmake.org/cmake/help/v4.4/command/if.html#matches), save subgroup matches in the variables [`CMAKE_MATCH_<n>`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MATCH_n.html#variable:CMAKE_MATCH_%3Cn%3E) for `<n>` 0..9.

`*`, `+` and `?` have higher precedence than concatenation. `|` has lower precedence than concatenation. This means that the regular expression `^ab+d$` matches `abbd` but not `ababd`, and the regular expression `^(ab|cd)$` matches `ab` but not `abd`.

CMake language [Escape Sequences](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#escape-sequences) such as `\t`, `\r`, `\n`, and `\\` may be used to construct literal tabs, carriage returns, newlines, and backslashes (respectively) to pass in a regex. For example:

- The quoted argument `"[ \t\r\n]"` specifies a regex that matches any single whitespace character.

- The quoted argument `"[/\\]"` specifies a regex that matches a single forward slash `/` or backslash `\`.

- The quoted argument `"[A-Za-z0-9_]"` specifies a regex that matches any single "word" character in the C locale.

- The quoted argument `"\\(\\a\\+b\\)"` specifies a regex that matches the exact string `(a+b)`. Each `\\` is parsed in a quoted argument as just `\`, so the regex itself is actually `\(\a\+\b\)`. This can alternatively be specified in a [Bracket Argument](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#bracket-argument) without having to escape the backslashes, e.g. `[[\(\a\+\b\)]]`.

## Manipulation

**string(APPEND \<string_variable\> [\<input\>...])**

Added in version 3.4.

Append all the `<input>` arguments to the string.

**string(PREPEND \<string_variable\> [\<input\>...])**

Added in version 3.10.

Prepend all the `<input>` arguments to the string.

**string(CONCAT \<output_variable\> [\<input\>...])**

Concatenate all the `<input>` arguments together and store the result in the named `<output_variable>`.

**string(JOIN \<glue\> \<output_variable\> [\<input\>...])**

Added in version 3.12.

Join all the `<input>` arguments together using the `<glue>` string and store the result in the named `<output_variable>`.

To join a list's elements, prefer to use the `JOIN` operator from the [`list()`](https://cmake.org/cmake/help/v4.4/command/list.html#command:list) command. This allows for the elements to have special characters like `;` in them.

**string(TOLOWER \<string\> \<output_variable\>)**

Convert `<string>` to lower characters.

**string(TOUPPER \<string\> \<output_variable\>)**

Convert `<string>` to upper characters.

**string(LENGTH \<string\> \<output_variable\>)**

Store in an `<output_variable>` a given string's length in bytes. Note that this means if `<string>` contains multi-byte characters, the result stored in `<output_variable>` will *not* be the number of characters.

**string(SUBSTRING \<string\> \<begin\> \<length\> \<output_variable\>)**

Store in an `<output_variable>` a substring of a given `<string>`. If `<length>` is `-1` the remainder of the string starting at `<begin>` will be returned.

Changed in version 3.2: If `<string>` is shorter than `<length>` then the end of the string is used instead. Previous versions of CMake reported an error in this case.

Both `<begin>` and `<length>` are counted in bytes, so care must be exercised if `<string>` could contain multi-byte characters.

**string(STRIP \<string\> \<output_variable\>)**

Store in an `<output_variable>` a substring of a given `<string>` with leading and trailing spaces removed.

**string(GENEX_STRIP \<string\> \<output_variable\>)**

Added in version 3.1.

Strip any [`generator expressions`](https://cmake.org/cmake/help/v4.4/manual/cmake-generator-expressions.7.html#manual:cmake-generator-expressions(7)) from the input `<string>` and store the result in the `<output_variable>`.

**string(REPEAT \<string\> \<count\> \<output_variable\>)**

Added in version 3.15.

Produce the output string as the input `<string>` repeated `<count>` times.

**string(REGEX QUOTE \<out-var\> \<input\>...)**

Added in version 4.2.

Store in an `<out-var>` a regular expression matching the `<input>`. All characters that have special meaning in a regular expression are escaped, such that the output string can be used as part of a regular expression to match the input literally.

## Comparison

**string(COMPARE LESS \<string1\> \<string2\> \<output_variable\>)**

**string(COMPARE GREATER \<string1\> \<string2\> \<output_variable\>)**

**string(COMPARE EQUAL \<string1\> \<string2\> \<output_variable\>)**

**string(COMPARE NOTEQUAL \<string1\> \<string2\> \<output_variable\>)**

**string(COMPARE LESS_EQUAL \<string1\> \<string2\> \<output_variable\>)**

**string(COMPARE GREATER_EQUAL \<string1\> \<string2\> \<output_variable\>)**

Compare the strings and store true or false in the `<output_variable>`.

Added in version 3.7: Added the `LESS_EQUAL` and `GREATER_EQUAL` options.

## Hashing

**string(\<HASH\> \<output_variable\> \<input\>)**

Compute a cryptographic hash of the `<input>` string. The supported `<HASH>` algorithm names are:

**`MD5`**

Message-Digest Algorithm 5, RFC 1321.

**`SHA1`**

US Secure Hash Algorithm 1, RFC 3174.

**`SHA224`**

US Secure Hash Algorithms, RFC 4634.

**`SHA256`**

US Secure Hash Algorithms, RFC 4634.

**`SHA384`**

US Secure Hash Algorithms, RFC 4634.

**`SHA512`**

US Secure Hash Algorithms, RFC 4634.

**`SHA3_224`**

Keccak SHA-3.

**`SHA3_256`**

Keccak SHA-3.

**`SHA3_384`**

Keccak SHA-3.

**`SHA3_512`**

Keccak SHA-3.

Added in version 3.8: Added the `SHA3_*` hash algorithms.

## Generation

**string(ASCII \<number\> [\<number\> ...] \<output_variable\>)**

Convert all numbers into corresponding ASCII characters.

**string(HEX \<string\> \<output_variable\>)**

Added in version 3.18.

Convert each byte in the input `<string>` to its hexadecimal representation and store the concatenated hex digits in the `<output_variable>`. Letters in the output (`a` through `f`) are in lowercase.

**string(CONFIGURE \<string\> \<output_variable\> [@ONLY] [ESCAPE_QUOTES])**

Transform a `<string>` like [`configure_file()`](https://cmake.org/cmake/help/v4.4/command/configure_file.html#command:configure_file) transforms a file.

**string(MAKE_C_IDENTIFIER \<string\> \<output_variable\>)**

Convert each non-alphanumeric character in the input `<string>` to an underscore and store the result in the `<output_variable>`. If the first character of the `<string>` is a digit, an underscore will also be prepended to the result.

**string(RANDOM [LENGTH \<length\>] [ALPHABET \<alphabet\>] [RANDOM_SEED \<seed\>] \<output_variable\>)**

Return a random string of given `<length>` consisting of characters from the given `<alphabet>`. Default length is 5 characters and default alphabet is all numbers and upper and lower case letters. If an integer `RANDOM_SEED` is given, its value will be used to seed the random number generator.

**string(TIMESTAMP \<output_variable\> [\<format_string\>] [UTC])**

Write a string representation of the current date and/or time to the `<output_variable>`.

If the command is unable to obtain a timestamp, the `<output_variable>` will be set to the empty string `""`.

The optional `UTC` flag requests the current date/time representation to be in Coordinated Universal Time (UTC) rather than local time.

The optional `<format_string>` may contain the following format specifiers:

**`%%`**

Added in version 3.8.

A literal percent sign (%).

**`%d`**

The day of the current month (01-31).

**`%H`**

The hour on a 24-hour clock (00-23).

**`%I`**

The hour on a 12-hour clock (01-12).

**`%j`**

The day of the current year (001-366).

**`%m`**

The month of the current year (01-12).

**`%b`**

Added in version 3.7.

Abbreviated month name (e.g. Oct).

**`%B`**

Added in version 3.10.

Full month name (e.g. October).

**`%M`**

The minute of the current hour (00-59).

**`%s`**

Added in version 3.6.

Seconds since midnight (UTC) 1-Jan-1970 (UNIX time).

**`%S`**

The second of the current minute. 60 represents a leap second. (00-60)

**`%f`**

Added in version 3.23.

The microsecond of the current second (000000-999999).

**`%U`**

The week number of the current year (00-53).

**`%V`**

Added in version 3.22.

The ISO 8601 week number of the current year (01-53).

**`%w`**

The day of the current week. 0 is Sunday. (0-6)

**`%a`**

Added in version 3.7.

Abbreviated weekday name (e.g. Fri).

**`%A`**

Added in version 3.10.

Full weekday name (e.g. Friday).

**`%y`**

The last two digits of the current year (00-99).

**`%Y`**

The current year.

**`%z`**

Added in version 3.26.

The offset of the time zone from UTC, in hours and minutes, with format `+hhmm` or `-hhmm`.

**`%Z`**

Added in version 3.26.

The time zone name.

Unknown format specifiers will be ignored and copied to the output as-is.

If no explicit `<format_string>` is given, it will default to:

- `%Y-%m-%dT%H:%M:%S` for local time.

- `%Y-%m-%dT%H:%M:%SZ` for UTC.

Added in version 3.8: If the `SOURCE_DATE_EPOCH` environment variable is set, its value will be used instead of the current time. See [https://reproducible-builds.org/specs/source-date-epoch/](https://reproducible-builds.org/specs/source-date-epoch/) for details.

**string(UUID \<output_variable\> NAMESPACE \<namespace\> NAME \<name\> TYPE \<MD5|SHA1\> [UPPER])**

Added in version 3.1.

Create a universally unique identifier (aka GUID) as per RFC4122 based on the hash of the combined values of `<namespace>` (which itself has to be a valid UUID) and `<name>`. The hash algorithm can be either `MD5` (Version 3 UUID) or `SHA1` (Version 5 UUID). A UUID has the format `xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx` where each `x` represents a lower case hexadecimal character. Where required, an uppercase representation can be requested with the optional `UPPER` flag.

## JSON

Added in version 3.19.

Functionality for querying a JSON string.

Note

In each of the following JSON-related subcommands, if the optional `ERROR_VARIABLE` argument is given, errors will be reported in `<error-variable>` and the `<out-var>` will be set to `<member|index>-[<member|index>...]-NOTFOUND` with the path elements up to the point where the error occurred, or just `NOTFOUND` if there is no relevant path. If an error occurs but the `ERROR_VARIABLE` option is not present, a fatal error message is generated. If no error occurs, the `<error-variable>` will be set to `NOTFOUND`.

In the following subcommands, the `<json-string>` argument should be written as a [Quoted Argument](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#quoted-argument) to ensure the entire JSON string is passed as a single argument even if it contains semicolons.

**string(JSON \<out-var\> [ERROR_VARIABLE \<error-variable\>] GET \<json-string\> [\<member|index\> ...])**

Get an element from `<json-string>` at the location given by the list of `<member|index>` arguments. Array and object elements will be returned as a JSON string. Boolean elements will be returned as `ON` or `OFF`. Null elements will be returned as an empty string. Number and string types will be returned as strings.

**string(JSON \<out-var\> [ERROR_VARIABLE \<error-variable\>] GET_RAW \<json-string\> [\<member|index\> ...])**

Added in version 4.3.

Get an element from `<json-string>` at the location given by the list of `<member|index>` arguments. Similar to [`GET`](https://cmake.org/cmake/help/v4.4/command/string.html#json-get), but does not convert number, string, boolean, or null elements.

**string(JSON \<out-var\> [ERROR_VARIABLE \<error-variable\>] TYPE \<json-string\> [\<member|index\> ...])**

Get the type of an element in `<json-string>` at the location given by the list of `<member|index>` arguments. The `<out-var>` will be set to one of `NULL`, `NUMBER`, `STRING`, `BOOLEAN`, `ARRAY`, or `OBJECT`.

**string(JSON \<out-var\> [ERROR_VARIABLE \<error-var\>] MEMBER \<json-string\> [\<member|index\> ...] \<index\>)**

Get the name of the `<index>`-th member in `<json-string>` at the location given by the list of `<member|index>` arguments. Requires an element of object type.

**string(JSON \<out-var\> [ERROR_VARIABLE \<error-variable\>] LENGTH \<json-string\> [\<member|index\> ...])**

Get the length of an element in `<json-string>` at the location given by the list of `<member|index>` arguments. Requires an element of array or object type.

**string(JSON \<out-var\> [ERROR_VARIABLE \<error-variable\>] REMOVE \<json-string\> \<member|index\> [\<member|index\> ...])**

Remove an element from `<json-string>` at the location given by the list of `<member|index>` arguments. The JSON string without the removed element will be stored in `<out-var>`.

**string(JSON \<out-var\> [ERROR_VARIABLE \<error-variable\>] SET \<json-string\> \<member|index\> [\<member|index\> ...] \<value\>)**

Set an element in `<json-string>` at the location given by the list of `<member|index>` arguments to `<value>`. The contents of `<value>` should be valid JSON. If `<json-string>` is an array, `<value>` can be appended to the end of the array by using a number greater or equal to the array length as the `<member|index>` argument.

**string(JSON \<out-var\> [ERROR_VARIABLE \<error-var\>] EQUAL \<json-string1\> \<json-string2\>)**

Compare the two JSON objects given by `<json-string1>` and `<json-string2>` for equality. The contents of `<json-string1>` and `<json-string2>` should be valid JSON. The `<out-var>` will be set to a true value if the JSON objects are considered equal, or a false value otherwise.

**string(JSON \<out-var\> [ERROR_VARIABLE \<error-var\>] PARTIAL_EQUAL \<json-pattern\> \<json-actual\>)**

Added in version 4.4.

Compare two JSON values using *partial* equality. The first argument (`<json-pattern>`) is treated as a pattern to match against the second argument (`<json-actual>`), which is treated as the complete JSON value. The contents of `<json-pattern>` and `<json-actual>` must be valid JSON.

The `<out-var>` will be set to a true value if `<json-pattern>` is contained within `<json-actual>` according to the rules below, or a false value otherwise.

Partial equality is not symmetric. In general, `PARTIAL_EQUAL(A, B)` and `PARTIAL_EQUAL(B, A)` may produce different results.

Matching rules:

- **Scalars** (null, number, string, boolean) must match exactly.

- **Objects** match if every member in `<json-pattern>` exists in `<json-actual>` and the corresponding values also match recursively. Extra members in `<json-actual>` are ignored.

- **Arrays** match if every element in `<json-pattern>` matches a distinct element in `<json-actual>` recursively. Once an element in `<json-actual>` successfully matches an element in `<json-pattern>`, it is consumed and cannot be reused to match any subsequent elements. Array element order is not significant. Extra elements in `<json-actual>` are ignored.

Type differences do not match (for example, an object does not match an array, and a string does not match a number).

**string(JSON \<out-var\> [ERROR_VARIABLE \<error-var\>] STRING_ENCODE \<string\>)**

Added in version 4.3.

Turn a raw string into a JSON string surrounded by quotes. Special characters will be properly escaped inside the JSON string.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
