# Install script for directory: /Users/kayleegeorge/.cargo/registry/src/index.crates.io-6f17d22bba15001f/turbojpeg-sys-1.0.0/libjpeg-turbo

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/Users/kayleegeorge/code/attested-img-editor/img-editor/script/target/release/build/turbojpeg-sys-b4c9f12563688f74/out")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Release")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

# Set default install directory permissions.
if(NOT DEFINED CMAKE_OBJDUMP)
  set(CMAKE_OBJDUMP "/usr/bin/objdump")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "/Users/kayleegeorge/code/attested-img-editor/img-editor/script/target/release/build/turbojpeg-sys-b4c9f12563688f74/out/build/libturbojpeg.a")
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libturbojpeg.a" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libturbojpeg.a")
    execute_process(COMMAND "/usr/bin/ranlib" "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libturbojpeg.a")
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/bin" TYPE PROGRAM RENAME "tjbench" FILES "/Users/kayleegeorge/code/attested-img-editor/img-editor/script/target/release/build/turbojpeg-sys-b4c9f12563688f74/out/build/tjbench-static")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include" TYPE FILE FILES "/Users/kayleegeorge/.cargo/registry/src/index.crates.io-6f17d22bba15001f/turbojpeg-sys-1.0.0/libjpeg-turbo/turbojpeg.h")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "/Users/kayleegeorge/code/attested-img-editor/img-editor/script/target/release/build/turbojpeg-sys-b4c9f12563688f74/out/build/libjpeg.a")
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libjpeg.a" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libjpeg.a")
    execute_process(COMMAND "/usr/bin/ranlib" "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libjpeg.a")
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/bin" TYPE PROGRAM RENAME "cjpeg" FILES "/Users/kayleegeorge/code/attested-img-editor/img-editor/script/target/release/build/turbojpeg-sys-b4c9f12563688f74/out/build/cjpeg-static")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/bin" TYPE PROGRAM RENAME "djpeg" FILES "/Users/kayleegeorge/code/attested-img-editor/img-editor/script/target/release/build/turbojpeg-sys-b4c9f12563688f74/out/build/djpeg-static")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/bin" TYPE PROGRAM RENAME "jpegtran" FILES "/Users/kayleegeorge/code/attested-img-editor/img-editor/script/target/release/build/turbojpeg-sys-b4c9f12563688f74/out/build/jpegtran-static")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/bin" TYPE EXECUTABLE FILES "/Users/kayleegeorge/code/attested-img-editor/img-editor/script/target/release/build/turbojpeg-sys-b4c9f12563688f74/out/build/rdjpgcom")
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/bin/rdjpgcom" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/bin/rdjpgcom")
    if(CMAKE_INSTALL_DO_STRIP)
      execute_process(COMMAND "/usr/bin/strip" -u -r "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/bin/rdjpgcom")
    endif()
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/bin" TYPE EXECUTABLE FILES "/Users/kayleegeorge/code/attested-img-editor/img-editor/script/target/release/build/turbojpeg-sys-b4c9f12563688f74/out/build/wrjpgcom")
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/bin/wrjpgcom" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/bin/wrjpgcom")
    if(CMAKE_INSTALL_DO_STRIP)
      execute_process(COMMAND "/usr/bin/strip" -u -r "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/bin/wrjpgcom")
    endif()
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/doc/libjpeg-turbo" TYPE FILE FILES
    "/Users/kayleegeorge/.cargo/registry/src/index.crates.io-6f17d22bba15001f/turbojpeg-sys-1.0.0/libjpeg-turbo/README.ijg"
    "/Users/kayleegeorge/.cargo/registry/src/index.crates.io-6f17d22bba15001f/turbojpeg-sys-1.0.0/libjpeg-turbo/README.md"
    "/Users/kayleegeorge/.cargo/registry/src/index.crates.io-6f17d22bba15001f/turbojpeg-sys-1.0.0/libjpeg-turbo/example.c"
    "/Users/kayleegeorge/.cargo/registry/src/index.crates.io-6f17d22bba15001f/turbojpeg-sys-1.0.0/libjpeg-turbo/tjexample.c"
    "/Users/kayleegeorge/.cargo/registry/src/index.crates.io-6f17d22bba15001f/turbojpeg-sys-1.0.0/libjpeg-turbo/libjpeg.txt"
    "/Users/kayleegeorge/.cargo/registry/src/index.crates.io-6f17d22bba15001f/turbojpeg-sys-1.0.0/libjpeg-turbo/structure.txt"
    "/Users/kayleegeorge/.cargo/registry/src/index.crates.io-6f17d22bba15001f/turbojpeg-sys-1.0.0/libjpeg-turbo/usage.txt"
    "/Users/kayleegeorge/.cargo/registry/src/index.crates.io-6f17d22bba15001f/turbojpeg-sys-1.0.0/libjpeg-turbo/wizard.txt"
    "/Users/kayleegeorge/.cargo/registry/src/index.crates.io-6f17d22bba15001f/turbojpeg-sys-1.0.0/libjpeg-turbo/LICENSE.md"
    )
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/man/man1" TYPE FILE FILES
    "/Users/kayleegeorge/.cargo/registry/src/index.crates.io-6f17d22bba15001f/turbojpeg-sys-1.0.0/libjpeg-turbo/cjpeg.1"
    "/Users/kayleegeorge/.cargo/registry/src/index.crates.io-6f17d22bba15001f/turbojpeg-sys-1.0.0/libjpeg-turbo/djpeg.1"
    "/Users/kayleegeorge/.cargo/registry/src/index.crates.io-6f17d22bba15001f/turbojpeg-sys-1.0.0/libjpeg-turbo/jpegtran.1"
    "/Users/kayleegeorge/.cargo/registry/src/index.crates.io-6f17d22bba15001f/turbojpeg-sys-1.0.0/libjpeg-turbo/rdjpgcom.1"
    "/Users/kayleegeorge/.cargo/registry/src/index.crates.io-6f17d22bba15001f/turbojpeg-sys-1.0.0/libjpeg-turbo/wrjpgcom.1"
    )
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/pkgconfig" TYPE FILE FILES "/Users/kayleegeorge/code/attested-img-editor/img-editor/script/target/release/build/turbojpeg-sys-b4c9f12563688f74/out/build/pkgscripts/libjpeg.pc")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/pkgconfig" TYPE FILE FILES "/Users/kayleegeorge/code/attested-img-editor/img-editor/script/target/release/build/turbojpeg-sys-b4c9f12563688f74/out/build/pkgscripts/libturbojpeg.pc")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/libjpeg-turbo" TYPE FILE FILES
    "/Users/kayleegeorge/code/attested-img-editor/img-editor/script/target/release/build/turbojpeg-sys-b4c9f12563688f74/out/build/pkgscripts/libjpeg-turboConfig.cmake"
    "/Users/kayleegeorge/code/attested-img-editor/img-editor/script/target/release/build/turbojpeg-sys-b4c9f12563688f74/out/build/pkgscripts/libjpeg-turboConfigVersion.cmake"
    )
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/libjpeg-turbo/libjpeg-turboTargets.cmake")
    file(DIFFERENT _cmake_export_file_changed FILES
         "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/libjpeg-turbo/libjpeg-turboTargets.cmake"
         "/Users/kayleegeorge/code/attested-img-editor/img-editor/script/target/release/build/turbojpeg-sys-b4c9f12563688f74/out/build/CMakeFiles/Export/f0d506f335508d6549928070f26fb787/libjpeg-turboTargets.cmake")
    if(_cmake_export_file_changed)
      file(GLOB _cmake_old_config_files "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/libjpeg-turbo/libjpeg-turboTargets-*.cmake")
      if(_cmake_old_config_files)
        string(REPLACE ";" ", " _cmake_old_config_files_text "${_cmake_old_config_files}")
        message(STATUS "Old export file \"$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/libjpeg-turbo/libjpeg-turboTargets.cmake\" will be replaced.  Removing files [${_cmake_old_config_files_text}].")
        unset(_cmake_old_config_files_text)
        file(REMOVE ${_cmake_old_config_files})
      endif()
      unset(_cmake_old_config_files)
    endif()
    unset(_cmake_export_file_changed)
  endif()
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/libjpeg-turbo" TYPE FILE FILES "/Users/kayleegeorge/code/attested-img-editor/img-editor/script/target/release/build/turbojpeg-sys-b4c9f12563688f74/out/build/CMakeFiles/Export/f0d506f335508d6549928070f26fb787/libjpeg-turboTargets.cmake")
  if(CMAKE_INSTALL_CONFIG_NAME MATCHES "^([Rr][Ee][Ll][Ee][Aa][Ss][Ee])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/libjpeg-turbo" TYPE FILE FILES "/Users/kayleegeorge/code/attested-img-editor/img-editor/script/target/release/build/turbojpeg-sys-b4c9f12563688f74/out/build/CMakeFiles/Export/f0d506f335508d6549928070f26fb787/libjpeg-turboTargets-release.cmake")
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include" TYPE FILE FILES
    "/Users/kayleegeorge/code/attested-img-editor/img-editor/script/target/release/build/turbojpeg-sys-b4c9f12563688f74/out/build/jconfig.h"
    "/Users/kayleegeorge/.cargo/registry/src/index.crates.io-6f17d22bba15001f/turbojpeg-sys-1.0.0/libjpeg-turbo/jerror.h"
    "/Users/kayleegeorge/.cargo/registry/src/index.crates.io-6f17d22bba15001f/turbojpeg-sys-1.0.0/libjpeg-turbo/jmorecfg.h"
    "/Users/kayleegeorge/.cargo/registry/src/index.crates.io-6f17d22bba15001f/turbojpeg-sys-1.0.0/libjpeg-turbo/jpeglib.h"
    )
endif()

if(NOT CMAKE_INSTALL_LOCAL_ONLY)
  # Include the install script for each subdirectory.
  include("/Users/kayleegeorge/code/attested-img-editor/img-editor/script/target/release/build/turbojpeg-sys-b4c9f12563688f74/out/build/simd/cmake_install.cmake")
  include("/Users/kayleegeorge/code/attested-img-editor/img-editor/script/target/release/build/turbojpeg-sys-b4c9f12563688f74/out/build/md5/cmake_install.cmake")

endif()

if(CMAKE_INSTALL_COMPONENT)
  set(CMAKE_INSTALL_MANIFEST "install_manifest_${CMAKE_INSTALL_COMPONENT}.txt")
else()
  set(CMAKE_INSTALL_MANIFEST "install_manifest.txt")
endif()

string(REPLACE ";" "\n" CMAKE_INSTALL_MANIFEST_CONTENT
       "${CMAKE_INSTALL_MANIFEST_FILES}")
file(WRITE "/Users/kayleegeorge/code/attested-img-editor/img-editor/script/target/release/build/turbojpeg-sys-b4c9f12563688f74/out/build/${CMAKE_INSTALL_MANIFEST}"
     "${CMAKE_INSTALL_MANIFEST_CONTENT}")
