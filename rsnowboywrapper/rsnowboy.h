#ifndef RSNOWBOY_H
#define RSNOWBOY_H

#include <stdbool.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct RSnowboyDetect RSnowboyDetect;
typedef struct RSnowboyVad RSnowboyVad;

RSnowboyDetect* detect_create(const char* resource_filename, const char* model_str);

bool detect_reset(RSnowboyDetect* detector);

int detect_run_char_detection(RSnowboyDetect* detector, const char* data);

int detect_run_float_array_detection(RSnowboyDetect* detector, const float* const data,
                                     const int array_length, bool is_end);

int detect_run_short_array_detection(RSnowboyDetect* detector, const int16_t* const data,
                                     const int array_length, bool is_end);

int detect_run_integer_array_detection(RSnowboyDetect* detector, const int32_t* const data,
                                       const int array_length, bool is_end);

void detect_set_sensitivity(RSnowboyDetect* detector, const char* sensitivity_str);

const char* detect_get_sensitivity(RSnowboyDetect* detector);

void detect_set_audio_gain(RSnowboyDetect* detector, const float audio_gain);

void detect_update_model(RSnowboyDetect* detector);

int detect_num_hotwords(RSnowboyDetect* detector);

void detect_apply_frontend(RSnowboyDetect* detector, const bool apply_frontend);

int detect_sample_rate(RSnowboyDetect* detector);

int detect_num_channels(RSnowboyDetect* detector);

int detect_bits_per_sample(RSnowboyDetect* detector);

void detect_destroy(RSnowboyDetect* detector);



RSnowboyVad* vad_create(const char* resource_filename);

bool vad_reset(RSnowboyVad *vad);

int vad_run_char(RSnowboyVad* vad, const char* data);

int vad_run_float_array(RSnowboyVad* vad, const float* const data,
                        const int array_length, bool is_end);

int vad_run_short_array(RSnowboyVad* vad, const int16_t* const data,
                        const int array_length, bool is_end);

int vad_run_integer_array(RSnowboyVad* vad, const int32_t* const data,
                          const int array_length, bool is_end);

void vad_set_audio_gain(RSnowboyVad* vad, const float audio_gain);

void vad_apply_frontend(RSnowboyVad* vad, const bool apply_frontend);

int vad_sample_rate(RSnowboyVad* vad);

int vad_num_channels(RSnowboyVad* vad);

int vad_bits_per_sample(RSnowboyVad* vad);

void vad_destroy(RSnowboyVad* vad);

#ifdef __cplusplus
}
#endif

#endif // RSNOWBOY_H
