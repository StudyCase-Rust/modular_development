Project Structure
====

## Packages

* `x_core`: this package is used in each `x_instance*`
* `x_lib_*`: this package is used in each `x_instance*`
* `x_instance_*`: this package is used in `x_instance`
* `x_instance`: this package is the outer package.

* `x_instance`
  * `x_instance_a`
    * `x_core`
    * `x_lib_p` or `x_lib_q`
  * `x_instance_b`
    * `x_core`
    * `x_lib_p` or `x_lib_q`
