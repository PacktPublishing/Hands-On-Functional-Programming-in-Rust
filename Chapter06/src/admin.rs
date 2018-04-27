use magic;
use libc::c_void;

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

pub struct AuthorizedSession
{
   session: *const c_void
}

pub fn authorize_override() -> Result<AuthorizedSession,ErrorCode> {
   let session = unsafe {
      magic::issue_override_code(OverrideCode::IssueOverride as i32);
      magic::poll_override_session()
   };
   let session = AuthorizedSession {
      session: session
   };
   check_error(session)
}

pub fn authorize_privileged() -> Result<AuthorizedSession,ErrorCode> {
   let session = unsafe {
      magic::issue_override_code(OverrideCode::IssuePrivileged as i32);
      magic::poll_physical_override_privileged_session()
   };
   let session = AuthorizedSession {
      session: session
   };
   check_error(session)
}

pub fn authorize_admin() -> Result<AuthorizedSession,ErrorCode> {
   let session = unsafe {
      magic::issue_override_code(OverrideCode::IssueAdmin as i32);
      magic::poll_physical_override_admin_session()
   };
   let session = AuthorizedSession {
      session: session
   };
   check_error(session)
}

pub fn reset_state()
{
   unsafe {
      magic::override_reset_state();
   }
}

pub fn check_error<T>(t: T) -> Result<T,ErrorCode>
{
   let err = unsafe {
      magic::poll_override_error()
   };
   if err==0 {
      Result::Ok(t)
   } else {
      Result::Err(toErrorCode(err))
   }
}

pub fn input_floor(floor: i32) -> Result<(),ErrorCode>
{
   unsafe {
      magic::override_input_floor(floor);
   }
   check_error(())
}

pub fn manual_mode() -> Result<(),ErrorCode>
{
   unsafe {
      magic::override_manual_mode();
   }
   check_error(())
}

pub fn normal_mode() -> Result<(),ErrorCode>
{
   unsafe {
      magic::override_normal_mode();
   }
   check_error(())
}

pub fn flash(pattern: i32) -> Result<(),ErrorCode>
{
   unsafe {
      magic::elevator_display_flash(pattern);
   }
   check_error(())
}
pub fn toggle_light(light_id: i32) -> Result<(),ErrorCode>
{
   unsafe {
      magic::elevator_display_toggle_light(light_id);
   }
   check_error(())
}
pub fn set_light_color(light_id: i32, color: i32) -> Result<(),ErrorCode>
{
   unsafe {
      magic::elevator_display_set_light_color(light_id, color);
   }
   check_error(())
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

