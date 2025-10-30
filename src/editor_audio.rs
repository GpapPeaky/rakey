// Editor audio module

use macroquad::audio::{Sound, load_sound, play_sound_once};

pub struct EditorAudio {
    pub insert: Sound,
    pub delete: Sound,
    pub space: Sound,
    pub enter: Sound
}

impl EditorAudio {
    pub async fn load() -> Self {
        let editor_audio =  EditorAudio {
            insert: load_sound("assets/sound/insert.wav").await.unwrap(),
            delete: load_sound("assets/sound/del.wav").await.unwrap(),
            space: load_sound("assets/sound/space.wav").await.unwrap(),
            enter: load_sound("assets/sound/return.wav").await.unwrap()
        };

        editor_audio
    }

    pub fn play_insert(&self) {
        play_sound_once(&self.insert);
    }

    pub fn play_delete(&self) {
        play_sound_once(&self.delete);
    }

    pub fn play_space(&self) {
        play_sound_once(&self.space);
    }

    pub fn play_return(&self) {
        play_sound_once(&self.enter);
    }
}