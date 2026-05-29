cmake_minimum_required(VERSION 3.10)
project(nav_bridge)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

find_package(ament_cmake REQUIRED)
find_package(rclcpp REQUIRED)
find_package(geometry_msgs REQUIRED)
find_package(sensor_msgs REQUIRED)
find_package(example_interfaces REQUIRED)
find_package(auto_aim_interfaces REQUIRED)

include_directories(
  "/home/xxx/HUN_sentry_xxx/sk2026_xxx_version/HUN_serial"
)

add_executable(nav_ros2_bridge src/nav_ros2_bridge.cpp)
target_include_directories(nav_ros2_bridge PRIVATE
  "/home/xxx/HUN_sentry_xxx/sk2026_xxx_version/HUN_serial"
)
ament_target_dependencies(nav_ros2_bridge
  rclcpp geometry_msgs sensor_msgs example_interfaces auto_aim_interfaces
)

install(TARGETS nav_ros2_bridge DESTINATION lib/${PROJECT_NAME})
ament_package()