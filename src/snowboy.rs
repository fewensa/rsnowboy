use std::ffi::{CString, CStr};

use crate::rawrsnoboy::root as rsnowboy;

#[derive(Debug, Copy, Clone)]
pub struct SnowboyDetect {
  rsnowboy_detect: *mut rsnowboy::RSnowboyDetect
}

unsafe impl Send for SnowboyDetect {}
unsafe impl Sync for SnowboyDetect {}

impl SnowboyDetect {
  pub fn new<S>(resource_filename: S, model_str: S) -> Self where S: AsRef<str> {
    unsafe {
      let rsnowboy_detect = rsnowboy::detect_create(
        CString::new(resource_filename.as_ref()).unwrap().as_ptr(), //
        CString::new(model_str.as_ref()).unwrap().as_ptr(),
      );
      Self {
        rsnowboy_detect
      }
    }
  }

  pub fn reset(&self) -> bool {
    unsafe {
      rsnowboy::detect_reset(self.rsnowboy_detect)
    }
  }

  pub fn run_char_detection(&self, data: *const ::std::os::raw::c_char, is_end: bool) -> i32 {
    unsafe {
      rsnowboy::detect_run_char_detection(self.rsnowboy_detect, data)
    }
  }

  pub fn run_float_array_detection(&self, data: *const f32, array_length: ::std::os::raw::c_int, is_end: bool) -> i32 {
    unsafe {
      rsnowboy::detect_run_float_array_detection(self.rsnowboy_detect, data, array_length, is_end)
    }
  }

  pub fn run_short_array_detection(&self, data: *const i16, array_length: ::std::os::raw::c_int, is_end: bool) -> i32 {
    unsafe {
      rsnowboy::detect_run_short_array_detection(self.rsnowboy_detect, data, array_length, is_end)
    }
  }

  pub fn run_integer_array_detection(&self, data: *const i32, array_length: ::std::os::raw::c_int, is_end: bool) -> i32 {
    unsafe {
      rsnowboy::detect_run_integer_array_detection(self.rsnowboy_detect, data, array_length, is_end)
    }
  }

  pub fn set_sensitivity<S>(&self, sensitivity_str: S) where S: AsRef<str> {
    unsafe {
      rsnowboy::detect_set_sensitivity(self.rsnowboy_detect,
                                       CString::new(sensitivity_str.as_ref()).unwrap().as_ptr())
    }
  }

  pub fn get_sensitivity(&self) -> String {
    unsafe {
      let ret = rsnowboy::detect_get_sensitivity(self.rsnowboy_detect);
      let cstr = CStr::from_ptr(ret);
      String::from_utf8_lossy(cstr.to_bytes()).to_string()
    }
  }

  pub fn set_audio_gain(&self, audio_gain: f32) {
    unsafe {
      rsnowboy::detect_set_audio_gain(self.rsnowboy_detect, audio_gain)
    }
  }

  pub fn update_model(&self) {
    unsafe {
      rsnowboy::detect_update_model(self.rsnowboy_detect)
    }
  }

  pub fn num_hotwords(&self) -> i32 {
    unsafe {
      rsnowboy::detect_num_hotwords(self.rsnowboy_detect)
    }
  }

  pub fn apply_frontend(&self, apply_frontend: bool) {
    unsafe {
      rsnowboy::detect_apply_frontend(self.rsnowboy_detect, apply_frontend)
    }
  }

  pub fn sample_rate(&self) -> i32 {
    unsafe {
      rsnowboy::detect_sample_rate(self.rsnowboy_detect)
    }
  }

  pub fn num_channels(&self) -> i32 {
    unsafe {
      rsnowboy::detect_num_channels(self.rsnowboy_detect)
    }
  }

  pub fn bits_per_sample(&self) -> i32 {
    unsafe {
      rsnowboy::detect_bits_per_sample(self.rsnowboy_detect)
    }
  }

  pub fn destroy(&self) {
    unsafe {
      rsnowboy::detect_destroy(self.rsnowboy_detect)
    }
  }

}

#[derive(Debug, Copy, Clone)]
pub struct SnowboyVad {
  rsnowboy_vad: *mut rsnowboy::RSnowboyVad
}

impl SnowboyVad {
  pub fn new<S>(resource_filename: S) -> Self where S: AsRef<str> {
    unsafe {
      let rsnowboy_vad =
        rsnowboy::vad_create(
          CString::new(resource_filename.as_ref()).unwrap().as_ptr());
      Self {
        rsnowboy_vad
      }
    }
  }

  pub fn reset(&self) -> bool {
    unsafe {
      rsnowboy::vad_reset(self.rsnowboy_vad)
    }
  }

  pub fn run_char(&self, data: *const ::std::os::raw::c_char) -> i32 {
    unsafe {
      rsnowboy::vad_run_char(self.rsnowboy_vad, data)
    }
  }

  pub fn run_float_array(&self, data: *const f32, array_length: ::std::os::raw::c_int, is_end: bool) -> i32 {
    unsafe {
      rsnowboy::vad_run_float_array(self.rsnowboy_vad, data, array_length, is_end)
    }
  }

  pub fn run_short_array(&self, data: *const i16, array_length: ::std::os::raw::c_int, is_end: bool) -> i32 {
    unsafe {
      rsnowboy::vad_run_short_array(self.rsnowboy_vad, data, array_length, is_end)
    }
  }

  pub fn run_integer_array(&self, data: *const i32, array_length: ::std::os::raw::c_int, is_end: bool) -> i32 {
    unsafe {
      rsnowboy::vad_run_integer_array(self.rsnowboy_vad, data, array_length, is_end)
    }
  }

  pub fn set_audio_gain(&self, audio_gain: f32) {
    unsafe {
      rsnowboy::vad_set_audio_gain(self.rsnowboy_vad, audio_gain)
    }
  }

  pub fn apply_frontend(&self, apply_frontend: bool) {
    unsafe {
      rsnowboy::vad_apply_frontend(self.rsnowboy_vad, apply_frontend)
    }
  }

  pub fn sample_rate(&self) -> i32 {
    unsafe {
      rsnowboy::vad_sample_rate(self.rsnowboy_vad)
    }
  }

  pub fn num_channels(&self) -> i32 {
    unsafe {
      rsnowboy::vad_num_channels(self.rsnowboy_vad)
    }
  }

  pub fn bits_per_sample(&self) -> i32 {
    unsafe {
      rsnowboy::vad_bits_per_sample(self.rsnowboy_vad)
    }
  }

  pub fn destroy(&self) {
    unsafe {
      rsnowboy::vad_destroy(self.rsnowboy_vad)
    }
  }

}
