//!
//! # rsnowboy
//!
//! The snowboy binding for rust. The more document you can see [snowboy][snowboy] and [kitt.ai][kitt_ai]
//!
//! ## Examples
//!
//! ```no_run
//! use rsnowboy::SnowboyDetect;
//!
//! fn main() {
//!   let resource_filename = "resources/common.res";
//!   let model_filename = "resources/models/snowboy.umdl";
//!   let sensitivity_str = "1";
//!   let audio_gain = 1 as f32;
//!   let apply_frontend = false;
//!
//!   let detector = SnowboyDetect::new(resource_filename, model_filename);
//!
//!   detector.set_sensitivity(sensitivity_str);
//!   detector.set_audio_gain(audio_gain);
//!   detector.apply_frontend(apply_frontend);
//!
//!   let mut rec_count = 0;
//!   let mut sil_count = 0;
//!   let mut save_buffer: Vec<i16> = Vec::new();
//!
//!   // loop get voice
//! //  loop {
//!     let mut voice = voice(); // get voice from your device
//!     let result = detector.run_short_array_detection(voice.as_ptr(), voice.len() as i32, false);
//!
//!     match result {
//!       0 => {
//!         println!("voice");
//!         save_buffer.append(voice.as_mut());
//!         rec_count += 1;
//!         sil_count = 0;
//!       }
//!       1 => {
//!         println!("Hotword {} detected!", result);
//!         // play resources/ding.wav
//!         save_buffer.clear();
//!         rec_count = 1;
//!       }
//!       -2 => {
//!         println!("silence");
//!         save_buffer.append(voice.as_mut());
//!         sil_count += 1;
//!
//!         if rec_count > 0 {
//!           save_buffer.append(voice.as_mut());
//!           sil_count += 1;
//!           println!("silence");
//!         }
//!
//!         if sil_count > 2 && rec_count > 1 {
//!           /*
//!           play resources/dong.wav
//!           There you can write `save_buffer` to wave file, and use any speech to text service convert this voice
//!           file to text, and do any thing.
//!           */
//!           rec_count = 0;
//!           sil_count = 0;
//!           save_buffer.clear();
//!           println!("Listening...");
//!         }
//!       }
//!       _ => {}
//!     }
//!
//! //  }
//!
//! }
//!
//! fn voice() -> Vec<i16> {
//!   vec![
//!     -11, -12, -10, -20, -15, -5, 2, 5, 8,
//!     10, 12, 14, 13, 20, 25, 14, 12, 9, 12,
//!     -1, -5, 20, 22, 24, 24, 14, 10, 15, 11
//!   ]
//! }
//! ```
//!
//! [snowboy]: https://github.com/Kitt-AI/snowboy
//! [kitt_ai]: https://snowboy.kitt.ai
//!

pub use self::snowboy::*;

mod rawrsnoboy;
mod snowboy;

