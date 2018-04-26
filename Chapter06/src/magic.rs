use libc::{c_int, c_void};

#[link(name = "elevatormagic")]
extern {
   pub fn issue_override_code(code: c_int);
   pub fn poll_override_code() -> c_int;
   pub fn poll_override_input_floor() -> c_int;
   pub fn poll_override_error() -> c_int;
   pub fn poll_override_session() -> *const c_void;
   pub fn free_override_session(session: *const c_void);
   pub fn poll_physical_override_privileged_session() -> *const c_void;
   pub fn poll_physical_override_admin_session() -> *const c_void;
   pub fn override_manual_mode();
   pub fn override_normal_mode();
   pub fn override_reset_state();
   pub fn elevator_display_flash(pattern: c_int);
   pub fn elevator_display_toggle_light(light_id: c_int);
   pub fn elevator_display_set_light_color(light_id: c_int, color: c_int);
}
