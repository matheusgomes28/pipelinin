include(GoogleTest)

add_executable(example_test)
target_sources(example_test
 PRIVATE
  main.cpp)

target_link_libraries(example_test
 PRIVATE
  gtest
  gtest_main)

gtest_discover_tests(example_test)
