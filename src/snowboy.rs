use std::ffi::{CString, CStr};

use crate::rawrsnoboy::root as rsnowboy;

/// SnowboyDetect class interface.
#[derive(Debug, Copy, Clone)]
pub struct SnowboyDetect {
  rsnowboy_detect: *mut rsnowboy::RSnowboyDetect
}

unsafe impl Send for SnowboyDetect {}

unsafe impl Sync for SnowboyDetect {}

impl SnowboyDetect {
  /// Constructor that takes a resource file, and a list of hotword models which
  /// are separated by comma. In the case that more than one hotword exist in the
  /// provided models, RunDetection() will return the index of the hotword, if
  /// the corresponding hotword is triggered.
  ///
  /// CAVEAT: a personal model only contain one hotword, but an universal model
  ///         may contain multiple hotwords. It is your responsibility to figure
  ///         out the index of the hotword. For example, if your model string is
  ///         "foo.pmdl,bar.umdl", where foo.pmdl contains hotword x, bar.umdl
  ///         has two hotwords y and z, the indices of different hotwords are as
  ///         follows:
  ///         x 1
  ///         y 2
  ///         z 3
  ///
  /// @param [in]  resource_filename   Filename of resource file.
  /// @param [in]  model_str           A string of multiple hotword models,
  ///                                  separated by comma.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// # use rsnowboy::SnowboyDetect;
  ///
  /// let _ = SnowboyDetect::new("resources/common.res", "resources/models/snowboy.umdl");
  /// ```
  ///
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

  /// Resets the detection. This class handles voice activity detection (VAD)
  /// internally. But if you have an external VAD, you should call Reset()
  /// whenever you see segment end from your VAD.
  ///
  /// ```no_run
  /// # use rsnowboy::SnowboyDetect;
  ///
  /// let detector = SnowboyDetect::new("resources/common.res", "resources/models/snowboy.umdl");
  /// detector.reset();
  /// ```
  ///
  pub fn reset(&self) -> bool {
    unsafe {
      rsnowboy::detect_reset(self.rsnowboy_detect)
    }
  }

  /// Runs hotword detection. Supported audio format is WAVE (with linear PCM,
  /// 8-bits unsigned integer, 16-bits signed integer or 32-bits signed integer).
  /// See SampleRate(), NumChannels() and BitsPerSample() for the required
  /// sampling rate, number of channels and bits per sample values. You are
  /// supposed to provide a small chunk of data (e.g., 0.1 second) each time you
  /// call RunDetection(). Larger chunk usually leads to longer delay, but less
  /// CPU usage.
  ///
  /// Definition of return values:
  /// -2: Silence.
  /// -1: Error.
  ///  0: No event.
  ///  1: Hotword 1 triggered.
  ///  2: Hotword 2 triggered.
  ///  ...
  ///
  /// @param [in]  data               Small chunk of data to be detected. See
  ///                                 above for the supported data format.
  /// @param [in]  is_end             Set it to true if it is the end of a
  ///                                 utterance or file.
  ///
  ///
  pub fn run_char_detection(&self, data: *const ::std::os::raw::c_char, is_end: bool) -> i32 {
    unsafe {
      rsnowboy::detect_run_char_detection(self.rsnowboy_detect, data)
    }
  }

  /// Various versions of RunDetection() that take different format of audio. If
  /// NumChannels() > 1, e.g., NumChannels() == 2, then the array is as follows:
  ///
  ///   d1c1, d1c2, d2c1, d2c2, d3c1, d3c2, ..., dNc1, dNc2
  ///
  /// where d1c1 means data point 1 of channel 1.
  ///
  /// @param [in]  data               Small chunk of data to be detected. See
  ///                                 above for the supported data format.
  /// @param [in]  array_length       Length of the data array.
  /// @param [in]  is_end             Set it to true if it is the end of a
  ///                                 utterance or file.
  ///
  /// # Examples
  ///
  /// # Examples
  ///
  /// ```no_run
  /// # use rsnowboy::SnowboyDetect;
  ///
  /// let detector = SnowboyDetect::new("resources/common.res", "resources/models/snowboy.umdl");
  /// detector.set_sensitivity("1");
  /// detector.set_audio_gain(1 as f32);
  /// detector.apply_frontend(false);
  /// let voice = vec![
  ///   -11, -12, -10, -20, -15, -5, 2, 5, 8,
  ///   10, 12, 14, 13, 20, 25, 14, 12, 9, 12,
  ///   -1, -5, 20, 22, 24, 24, 14, 10, 15, 11
  /// ];
  /// detector.run_short_array_detection(voice.as_ptr(), voice.len() as i32, false);
  /// ```
  ///
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

  /// Sets the sensitivity string for the loaded hotwords. A <sensitivity_str> is
  /// a list of floating numbers between 0 and 1, and separated by comma. For
  /// example, if there are 3 loaded hotwords, your string should looks something
  /// like this:
  ///   0.4,0.5,0.8
  /// Make sure you properly align the sensitivity value to the corresponding
  /// hotword.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// # use rsnowboy::SnowboyDetect;
  ///
  /// let detector = SnowboyDetect::new("resources/common.res", "resources/models/snowboy.umdl");
  /// detector.set_sensitivity("1");
  /// ```
  pub fn set_sensitivity<S>(&self, sensitivity_str: S) where S: AsRef<str> {
    unsafe {
      rsnowboy::detect_set_sensitivity(self.rsnowboy_detect,
                                       CString::new(sensitivity_str.as_ref()).unwrap().as_ptr())
    }
  }

  /// Returns the sensitivity string for the current hotwords.
  pub fn get_sensitivity(&self) -> String {
    unsafe {
      let ret = rsnowboy::detect_get_sensitivity(self.rsnowboy_detect);
      let cstr = CStr::from_ptr(ret);
      String::from_utf8_lossy(cstr.to_bytes()).to_string()
    }
  }

  /// Applied a fixed gain to the input audio. In case you have a very weak
  /// microphone, you can use this function to boost input audio level.
  /// # Examples
  ///
  /// ```no_run
  /// # use rsnowboy::SnowboyDetect;
  ///
  /// let detector = SnowboyDetect::new("resources/common.res", "resources/models/snowboy.umdl");
  /// detector.set_audio_gain(1 as f32);
  /// ```
  pub fn set_audio_gain(&self, audio_gain: f32) {
    unsafe {
      rsnowboy::detect_set_audio_gain(self.rsnowboy_detect, audio_gain)
    }
  }

  /// Writes the models to the model filenames specified in <model_str> in the
  /// constructor. This overwrites the original model with the latest parameter
  /// setting. You are supposed to call this function if you have updated the
  /// hotword sensitivities through SetSensitivity(), and you would like to store
  /// those values in the model as the default value.
  pub fn update_model(&self) {
    unsafe {
      rsnowboy::detect_update_model(self.rsnowboy_detect)
    }
  }

  /// Returns the number of the loaded hotwords. This helps you to figure the
  /// index of the hotwords.
  pub fn num_hotwords(&self) -> i32 {
    unsafe {
      rsnowboy::detect_num_hotwords(self.rsnowboy_detect)
    }
  }

  /// If <apply_frontend> is true, then apply frontend audio processing;
  /// otherwise turns the audio processing off. Frontend audio processing
  /// includes algorithms such as automatic gain control (AGC), noise suppression
  /// (NS) and so on. Generally adding frontend audio processing helps the
  /// performance, but if the model is not trained with frontend audio
  /// processing, it may decrease the performance. The general rule of thumb is:
  ///   1. For personal models, set it to false.
  ///   2. For universal models, follow the instruction of each published model
  pub fn apply_frontend(&self, apply_frontend: bool) {
    unsafe {
      rsnowboy::detect_apply_frontend(self.rsnowboy_detect, apply_frontend)
    }
  }

  /// Returns the required sampling rate, number of channels and bits per sample
  /// values for the audio data. You should use this information to set up your
  /// audio capturing interface.
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

/// SnowboyVad class interface.
#[derive(Debug, Copy, Clone)]
pub struct SnowboyVad {
  rsnowboy_vad: *mut rsnowboy::RSnowboyVad
}

impl SnowboyVad {
  /// Constructor that takes a resource file. It shares the same resource file
  /// with SnowboyDetect.
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

  /// Resets the VAD.
  pub fn reset(&self) -> bool {
    unsafe {
      rsnowboy::vad_reset(self.rsnowboy_vad)
    }
  }

  /// Runs the VAD algorithm. Supported audio format is WAVE (with linear PCM,
  /// 8-bits unsigned integer, 16-bits signed integer or 32-bits signed integer).
  /// See SampleRate(), NumChannels() and BitsPerSample() for the required
  /// sampling rate, number of channels and bits per sample values. You are
  /// supposed to provide a small chunk of data (e.g., 0.1 second) each time you
  /// call RunDetection(). Larger chunk usually leads to longer delay, but less
  /// CPU usage.
  ///
  /// Definition of return values:
  /// -2: Silence.
  /// -1: Error.
  ///  0: Non-silence.
  ///
  /// @param [in]  data               Small chunk of data to be detected. See
  ///                                 above for the supported data format.
  /// @param [in]  is_end             Set it to true if it is the end of a
  ///                                 utterance or file.
  pub fn run_char(&self, data: *const ::std::os::raw::c_char) -> i32 {
    unsafe {
      rsnowboy::vad_run_char(self.rsnowboy_vad, data)
    }
  }

  /// Various versions of RunVad() that take different format of audio. If
  /// NumChannels() > 1, e.g., NumChannels() == 2, then the array is as follows:
  ///
  ///   d1c1, d1c2, d2c1, d2c2, d3c1, d3c2, ..., dNc1, dNc2
  ///
  /// where d1c1 means data point 1 of channel 1.
  ///
  /// @param [in]  data               Small chunk of data to be detected. See
  ///                                 above for the supported data format.
  /// @param [in]  array_length       Length of the data array.
  /// @param [in]  is_end             Set it to true if it is the end of a
  ///                                 utterance or file.
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

  /// Applied a fixed gain to the input audio. In case you have a very weak
  /// microphone, you can use this function to boost input audio level.
  pub fn set_audio_gain(&self, audio_gain: f32) {
    unsafe {
      rsnowboy::vad_set_audio_gain(self.rsnowboy_vad, audio_gain)
    }
  }

  /// If <apply_frontend> is true, then apply frontend audio processing;
  /// otherwise turns the audio processing off.
  pub fn apply_frontend(&self, apply_frontend: bool) {
    unsafe {
      rsnowboy::vad_apply_frontend(self.rsnowboy_vad, apply_frontend)
    }
  }

  /// Returns the required sampling rate, number of channels and bits per sample
  /// values for the audio data. You should use this information to set up your
  /// audio capturing interface.
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
