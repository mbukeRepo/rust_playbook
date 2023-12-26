extern crate colored;

use colored::*;
use crossbeam_channel::{unbounded, Receiver};

enum LightMsg {
	ChangeColor(u8, u8, u8),
	Disconnect,
	On,
	Off
}

enum LightStatus {
	Off, 
	On
}

fn spawn_light_thread(receiver: Receiver<LightMsg>) -> std::thread::JoinHandle<LightStatus> {
	std::thread::spawn(move || {
		let mut light_status = LightStatus::Off;
		loop {
			if let Ok(msg) = receiver.recv() {
				match msg {
					LightMsg::ChangeColor(r, b, g) => {
						println!("Color changed to: {}", "  ".on_truecolor(r, b, g));
						match light_status {
							LightStatus::Off => println!("Light is Off"),
							LightStatus::On => println!("Light is on"),
						}
					},
					LightMsg::On => {
						println!("Turned light on.");
						light_status = LightStatus::On;
					},
					LightMsg::Off => {
						println!("Turned light off.");
						light_status = LightStatus::Off;
					},
					LightMsg::Disconnect => {
						println!("Disconnecting...");
						light_status = LightStatus::Off;
						break;
					}
				}
			} else {
				println!("channel disconnected!");
				light_status = LightStatus::Off;
				break;
			}
		}
		light_status
	})
}

fn main() {
	let (s, r) = unbounded();
	
	let light = spawn_light_thread(r);

	s.send(LightMsg::On);
	s.send(LightMsg::ChangeColor(255, 0, 0));
	s.send(LightMsg::ChangeColor(0, 128, 0));

	s.send(LightMsg::Off);
	s.send(LightMsg::Disconnect);

	let light_status = light.join();
}
