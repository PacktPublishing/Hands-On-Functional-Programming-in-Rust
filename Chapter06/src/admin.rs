use magic;

pub enum OverrideCode {
   IssueOverride = 1,
   IssuePrivileged = 2,
   IssueAdmin = 3,
   IssueInputFloor = 4,
   IssueManualMode = 5,
   IssueNormalMode = 6,
   IssueFlash = 7,
   IssueToggleLight = 8,
   IssueSetLightColor = 9,
}
pub fn toOverrideCode(i: i32) -> OverrideCode {
   match i {
      1 => OverrideCode::IssueOverride,
      2 => OverrideCode::IssuePrivileged,
      3 => OverrideCode::IssueAdmin,
      4 => OverrideCode::IssueInputFloor,
      5 => OverrideCode::IssueManualMode,
      6 => OverrideCode::IssueNormalMode,
      7 => OverrideCode::IssueFlash,
      8 => OverrideCode::IssueToggleLight,
      9 => OverrideCode::IssueSetLightColor,
      _ => panic!("Unexpected override code: {}", i)
   }
}

pub enum ErrorCode {
   DoubleAuthorize = 1,
   DoubleFree = 2,
   AccessDenied = 3,
}
pub fn toErrorCode(i: i32) -> ErrorCode {
   match i {
      1 => ErrorCode::DoubleAuthorize,
      2 => ErrorCode::DoubleFree,
      3 => ErrorCode::AccessDenied,
      _ => panic!("Unexpected error code: {}", i)
   }
}

pub fn reset_state()
{
   unsafe {
      magic::override_reset_state();
   }
}

pub fn check_error() -> Result<(),ErrorCode>
{
   let err = unsafe {
      magic::poll_override_error()
   };
   if err==0 {
      Result::Ok(())
   } else {
      Result::Err(toErrorCode(err))
   }
}

pub fn input_floor(floor: i32) -> Result<(),ErrorCode>
{
   unsafe {
      magic::override_input_floor(floor);
   }
   check_error()
}

pub fn manual_mode() -> Result<(),ErrorCode>
{
   unsafe {
      magic::override_manual_mode();
   }
   check_error()
}

pub fn normal_mode() -> Result<(),ErrorCode>
{
   unsafe {
      magic::override_normal_mode();
   }
   check_error()
}

pub fn flash(pattern: i32) -> Result<(),ErrorCode>
{
   unsafe {
      magic::elevator_display_flash(pattern);
   }
   check_error()
}
pub fn toggle_light(light_id: i32) -> Result<(),ErrorCode>
{
   unsafe {
      magic::elevator_display_toggle_light(light_id);
   }
   check_error()
}
pub fn set_light_color(light_id: i32, color: i32) -> Result<(),ErrorCode>
{
   unsafe {
      magic::elevator_display_set_light_color(light_id, color);
   }
   check_error()
}

pub fn is_override() -> bool
{
   unsafe {
      magic::is_override() != 0
   }
}

pub fn is_privileged() -> bool
{
   unsafe {
      magic::is_privileged() != 0
   }
}

pub fn is_admin() -> bool
{
   unsafe {
      magic::is_admin() != 0
   }
}

/*
extern {
   pub fn issue_override_code(code: c_int);
   pub fn poll_override_code() -> c_int;
   pub fn poll_override_input_floor() -> c_int;
   pub fn poll_override_error() -> c_int;
   pub fn poll_override_session() -> *const c_void;
   pub fn free_override_session(session: *const c_void);
   pub fn poll_physical_override_privileged_session() -> *const c_void;
   pub fn poll_physical_override_admin_session() -> *const c_void;
}
*/
