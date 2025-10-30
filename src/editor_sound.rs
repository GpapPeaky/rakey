use macroquad::audio::{load_sound, play_sound_once};

pub async fn navigation_alert() {
    let alert_sound = load_sound("assets/sound/alert.wav").await.unwrap();

    play_sound_once(&alert_sound);
}
