// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::io::prelude::*;
use std::sync::Mutex;
use evdev::{Device, enumerate, InputEventKind, Key, uinput::{VirtualDevice, VirtualDeviceBuilder}, AttributeSet, InputEvent, EventType};

struct AppState {
    device_list: Mutex<Vec<Device>>,
    selected_device_idx: Mutex<usize>,
    mappings: Mutex<Vec<(Key, Key)>>
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_devices(state: tauri::State<AppState>) -> Vec<(usize, String)> {
    let mut args = std::env::args_os();
    args.next();
    if let Some(dev_file) = args.next() {
        panic!("Don't know what to do now, we got a single input device maybe? {dev_file:?}");
    } else {
        let mut devices = enumerate().map(|t| t.1).collect::<Vec<_>>();
        // readdir returns them in reverse order from their eventN names for some reason
        devices.reverse();
        let mut device_list: Vec<(usize, String)> = Vec::new();

        for (i, d) in devices.iter().enumerate() {
            device_list.push((i, d.name().unwrap_or("Unnamed device").to_string()));
            println!("{}: {}", i, d.name().unwrap_or("Unnamed device"));
        }
        device_list
        //devices.iter()
        //.map(|device| String::from(device.name().unwrap_or("Unnamed device")))
        //.collect::<Vec<_>>()
    }
}

#[tauri::command]
fn select_device(idx: usize, state: tauri::State<AppState>) {
    let mut sel_dev_idx = state.selected_device_idx.lock().unwrap();
    *sel_dev_idx = idx;
}

fn old_thing() -> Device {
    let mut args = std::env::args_os();
    args.next();
    if let Some(dev_file) = args.next() {
        Device::open(dev_file).unwrap()
    } else {
        let mut devices = enumerate().map(|t| t.1).collect::<Vec<_>>();
        // readdir returns them in reverse order from their eventN names for some reason
        devices.reverse();
        for (i, d) in devices.iter().enumerate() {
            println!("{}: {}", i, d.name().unwrap_or("Unnamed device"));
        }
        print!("Select the device [0-{}]: ", devices.len());
        let _ = std::io::stdout().flush();
        let mut chosen = String::new();
        std::io::stdin().read_line(&mut chosen).unwrap();
        let n = chosen.trim().parse::<usize>().unwrap();
        devices.into_iter().nth(n).unwrap()
    }
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            device_list: Mutex::new(Vec::new()),
            selected_device_idx: Mutex::new(0),
            mappings: Mutex::new(Vec::new())
        })
        .invoke_handler(tauri::generate_handler![get_devices, select_device])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


pub fn pick_device() -> Device {
    let mut args = std::env::args_os();
    args.next();
    if let Some(dev_file) = args.next() {
        Device::open(dev_file).unwrap()
    } else {
        let mut devices = enumerate().map(|t| t.1).collect::<Vec<_>>();
        // readdir returns them in reverse order from their eventN names for some reason
        devices.reverse();
        for (i, d) in devices.iter().enumerate() {
            println!("{}: {}", i, d.name().unwrap_or("Unnamed device"));
        }
        print!("Select the device [0-{}]: ", devices.len());
        let _ = std::io::stdout().flush();
        let mut chosen = String::new();
        std::io::stdin().read_line(&mut chosen).unwrap();
        let n = chosen.trim().parse::<usize>().unwrap();
        devices.into_iter().nth(n).unwrap()
    }
}

fn virtual_mouse() -> std::io::Result<VirtualDevice> {
    return VirtualDeviceBuilder::new()?
        .name("input-mapper::fake-mouse")
        .with_keys(&AttributeSet::from_iter([
            Key::BTN_LEFT,
            Key::BTN_MIDDLE,
            Key::BTN_RIGHT,
        ]))?
        .build();
}

fn prev_main() {
    let mut dev = pick_device();
    let mut virtual_mouse = virtual_mouse().unwrap();
    println!("Events:");
    loop {
        for ev in dev.fetch_events().unwrap() {
            let kind = ev.kind();
            let value = ev.value();
            match kind {
                InputEventKind::Key(Key::KEY_T) => {
                    println!("Detected KEY_T event with value {}, emitting BTN_MIDDLE", value);
                    let middle_down = InputEvent::new(EventType::KEY, Key::BTN_MIDDLE.0, value);
                    virtual_mouse.emit(&[middle_down]).unwrap();
                }
                _ => {
                    // ignore
                }
            }
        }
    }
}

