
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
   //C Code
}

int poll_override_code() {
   //C Code
   return 0;
}

int poll_override_input_floor() {
   //C Code
   return 0;
}

int poll_override_error() {
   //C Code
   return 0;
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
   //C Code
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
