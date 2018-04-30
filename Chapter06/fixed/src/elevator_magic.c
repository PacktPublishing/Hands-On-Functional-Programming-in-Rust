int input_codes[3] = {0, 0, 0};
int error_code = 0;
void *active_session = 0;
void *poll_session = 0;
int active_light = 0;
int active_light_toggle = 0;

int is_override() {
   return (active_session == ((void*)1));
}

int is_privileged() {
   return (active_session == ((void*)2));
}

int is_admin() {
   return (active_session == ((void*)3));
}

void issue_override_code(int code) {
   input_codes[0] = 0;
   input_codes[1] = 0;
   input_codes[2] = 0;
   input_codes[0] = code;
   if( code == 1 ) {
      if( active_session == ((void*)0) ) {
         active_session = (void*)1;
         poll_session = (void*)1;
      } else {
         active_session = (void*)4;
         poll_session = (void*)0;
         error_code = 1;
      }
   } else if( code == 2 ) {
      if( active_session == ((void*)0) ) {
         active_session = (void*)2;
         poll_session = (void*)2;
      } else {
         active_session = (void*)4;
         poll_session = (void*)0;
         error_code = 1;
      }
   } else if( code == 3 ) {
      if( active_session == ((void*)0) ) {
         active_session = (void*)3;
         poll_session = (void*)3;
      } else {
         active_session = (void*)4;
         poll_session = (void*)0;
         error_code = 1;
      }
   }
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
   return poll_session;
}

void free_override_session(void* session) {
   if( active_session == 0 ) {
      error_code = 2;
   }
   active_session = 0;
}

void* poll_physical_override_privileged_session() {
   return poll_session;
}

void* poll_physical_override_admin_session() {
   return poll_session;
}

void override_input_floor(int floor) {
   if(active_session) {
      input_codes[0] = 4;
      input_codes[1] = floor;
      input_codes[2] = 0;
   } else {
      error_code = 3;
   }
}

void override_manual_mode() {
   if(active_session) {
      input_codes[0] = 5;
      input_codes[1] = 0;
      input_codes[2] = 0;
   } else {
      error_code = 3;
   }
}

void override_normal_mode() {
   if(active_session) {
      input_codes[0] = 6;
      input_codes[1] = 0;
      input_codes[2] = 0;
   } else {
      error_code = 3;
   }
}

void override_reset_state() {
   input_codes[0] = 0;
   input_codes[1] = 0;
   input_codes[2] = 0;
   error_code = 0;
   active_session = 0;
   poll_session = 0;
   active_light = 0;
   active_light_toggle = 0;
}

void elevator_display_flash(int pattern) {
   input_codes[0] = 7;
   input_codes[1] = pattern;
   input_codes[2] = 0;
}

void elevator_display_toggle_light(int light_id) {
   int last_state = 0;
   if( active_light == light_id ) {
      last_state = active_light_toggle;
   }
   active_light = light_id;
   active_light_toggle = last_state ? 0 : 1;
   input_codes[0] = 8;
   input_codes[1] = light_id;
   input_codes[2] = active_light_toggle;
}

void elevator_display_set_light_color(int light_id, int color) {
   input_codes[0] = 9;
   input_codes[1] = light_id;
   input_codes[2] = color;
}
