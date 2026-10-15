CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_COMPILER_LINKER_ID.html)

# CMAKE_\<LANG\>_COMPILER_LINKER_ID

Added in version 3.29.

Linker identification string.

A short string unique to the linker vendor. Possible values include:

Value |  Name |

`AIX` |  AIX system linker |

`AppleClang` |  Apple Clang |

`LLD` |  [LLVM LLD](https://lld.llvm.org) |

`GNU` |  [GNU Binutils - ld linker](https://sourceware.org/binutils) (also known as `bfd`) |

`GNUgold` |  [GNU Binutils - gold linker](https://sourceware.org/binutils) |

`MSVC` |  [Microsoft Visual Studio](https://visualstudio.microsoft.com) |

`MOLD` |  [mold: A Modern Linker](https://github.com/rui314/mold), or on Apple the [sold](https://github.com/bluewhalesystems/sold) linker |

`Solaris` |  SunOS system linker |

`WILD` |  [wild](https://github.com/wild-linker/wild) linker |

`OpenWatcom` |  [OpenWatcom wlink](https://www.openwatcom.com/) linker |

This variable is not guaranteed to be defined for all linkers or languages.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
