use std::collections::HashMap;
use std::path::Path;

use sdl2::{ self, Sdl, AudioSubsystem };
use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS, Sdl2MixerContext, Chunk, Channel};

use rust_engine_3d::utilities::system::{ self, newRcRefCell, RcRefCell };

use crate::application::project_application::ProjectApplication;
use crate::resource::project_resource::ProjectResources;

pub enum AudioLoop {
    ONCE,
    SOME(i32),
    LOOP,
}

pub struct AudioData {
    pub _audio_name: String,
    pub _sound_chunk: Chunk,
}

#[derive(Clone)]
pub struct AudioInstance {
    pub _audio_data: RcRefCell<AudioData>,
    pub _channel: Result<Channel, String>,
}

pub struct ProjectAudioManager {
    pub _project_application: *const ProjectApplication,
    pub _project_resources: *const ProjectResources,
    pub _audios: HashMap<i32, RcRefCell<AudioInstance>>,
    pub _bgm: Option<Box<AudioInstance>>,
    pub _audio: AudioSubsystem,
    pub _mixer_context: Sdl2MixerContext,
}

impl AudioInstance {
    pub fn create_audio(audio_data: &RcRefCell<AudioData>, audio_loop: AudioLoop) -> RcRefCell<AudioInstance> {
        let audio_loop = match audio_loop {
            AudioLoop::ONCE => 0,
            AudioLoop::SOME(x) => 0.max(x - 1),
            AudioLoop::LOOP => -1,
        };

        newRcRefCell(AudioInstance {
            _audio_data: audio_data.clone(),
            _channel: sdl2::mixer::Channel::all().play(&audio_data.borrow()._sound_chunk, audio_loop),
        })
    }
}

impl ProjectAudioManager {
    const MAX_CHANNEL_COUNT: i32 = 128;

    pub fn create_audio_manager(sdl: &Sdl) -> Box<ProjectAudioManager> {
        log::info!("create_audio_manager");
        let audio = sdl.audio().expect("failed to sdl.audio");
        let frequency = 44_100;
        let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
        let channels = DEFAULT_CHANNELS; // Stereo
        let chunk_size = 1_024;
        let _result = sdl2::mixer::open_audio(frequency, format, channels, chunk_size);
        let mixer_context = sdl2::mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG).expect("sdl2::mixer::init");
        let _channel_count = sdl2::mixer::allocate_channels(ProjectAudioManager::MAX_CHANNEL_COUNT);
        // audio debug info
        {
            log::debug!("\tsdl2::mixer::linked version: {}", sdl2::mixer::get_linked_version());
            let n = sdl2::mixer::get_chunk_decoders_number();
            log::debug!("\tavailable chunk(sample) decoders: {}", n);
            for i in 0..n {
                log::debug!("\t\tdecoder {} => {}", i, sdl2::mixer::get_chunk_decoder(i));
            }
            let n = sdl2::mixer::get_music_decoders_number();
            log::debug!("\tavailable music decoders: {}", n);
            for i in 0..n {
                log::debug!("\t\tdecoder {} => {}", i, sdl2::mixer::get_music_decoder(i));
            }
            log::debug!("\tquery spec => {:?}", sdl2::mixer::query_spec());
        }

        Box::new(ProjectAudioManager {
            _project_application: std::ptr::null(),
            _project_resources: std::ptr::null(),
            _audios: HashMap::new(),
            _bgm: None,
            _audio: audio,
            _mixer_context: mixer_context,
        })
    }

    pub fn initialize_audio_manager(&mut self, project_application: *const ProjectApplication, project_resources: *const ProjectResources) {
        self._project_application = project_application;
        self._project_resources = project_resources;
        self.create_audio("game_load", AudioLoop::LOOP);
    }

    pub fn destroy_audio_manager(&mut self) {
        sdl2::mixer::Music::halt();
        for (_key, audio) in self._audios.iter() {
            let channel = &audio.borrow()._channel;
            if channel.is_ok() {
                channel.as_ref().unwrap().halt();
            }
        }
        self._audios.clear();
    }

    pub fn get_project_application(&self) -> &ProjectApplication {
        unsafe { &*self._project_application }
    }

    pub fn get_project_resources(&self) -> &ProjectResources {
        unsafe { &*self._project_resources }
    }

    pub fn create_audio(&mut self, audio_name: &str, audio_loop: AudioLoop) -> RcRefCell<AudioInstance> {
        let audio_data = self.get_project_resources().get_audio_data(audio_name);
        let audio = AudioInstance::create_audio(&audio_data, audio_loop);
        match audio.borrow()._channel {
            Ok(Channel(channel)) => self._audios.insert(channel, audio.clone()),
            _ => None
        };
        audio
    }

    pub fn update_audio_manager(&mut self) {
        let mut remove_audios: Vec<i32> = Vec::new();
        for (key, audio) in self._audios.iter() {
            let channel = &audio.borrow()._channel;
            if channel.is_ok() {
                if false == channel.as_ref().unwrap().is_playing() {
                    remove_audios.push(*key);
                }
            }
        }

        for key in remove_audios.iter() {
            self._audios.remove(key);
        }
    }
}
