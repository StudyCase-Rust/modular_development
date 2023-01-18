#[cfg(feature = "flag_a")]
pub use x_instance_plugin_a as x_instance_plugin;
#[cfg(feature = "flag_b")]
pub use x_instance_plugin_b as x_instance_plugin;

pub fn fn_a() {
  x_plugin();
}
