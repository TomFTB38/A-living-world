//! Sortie audio : le mélangeur vit dans alw-core, ici on le branche sur la carte son.
//! Fil dédié : la carte son a ses propres règles de thread sous Windows. Sans périphérique, le jeu tourne en silence.
use alw_core::audio::{Bank, Cmd, Mixer};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc::{channel, Sender};
use std::time::Duration;
fn fill<T: Copy + Default>(m: &mut Mixer, buf: &mut Vec<f32>, o: &mut [T], ch: usize, conv: impl Fn(f32) -> T) {
    let frames = o.len() / ch.max(1); buf.resize(frames * 2, 0.0); m.render(buf);
    for (f, out) in o.chunks_mut(ch.max(1)).enumerate() {
        let (l, r) = (buf[f * 2], buf[f * 2 + 1]);
        if ch == 1 { out[0] = conv((l + r) * 0.5); } else { out[0] = conv(l); out[1] = conv(r); for x in &mut out[2..] { *x = T::default(); } }
    }
}
pub fn start() -> Option<Sender<Cmd>> {
    let bank = Bank::parse(include_bytes!("../assets/bank.bin"))?; let (ok, got) = channel();
    std::thread::Builder::new().name("audio".into()).spawn(move || {
        let host = cpal::default_host(); let Some(dev) = host.default_output_device() else { eprintln!("Aucune sortie audio"); return };
        let Ok(cfg) = dev.default_output_config() else { eprintln!("Sortie audio illisible"); return };
        let (sr, ch, fmt) = (cfg.sample_rate().0 as f32, cfg.channels() as usize, cfg.sample_format()); let config: cpal::StreamConfig = cfg.into();
        let (mut mixer, tx) = Mixer::new(sr, bank); let mut buf = Vec::new(); let err = |e: cpal::StreamError| eprintln!("Audio : {e}");
        let stream = match fmt {
            cpal::SampleFormat::F32 => dev.build_output_stream(&config, move |o: &mut [f32], _: &cpal::OutputCallbackInfo| fill(&mut mixer, &mut buf, o, ch, |v| v), err, None),
            cpal::SampleFormat::I16 => dev.build_output_stream(&config, move |o: &mut [i16], _: &cpal::OutputCallbackInfo| fill(&mut mixer, &mut buf, o, ch, |v| (v.clamp(-1.0, 1.0) * 32767.0) as i16), err, None),
            cpal::SampleFormat::U16 => dev.build_output_stream(&config, move |o: &mut [u16], _: &cpal::OutputCallbackInfo| fill(&mut mixer, &mut buf, o, ch, |v| ((v.clamp(-1.0, 1.0) * 0.5 + 0.5) * 65535.0) as u16), err, None),
            f => { eprintln!("Format audio non géré : {f:?}"); return }
        };
        let Ok(stream) = stream else { eprintln!("Ouverture de la sortie audio impossible"); return };
        if stream.play().is_err() { eprintln!("Lecture audio impossible"); return; }
        let _ = ok.send(tx);
        loop { std::thread::park(); } // le flux vit tant que ce fil existe
    }).ok()?;
    got.recv_timeout(Duration::from_secs(3)).ok()
}
