ROS2 Rust Client Library
^^^^^^^^^^^^^^^^^^^^^^^^

Create ROS2 rust client library from scratch. Doing this project because following reasons:

1. `Rust <https://www.rust-lang.org/>`_ is well developed recently and it almost covers all software world.
   Related to robotics, some of most interested topics is using rust in embeded system
   or controlling system to replace C/C++. In addition, rust is fast and security.
2. `ROS2 <https://index.ros.org/doc/ros2/>`_ is latest software framework for robotics. It includes a
   lot of libraries and tools. The most interesting in ROS2 is it uses `DDS <https://design.ros2.org/articles/ros_on_dds.html>`_
   as fundamental communication system. ROS2 already wrote C API, and other languages,
   like Python, Java, Lisp, etc..., are writing C wrapper to integrate with ROS2.
3. There are several rust library for ROS (ROS1 and ROS2), which listed on `robotics.rs <http://robotics.rs/>`_.
   The lib `ros2_rust <https://github.com/ros2-rust/ros2_rust>`_ is one of most rust support for ROS2.
   BUT this lib recently is not really active, the latest commit is 2019, and there are only 7 commits after 2017.
   Currently rust is developing so fast and there are lots of updates on ROS2 as well.
4. Personaly reason: my work is mainly using Python to write robotics programming. Python is fast
   to integrate at the beginning time, but after having a lot of code, python is really hard to maintain
   since it's too dynamic and there is no "compiler" to help. Rust has a really good compiler
   to help developer, which will capture most of syntax errors when writing code.


Current develop status
^^^^^^^^^^^^^^^^^^^^^^

Rust integrates ROS2 C API such that we are able to use them to create a `node`.

Using `eloquent <https://index.ros.org/doc/ros2/Installation/Eloquent/>`_ currently, and hardcoded `eloquent` in `build.rs`.
