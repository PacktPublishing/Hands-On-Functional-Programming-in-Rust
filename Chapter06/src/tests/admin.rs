use admin;

#[test]
fn authorize_override() {
   admin::reset_state();
   {
      let session = admin::authorize_override().ok();
      assert!(admin::is_override());
   }
   assert!(!admin::is_override());
   assert!(admin::check_error(()).is_ok());
}
#[test]
fn authorize_privileged() {
   admin::reset_state();
   {
      let session = admin::authorize_privileged().ok();
      assert!(admin::is_privileged());
   }
   assert!(!admin::is_privileged());
   assert!(admin::check_error(()).is_ok());
}
#[test]
fn issue_admin_code() {
   admin::reset_state();
   {
      let session = admin::authorize_admin().ok();
      assert!(admin::is_admin());
   }
   assert!(!admin::is_admin());
   assert!(admin::check_error(()).is_ok());
}

#[test]
fn double_override_failure() {
   admin::reset_state();
   let session = admin::authorize_override().ok();
   assert!(admin::authorize_override().err().is_some());
   assert!(!admin::check_error(()).is_ok());
}
#[test]
fn double_privileged_failure() {
   admin::reset_state();
   let session = admin::authorize_privileged().ok();
   assert!(admin::authorize_privileged().err().is_some());
   assert!(!admin::check_error(()).is_ok());
}
#[test]
fn double_admin_failure() {
   admin::reset_state();
   let session = admin::authorize_admin().ok();
   assert!(admin::authorize_admin().err().is_some());
   assert!(!admin::check_error(()).is_ok());
}

#[test]
fn clone_override() {
   admin::reset_state();
   {
      let session = admin::authorize_override().ok().unwrap();
      let session2 = session.clone();
      assert!(admin::is_override());
   }
   assert!(!admin::is_override());
   assert!(admin::check_error(()).is_ok());
}
#[test]
fn clone_privileged() {
   admin::reset_state();
   {
      let session = admin::authorize_privileged().ok().unwrap();
      let session2 = session.clone();
      assert!(admin::is_privileged());
   }
   assert!(!admin::is_privileged());
   assert!(admin::check_error(()).is_ok());
}
#[test]
fn clone_admin() {
   admin::reset_state();
   {
      let session = admin::authorize_admin().ok().unwrap();
      let session2 = session.clone();
      assert!(admin::is_admin());
   }
   assert!(!admin::is_admin());
   assert!(admin::check_error(()).is_ok());
}

#[test]
fn input_floor() {
   admin::reset_state();
   {
      let session = admin::authorize_admin().ok();
      admin::input_floor(2).ok();
   }
   assert!(!admin::is_admin());
   assert!(admin::check_error(()).is_ok());
}

#[test]
fn manual_mode() {
   admin::reset_state();
   {
      let session = admin::authorize_admin().ok();
      admin::manual_mode().ok();
   }
   assert!(!admin::is_admin());
   assert!(admin::check_error(()).is_ok());
}

#[test]
fn normal_mode() {
   admin::reset_state();
   {
      let session = admin::authorize_admin().ok();
      admin::normal_mode().ok();
   }
   assert!(!admin::is_admin());
   assert!(admin::check_error(()).is_ok());
}

#[test]
fn flash() {
   admin::reset_state();
   assert!(!admin::is_override());
   assert!(!admin::is_privileged());
   assert!(!admin::is_admin());
   admin::flash(222).ok();
   assert!(admin::check_error(()).is_ok());
}

#[test]
fn toggle_light() {
   admin::reset_state();
   assert!(!admin::is_override());
   assert!(!admin::is_privileged());
   assert!(!admin::is_admin());
   admin::toggle_light(7).ok();
   assert!(admin::check_error(()).is_ok());
}

#[test]
fn set_light_color() {
   admin::reset_state();
   assert!(!admin::is_override());
   assert!(!admin::is_privileged());
   assert!(!admin::is_admin());
   admin::set_light_color(33, 123).ok();
   assert!(admin::check_error(()).is_ok());
}

#[test]
fn deny_input_floor() {
   admin::reset_state();
   admin::input_floor(2).err();
   assert!(!admin::check_error(()).is_ok());
}

#[test]
fn deny_manual_mode() {
   admin::reset_state();
   admin::manual_mode().err();
   assert!(!admin::check_error(()).is_ok());
}

#[test]
fn deny_normal_mode() {
   admin::reset_state();
   admin::normal_mode().err();
   assert!(!admin::check_error(()).is_ok());
}
