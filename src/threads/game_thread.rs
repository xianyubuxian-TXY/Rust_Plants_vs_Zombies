use ggez::{filesystem, Context, GameResult};
use rodio::{Sink, OutputStream, source::Source};
use std::io::Read;
use std::sync::{mpsc, Arc};
use std::path::Path;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref AUDIO_POOL: std::sync::Mutex<HashMap<String, Arc<Vec<u8>>>> = 
        std::sync::Mutex::new(HashMap::new());
}


// 音效类型
pub enum AudioEvent {
    PlayBGM(String, bool), // 背景音乐文件名 + 是否循环
    PlaySFX(String),       // 音效文件名
    StopBGM,
}

pub fn load_audio(ctx: &mut Context, files: &[&str]) -> GameResult<()> {
    let mut pool = AUDIO_POOL.lock().unwrap();

    for file in files {
        let file_path = Path::new(file);

        // 使用 ggez 的 filesystem.open 方法打开文件
        let mut file = filesystem::open(ctx, file_path)?;

        // 使用标准库的 Read trait 来读取文件内容
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        // 使用文件路径作为 key 存储数据到池中
        pool.insert(file_path.to_str().unwrap().to_string(), Arc::new(data));
    }

    Ok(())
}


pub fn audio_thread(receiver: mpsc::Receiver<AudioEvent>) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let bgm_sink = Sink::try_new(&stream_handle).unwrap();
    
    // 设置背景音乐音量（避免盖过音效）
    bgm_sink.set_volume(0.6);
    
    loop {
        match receiver.recv() {
            Ok(AudioEvent::PlayBGM(file, looped)) => {
                if let Some(data) = AUDIO_POOL.lock().unwrap().get(&file) {
                    bgm_sink.stop(); // 停止当前背景音乐
                    
                    let cursor = std::io::Cursor::new(data.as_ref().clone());
                    if let Ok(source) = rodio::Decoder::new(cursor) {
                        if looped {
                            bgm_sink.append(source.repeat_infinite());
                        } else {
                            bgm_sink.append(source);
                        }
                        bgm_sink.play();
                    }
                }
            },
            Ok(AudioEvent::PlaySFX(file)) => {
                if let Some(data) = AUDIO_POOL.lock().unwrap().get(&file) {
                    let cursor = std::io::Cursor::new(data.as_ref().clone());
                    if let Ok(source) = rodio::Decoder::new(cursor) {
                        let sink = Sink::try_new(&stream_handle).unwrap();
                        sink.set_volume(1.0); // 音效全音量
                        sink.append(source);
                        sink.detach();
                    }
                }
            },
            Ok(AudioEvent::StopBGM) => {
                bgm_sink.stop();
            },
            Err(_) => break, // 通道关闭时退出线程
        }
    }
}

