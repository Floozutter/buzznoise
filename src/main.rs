mod loudnessbuffer;
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use buttplug::{
    client::{
        ButtplugClient, ButtplugClientEvent, ButtplugClientDeviceMessageType, 
        VibrateCommand,
    },
    server::ButtplugServerOptions,
};
use tokio::io::{self, AsyncBufReadExt, BufReader};
use futures::{StreamExt, Stream};
use futures_timer::Delay;
use std::sync::{Arc, Mutex};
use std::error::Error;

fn handle_input_data(data: &[f32], lb: &Arc<Mutex<loudnessbuffer::LoudnessBuffer>>) {
    (*lb.lock().unwrap()).extend(data.iter().copied());
}

async fn handle_scanning(mut event_stream: impl Stream<Item = ButtplugClientEvent> + Unpin) {
    loop {
        match event_stream.next().await.unwrap() {
            ButtplugClientEvent::DeviceAdded(dev) => {
                tokio::spawn(async move {
                    println!("device added: {}", dev.name);
                });
            },
            ButtplugClientEvent::ScanningFinished => {
                println!("scanning finished signaled!");
                return;
            },
            ButtplugClientEvent::ServerDisconnect => {
                println!("server disconnected!");
            },
            _ => {
                println!("something happened!");
            },
        }
    }
}

async fn run(lb: Arc<Mutex<loudnessbuffer::LoudnessBuffer>>) -> Result<(), Box<dyn Error>> {
    // connect Buttplug devices
    let client = ButtplugClient::new("buzznoise buttplug client");
    let event_stream = client.event_stream();
    client.connect_in_process(&ButtplugServerOptions::default()).await?;
    client.start_scanning().await?;
    let scan_handler = tokio::spawn(handle_scanning(event_stream));
    println!("\nscanning for devices! press enter at any point to stop scanning and start listening to audio input.");
    BufReader::new(io::stdin()).lines().next_line().await?;
    client.stop_scanning().await?;
    scan_handler.await?;
    // poll average
    let devices = client.devices();
    tokio::spawn(async move {
        loop {
            let power = 2.0 * (*lb.lock().unwrap()).rms();
            let speed = power.min(1.0) as f64;
            println!(
                "power: {:.5}  |  vibration speed: {:.5}  [{:<5}]",
                power, speed, "=".repeat((speed * 5.0) as usize)
            );
            for dev in devices.clone() {
                tokio::spawn(async move {
                    if dev.allowed_messages.contains_key(&ButtplugClientDeviceMessageType::VibrateCmd) {
                        dev.vibrate(VibrateCommand::Speed(speed)).await.unwrap();
                    }
                });
            }
            Delay::new(std::time::Duration::from_millis(50)).await;
        }
    });
    println!("\nconnected MIDI input to device output! press enter at any point to quit.");
    BufReader::new(io::stdin()).lines().next_line().await?;
    println!("stopping all devices and quitting...");
    client.stop_all_devices().await?;
    Ok(())
}

fn main() {
    // get command-line arguments
    let _matches = clap::App::new("buzznoise")
        .version("0.1")
        .about("get a buzz on audio input!")
        .get_matches();
    // connect audio stream to Buttplug
    let ending: Result<(), Box<dyn Error>> = (|| -> Result<(), Box<dyn Error>> {
        let lb_a = Arc::new(Mutex::new(loudnessbuffer::LoudnessBuffer::new(50)));
        let lb_b = lb_a.clone();
        // get audio stream
        let host = cpal::default_host();
        let device = host.default_input_device().unwrap();
        let config = device.default_input_config()?;
        let err_fn = move |err| { eprintln!("an error occurred on stream: {}", err); };
        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_input_stream(
                &config.into(),
                move |data, _: &_| handle_input_data(data, &lb_b),
                err_fn
            )?,
            _ => todo!(),
        };
        stream.play()?;
        // start async runtime
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;
        runtime.block_on(run(lb_a))?;
        Ok(())
    })();
    // say goodbye
    match ending {
        Ok(()) => { println!("bye-bye! >:3c"); },
        Err(e) => { eprintln!("error: {}", e); },
    }
}
