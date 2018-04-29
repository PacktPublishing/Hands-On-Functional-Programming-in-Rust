int input_codes[3] = {0, 0, 0};
int error_code = 0;

int is_override() {
   return 0;
}

int is_privileged() {
   return 0;
}

int is_admin() {
   return 0;
}

void issue_override_code(int code) {
   input_codes[0] = 0;
   input_codes[1] = 0;
   input_codes[2] = 0;
   input_codes[0] = code;
}

int poll_override_code() {
   int code = input_codes[0];
   input_codes[0] = input_codes[1];
   input_codes[1] = input_codes[2];
   input_codes[2] = 0;
   return code;
}

int poll_override_input_floor() {
   //C Code
   return 0;
}

int poll_override_error() {
   return error_code;
}

void* poll_override_session() {
   //C Code
   return 0;
}

void free_override_session(void* session) {
   //C Code
}

void* poll_physical_override_privileged_session() {
   //C Code
   return 0;
}

void* poll_physical_override_admin_session() {
   //C Code
   return 0;
}

void override_input_floor(int floor) {
}

void override_manual_mode() {
   //C Code
}

void override_normal_mode() {
   //C Code
}

void override_reset_state() {
   input_codes[0] = 0;
   input_codes[1] = 0;
   input_codes[2] = 0;
   error_code = 0;
}

void elevator_display_flash(int pattern) {
   //C Code
}

void elevator_display_toggle_light(int light_id) {
   //C Code
}

void elevator_display_set_light_color(int light_id, int color) {
   //C Code
}
