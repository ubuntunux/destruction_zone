use std::path::Path;

use sdl2::{ self, Sdl, AudioSubsystem };
use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS, Sdl2MixerContext, Chunk, Channel};

use rust_engine_3d::utilities::system::{ self, newRcRefCell, RcRefCell };

use crate::application::project_application::ProjectApplication;
use crate::resource::project_resource::ProjectResources;


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
    pub _audio: AudioSubsystem,
    pub _mixer_context: Sdl2MixerContext,
    pub _sound_chunk: Option<Chunk>,
    pub _channel: Option<Channel>,
}

impl AudioInstance {
    pub fn create_audio(audio_data: &RcRefCell<AudioDataCreateInfo>) -> RcRefCell<AudioInstance> {
        newRcRefCell(AudioInstance {
            _audio_data: audio_data.clone(),
        })
    }
}

impl ProjectAudioManager {
    pub fn create_audio_manager(sdl: &Sdl) -> Box<ProjectAudioManager> {
        log::info!("create_audio_manager");
        let audio = sdl.audio().expect("failed to sdl.audio");
        let frequency = 44_100;
        let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
        let channels = DEFAULT_CHANNELS; // Stereo
        let chunk_size = 1_024;
        let _result = sdl2::mixer::open_audio(frequency, format, channels, chunk_size);
        let mixer_context = sdl2::mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG).expect("sdl2::mixer::init");

        log::info!("\tsdl2::mixer::linked version: {}", sdl2::mixer::get_linked_version());
        sdl2::mixer::allocate_channels(4);

        let n = sdl2::mixer::get_chunk_decoders_number();
        log::info!("\tavailable chunk(sample) decoders: {}", n);
        for i in 0..n {
            log::info!("\t\tdecoder {} => {}", i, sdl2::mixer::get_chunk_decoder(i));
        }
        let n = sdl2::mixer::get_music_decoders_number();
        log::info!("\tavailable music decoders: {}", n);
        for i in 0..n {
            log::info!("\t\tdecoder {} => {}", i, sdl2::mixer::get_music_decoder(i));
        }
        log::info!("\tquery spec => {:?}", sdl2::mixer::query_spec());

        Box::new(ProjectAudioManager {
            _project_application: std::ptr::null(),
            _project_resources: std::ptr::null(),
            _audios: Vec::new(),
            _bgm: None,
            _audio: audio,
            _mixer_context: mixer_context,
            _sound_chunk: None,
            _channel: None,
        })
    }

    pub fn initialize_audio_manager(&mut self, project_application: *const ProjectApplication, project_resources: *const ProjectResources) {
        self._project_application = project_application;
        self._project_resources = project_resources;
        self.create_audio("game_load");
    }

    pub fn destroy_audio_manager(&mut self) {
        sdl2::mixer::Music::halt();
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
        self._sound_chunk = Some(sdl2::mixer::Chunk::from_file(wav_file).unwrap());
        println!("chunk volume => {:?}", self._sound_chunk.as_ref().unwrap().get_volume());
        self._channel = Some(sdl2::mixer::Channel::all().play(&self._sound_chunk.as_ref().unwrap(), 1).expect("failed to play"));
        //

        audio_instance
    }

    pub fn update_audio_manager(&mut self) {

    }
}
