#include "rsnowboy.h"
#include "snowboy-detect.h"



extern "C" {
RSnowboyDetect* detect_create(const char* resource_filename, const char* model_str) {
    return reinterpret_cast<RSnowboyDetect*>(new snowboy::SnowboyDetect(std::string(resource_filename),
                                                                        std::string(model_str)));
}

bool detect_reset(RSnowboyDetect* detector) {
    return reinterpret_cast<snowboy::SnowboyDetect*>(detector)->Reset();
}

int detect_run_char_detection(RSnowboyDetect* detector, const char* data) {
    return reinterpret_cast<snowboy::SnowboyDetect*>(detector)->RunDetection(std::string(data));
}

int detect_run_float_array_detection(RSnowboyDetect* detector, const float* const data,
                                     const int array_length, bool is_end = false) {
    return reinterpret_cast<snowboy::SnowboyDetect*>(detector)->RunDetection(data, array_length, is_end);
}

int detect_run_short_array_detection(RSnowboyDetect* detector, const int16_t* const data,
                                     const int array_length, bool is_end = false) {
    return reinterpret_cast<snowboy::SnowboyDetect*>(detector)->RunDetection(data, array_length, is_end);
}

int detect_run_integer_array_detection(RSnowboyDetect* detector, const int32_t* const data,
                                       const int array_length, bool is_end = false) {
    return reinterpret_cast<snowboy::SnowboyDetect*>(detector)->RunDetection(data, array_length, is_end);
}

void detect_set_sensitivity(RSnowboyDetect* detector, const char* sensitivity_str) {
    reinterpret_cast<snowboy::SnowboyDetect*>(detector)->SetSensitivity(std::string(sensitivity_str));
}

const char* detect_get_sensitivity(RSnowboyDetect* detector) {
    return reinterpret_cast<snowboy::SnowboyDetect*>(detector)->GetSensitivity().c_str();
}

void detect_set_audio_gain(RSnowboyDetect* detector, const float audio_gain) {
    reinterpret_cast<snowboy::SnowboyDetect*>(detector)->SetAudioGain(audio_gain);
}

void detect_update_model(RSnowboyDetect* detector) {
    reinterpret_cast<snowboy::SnowboyDetect*>(detector)->UpdateModel();
}

int detect_num_hotwords(RSnowboyDetect* detector) {
    return reinterpret_cast<snowboy::SnowboyDetect*>(detector)->NumHotwords();
}

void detect_apply_frontend(RSnowboyDetect* detector, const bool apply_frontend) {
    reinterpret_cast<snowboy::SnowboyDetect*>(detector)->ApplyFrontend(apply_frontend);
}

int detect_sample_rate(RSnowboyDetect* detector) {
    return reinterpret_cast<snowboy::SnowboyDetect*>(detector)->SampleRate();
}

int detect_num_channels(RSnowboyDetect* detector) {
    return reinterpret_cast<snowboy::SnowboyDetect*>(detector)->NumChannels();
}

int detect_bits_per_sample(RSnowboyDetect* detector) {
    return reinterpret_cast<snowboy::SnowboyDetect*>(detector)->BitsPerSample();
}

void detect_destroy(RSnowboyDetect* detector) {
    delete reinterpret_cast<snowboy::SnowboyDetect*>(detector);
}



RSnowboyVad *vad_create(const char *resource_filename) {
    return reinterpret_cast<RSnowboyVad*>(new snowboy::SnowboyVad(std::string(resource_filename)));
}

bool vad_reset(RSnowboyVad *vad){
    return reinterpret_cast<snowboy::SnowboyVad*>(vad)->Reset();
}


int vad_run_char(RSnowboyVad *vad, const char* data) {
    return reinterpret_cast<snowboy::SnowboyVad*>(vad)->RunVad(data);
}

int vad_run_float_array(RSnowboyVad *vad, const float * const data, const int array_length, bool is_end) {
    return reinterpret_cast<snowboy::SnowboyVad*>(vad)->RunVad(data, array_length, is_end);
}

int vad_run_short_array(RSnowboyVad *vad, const int16_t * const data, const int array_length, bool is_end) {
    return reinterpret_cast<snowboy::SnowboyVad*>(vad)->RunVad(data, array_length, is_end);
}

int vad_run_integer_array(RSnowboyVad *vad, const int32_t * const data, const int array_length, bool is_end) {
    return reinterpret_cast<snowboy::SnowboyVad*>(vad)->RunVad(data, array_length, is_end);
}

void vad_set_audio_gain(RSnowboyVad *vad, const float audio_gain) {
    reinterpret_cast<snowboy::SnowboyVad*>(vad)->SetAudioGain(audio_gain);
}

void vad_apply_frontend(RSnowboyVad *vad, const bool apply_frontend) {
    reinterpret_cast<snowboy::SnowboyVad*>(vad)->ApplyFrontend(apply_frontend);
}

int vad_sample_rate(RSnowboyVad *vad) {
    return reinterpret_cast<snowboy::SnowboyVad*>(vad)->SampleRate();
}

int vad_num_channels(RSnowboyVad *vad) {
    return reinterpret_cast<snowboy::SnowboyVad*>(vad)->NumChannels();
}

int vad_bits_per_sample(RSnowboyVad *vad) {
    return reinterpret_cast<snowboy::SnowboyVad*>(vad)->BitsPerSample();
}

void vad_destroy(RSnowboyVad *vad) {
    delete reinterpret_cast<snowboy::SnowboyVad*>(vad);
}


}
