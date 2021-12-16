cmake_minimum_required(VERSION 3.15)
project(fltk-check-env)

find_package(OpenGL REQUIRED)
add_executable(check main.cpp)
target_link_libraries(check OpenGL::GL OpenGL::GLU)
target_compile_features(check PUBLIC cxx_std_11)

install(TARGETS check DESTINATION ${CMAKE_INSTALL_PREFIX})