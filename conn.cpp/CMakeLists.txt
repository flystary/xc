cmake_minimum_required(VERSION 3.1)
project(MultiSessionSSH CXX)

# C++11
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# 可执行文件
add_executable(multi_ssh main.cpp)

# 链接 expect 库
find_library(EXPECT_LIB expect)
if(EXPECT_LIB)
    target_link_libraries(multi_ssh ${EXPECT_LIB})
else()
    message(FATAL_ERROR "libexpect not found. Please install libexpect-dev")
endif()

# pthread 支持（多线程）
find_package(Threads REQUIRED)
target_link_libraries(multi_ssh Threads::Threads)
