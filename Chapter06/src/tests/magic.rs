use magic;
use libc::{c_void};

#[test]
fn issue_override_code() {
   unsafe {
      magic::override_reset_state();
      magic::issue_override_code(1);
      assert!(magic::poll_override_code() == 1);
      assert!(magic::poll_override_error() == 0);
   }
}
#[test]
fn issue_privileged_code() {
   unsafe {
      magic::override_reset_state();
      magic::issue_override_code(2);
      assert!(magic::poll_override_code() == 2);
      assert!(magic::poll_override_error() == 0);
   }
}
#[test]
fn issue_admin_code() {
   unsafe {
      magic::override_reset_state();
      magic::issue_override_code(3);
      assert!(magic::poll_override_code() == 3);
      assert!(magic::poll_override_error() == 0);
   }
}

#[test]
fn authorize_override_success() {
   unsafe {
      magic::override_reset_state();
      magic::issue_override_code(1);
      let session = magic::poll_override_session();
      assert!(session != (0 as *const c_void));
      magic::free_override_session(session);
      assert!(magic::poll_override_error() == 0);
   }
}
#[test]
fn authorize_privileged_success() {
   unsafe {
      magic::override_reset_state();
      magic::issue_override_code(2);
      let session = magic::poll_physical_override_privileged_session();
      assert!(session != (0 as *const c_void));
      magic::free_override_session(session);
      assert!(magic::poll_override_error() == 0);
   }
}
#[test]
fn authorize_admin_success() {
   unsafe {
      magic::override_reset_state();
      magic::issue_override_code(3);
      let session = magic::poll_physical_override_admin_session();
      assert!(session != (0 as *const c_void));
      magic::free_override_session(session);
      assert!(magic::poll_override_error() == 0);
   }
}

#[test]
fn double_override_failure() {
   unsafe {
      magic::override_reset_state();
      magic::issue_override_code(1);
      magic::issue_override_code(1);
      assert!(magic::poll_override_session() == (0 as *const c_void));
      assert!(magic::poll_override_error() == 1);
   }
}
#[test]
fn double_privileged_failure() {
   unsafe {
      magic::override_reset_state();
      magic::issue_override_code(2);
      magic::issue_override_code(2);
      assert!(magic::poll_physical_override_privileged_session() == (0 as *const c_void));
      assert!(magic::poll_override_error() == 1);
   }
}
#[test]
fn double_admin_failure() {
   unsafe {
      magic::override_reset_state();
      magic::issue_override_code(3);
      magic::issue_override_code(3);
      assert!(magic::poll_physical_override_admin_session() == (0 as *const c_void));
      assert!(magic::poll_override_error() == 1);
   }
}

#[test]
fn double_free_override_failure() {
   unsafe {
      magic::override_reset_state();
      magic::issue_override_code(1);
      let session = magic::poll_override_session();
      assert!(session != (0 as *const c_void));
      magic::free_override_session(session);
      magic::free_override_session(session);
      assert!(magic::poll_override_error() == 2);
   }
}
#[test]
fn double_free_privileged_failure() {
   unsafe {
      magic::override_reset_state();
      magic::issue_override_code(2);
      let session = magic::poll_physical_override_privileged_session();
      assert!(session != (0 as *const c_void));
      magic::free_override_session(session);
      magic::free_override_session(session);
      assert!(magic::poll_override_error() == 2);
   }
}
#[test]
fn double_free_admin_failure() {
   unsafe {
      magic::override_reset_state();
      magic::issue_override_code(3);
      let session = magic::poll_physical_override_admin_session();
      assert!(session != (0 as *const c_void));
      magic::free_override_session(session);
      magic::free_override_session(session);
      assert!(magic::poll_override_error() == 2);
   }
}

#[test]
fn input_floor() {
   unsafe {
      magic::override_reset_state();
      magic::issue_override_code(4);
      magic::issue_override_code(2);
      assert!(magic::poll_override_code() == 4);
      assert!(magic::poll_override_code() == 2);
   }
}

#[test]
fn manual_mode() {
   unsafe {
      magic::override_reset_state();
      magic::issue_override_code(5);
      assert!(magic::poll_override_code() == 5);
   }
}

#[test]
fn normal_mode() {
   unsafe {
      magic::override_reset_state();
      magic::issue_override_code(6);
      assert!(magic::poll_override_code() == 6);
   }
}

#[test]
fn flash() {
   unsafe {
      magic::override_reset_state();
      magic::elevator_display_flash(222);
      assert!(magic::poll_override_code() == 7);
      assert!(magic::poll_override_code() == 222);
   }
}

#[test]
fn toggle_light() {
   unsafe {
      magic::override_reset_state();
      magic::elevator_display_toggle_light(33);
      assert!(magic::poll_override_code() == 8);
      assert!(magic::poll_override_code() == 33);
      assert!(magic::poll_override_code() == 1);
      magic::elevator_display_toggle_light(33);
      assert!(magic::poll_override_code() == 8);
      assert!(magic::poll_override_code() == 33);
      assert!(magic::poll_override_code() == 0);
   }
}

#[test]
fn set_light_color() {
   unsafe {
      magic::override_reset_state();
      magic::elevator_display_set_light_color(33, 222);
      assert!(magic::poll_override_code() == 9);
      assert!(magic::poll_override_code() == 33);
      assert!(magic::poll_override_code() == 222);
   }
}

#[test]
fn deny_input_floor() {
   unsafe {
      magic::override_reset_state();
      magic::issue_override_code(4);
      magic::issue_override_code(2);
      assert!(magic::poll_override_error() == 3);
   }
}

#[test]
fn deny_manual_mode() {
   unsafe {
      magic::override_reset_state();
      magic::issue_override_code(5);
      assert!(magic::poll_override_error() == 3);
   }
}

#[test]
fn deny_normal_mode() {
   unsafe {
      magic::override_reset_state();
      magic::issue_override_code(6);
      assert!(magic::poll_override_error() == 3);
   }
}
