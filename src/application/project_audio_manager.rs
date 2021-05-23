use rust_engine_3d::utilities::system::{ self, newRcRefCell, RcRefCell };

use crate::application::project_application::ProjectApplication;
use crate::resource::project_resource::ProjectResources;


//
extern crate sdl2;

use sdl2::audio::{AudioCVT, AudioCallback, AudioSpecDesired, AudioSpecWAV};
use std::path::{Path, PathBuf};
use self::sdl2::audio::AudioDevice;

// NOTE: You probably want to investigate the
// mixer feature for real use cases.
pub struct Sound {
    data: Vec<u8>,
    volume: f32,
    pos: usize,
}

impl AudioCallback for Sound {
    type Channel = u8;

    fn callback(&mut self, out: &mut [u8]) {
        for dst in out.iter_mut() {
            let pre_scale = *self.data.get(self.pos).unwrap_or(&128);
            let scaled_signed_float = (pre_scale as f32 - 128.0) * self.volume;
            let scaled = (scaled_signed_float + 128.0) as u8;
            *dst = scaled;
            self.pos += 1;
        }
    }
}


//

pub struct AudioDataCreateInfo {
    pub _audio_name: String,
    pub _audio_source: bool,
}

#[derive(Clone)]
pub struct AudioInstance {
    pub _audio_data: RcRefCell<AudioDataCreateInfo>,
}

pub struct ProjectAudioManager {
    pub _project_application: *const ProjectApplication,
    pub _project_resources: *const ProjectResources,
    pub _audios: Vec<RcRefCell<AudioInstance>>,
    pub _bgm: Option<Box<AudioInstance>>,
    pub _device: Option<AudioDevice<Sound>>
}

impl AudioInstance {
    pub fn create_audio(audio_data: &RcRefCell<AudioDataCreateInfo>) -> RcRefCell<AudioInstance> {
        newRcRefCell(AudioInstance {
            _audio_data: audio_data.clone(),
        })
    }
}

impl ProjectAudioManager {
    pub fn create_audio_manager() -> Box<ProjectAudioManager> {
        Box::new(ProjectAudioManager {
            _project_application: std::ptr::null(),
            _project_resources: std::ptr::null(),
            _audios: Vec::new(),
            _bgm: None,
            _device: None,
        })
    }

    pub fn initialize_audio_manager(&mut self, project_application: *const ProjectApplication, project_resources: *const ProjectResources) {
        self._project_application = project_application;
        self._project_resources = project_resources;
        self.create_audio("game_load");
    }

    pub fn get_project_application(&self) -> &ProjectApplication {
        unsafe { &*self._project_application }
    }

    pub fn get_project_resources(&self) -> &ProjectResources {
        unsafe { &*self._project_resources }
    }

    pub fn create_audio(&mut self, audio_name: &str) -> RcRefCell<AudioInstance> {
        let audio_data = self.get_project_resources().get_audio_data(audio_name);
        let audio_instance = AudioInstance::create_audio(&audio_data);
        self._audios.push(audio_instance.clone());

        //
        let wav_file = Path::new("resource/sounds/game_load.wav");
        let sdl_context = sdl2::init().expect("");
        let audio_subsystem = sdl_context.audio().expect("");
        let loaded_contents = system::load(&wav_file);

        let desired_spec = AudioSpecDesired {
            freq: Some(44_100),
            channels: Some(1), // mono
            samples: None,     // default
        };

        self._device = Some(audio_subsystem.open_playback(None, &desired_spec, |spec| {
            let wav = AudioSpecWAV::load_wav(wav_file).expect("Could not load test WAV file");

            let cvt = AudioCVT::new(
                wav.format,
                wav.channels,
                wav.freq,
                spec.format,
                spec.channels,
                spec.freq,
            ).expect("Could not convert WAV file");

            let data = cvt.convert(wav.buffer().to_vec());

            // initialize the audio callback
            Sound {
                data,
                volume: 0.25,
                pos: 0,
            }
        }).expect(""));

        // Start playback
        self._device.as_ref().unwrap().resume();

        audio_instance
    }

    pub fn update_audio_manager(&mut self) {

    }
}
