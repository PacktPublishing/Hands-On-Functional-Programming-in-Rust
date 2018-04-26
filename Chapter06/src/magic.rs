use libc::{c_int, c_void};

#[link(name = "poll_override_code")]
extern {
   pub fn poll_override_code() -> c_int;
}

#[link(name = "poll_override_input_floor")]
extern {
   pub fn poll_override_input_floor() -> c_int;
}

#[link(name = "poll_override_errors")]
extern {
   pub fn poll_override_errors() -> c_int;
}

#[link(name = "poll_override_session")]
extern {
   pub fn poll_override_session() -> *const c_void;
}

#[link(name = "free_override_session")]
extern {
   pub fn free_override_session(session: *const c_void);
}

#[link(name = "poll_physical_override_privileged")]
extern {
   pub fn poll_physical_override_privileged() -> *const c_void;
}

#[link(name = "poll_physical_override_admin")]
extern {
   pub fn poll_physical_override_admin() -> *const c_void;
}


#[link(name = "override_manual_mode")]
extern {
   pub fn override_manual_mode();
}

#[link(name = "override_normal_mode")]
extern {
   pub fn override_normal_mode();
}

#[link(name = "override_reset_state")]
extern {
   pub fn override_reset_state();
}

#[link(name = "elevator_display_flash")]
extern {
   pub fn elevator_display_flash(pattern: c_int);
}

#[link(name = "elevator_display_toggle_light")]
extern {
   pub fn elevator_display_toggle_light(light_id: c_int);
}

#[link(name = "elevator_display_set_light_color")]
extern {
   pub fn elevator_display_set_light_color(light_id: c_int, color: c_int);
}
