# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.24

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:

#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:

# Disable VCS-based implicit rules.
% : %,v

# Disable VCS-based implicit rules.
% : RCS/%

# Disable VCS-based implicit rules.
% : RCS/%,v

# Disable VCS-based implicit rules.
% : SCCS/s.%

# Disable VCS-based implicit rules.
% : s.%

.SUFFIXES: .hpux_make_needs_suffix_list

# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:
.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /opt/homebrew/Cellar/cmake/3.24.3/bin/cmake

# The command to remove a file.
RM = /opt/homebrew/Cellar/cmake/3.24.3/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /Users/kayleegeorge/.cargo/registry/src/index.crates.io-6f17d22bba15001f/turbojpeg-sys-1.0.0/libjpeg-turbo

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /Users/kayleegeorge/code/attested-img-editor/img-editor/script/target/release/build/turbojpeg-sys-b4c9f12563688f74/out/build

# Utility rule file for tjtest12.

# Include any custom commands dependencies for this target.
include CMakeFiles/tjtest12.dir/compiler_depend.make

# Include the progress variables for this target.
include CMakeFiles/tjtest12.dir/progress.make

CMakeFiles/tjtest12: /Users/kayleegeorge/.cargo/registry/src/index.crates.io-6f17d22bba15001f/turbojpeg-sys-1.0.0/libjpeg-turbo/cmakescripts/tjbenchtest.cmake
CMakeFiles/tjtest12: tjbenchtest
CMakeFiles/tjtest12: tjexampletest
	/opt/homebrew/Cellar/cmake/3.24.3/bin/cmake -DWITH_JAVA=0 -DPRECISION=12 -P /Users/kayleegeorge/.cargo/registry/src/index.crates.io-6f17d22bba15001f/turbojpeg-sys-1.0.0/libjpeg-turbo/cmakescripts/tjbenchtest.cmake

tjtest12: CMakeFiles/tjtest12
tjtest12: CMakeFiles/tjtest12.dir/build.make
.PHONY : tjtest12

# Rule to build all files generated by this target.
CMakeFiles/tjtest12.dir/build: tjtest12
.PHONY : CMakeFiles/tjtest12.dir/build

CMakeFiles/tjtest12.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/tjtest12.dir/cmake_clean.cmake
.PHONY : CMakeFiles/tjtest12.dir/clean

CMakeFiles/tjtest12.dir/depend:
	cd /Users/kayleegeorge/code/attested-img-editor/img-editor/script/target/release/build/turbojpeg-sys-b4c9f12563688f74/out/build && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /Users/kayleegeorge/.cargo/registry/src/index.crates.io-6f17d22bba15001f/turbojpeg-sys-1.0.0/libjpeg-turbo /Users/kayleegeorge/.cargo/registry/src/index.crates.io-6f17d22bba15001f/turbojpeg-sys-1.0.0/libjpeg-turbo /Users/kayleegeorge/code/attested-img-editor/img-editor/script/target/release/build/turbojpeg-sys-b4c9f12563688f74/out/build /Users/kayleegeorge/code/attested-img-editor/img-editor/script/target/release/build/turbojpeg-sys-b4c9f12563688f74/out/build /Users/kayleegeorge/code/attested-img-editor/img-editor/script/target/release/build/turbojpeg-sys-b4c9f12563688f74/out/build/CMakeFiles/tjtest12.dir/DependInfo.cmake --color=$(COLOR)
.PHONY : CMakeFiles/tjtest12.dir/depend

