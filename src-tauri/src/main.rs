#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    sync::mpsc::{self, Receiver, SyncSender},
    thread,
};

use monitor::{input::MonitorInput, Monitor};
use tauri::{
    AppHandle, CustomMenuItem, Manager, RunEvent, SystemTray, SystemTrayEvent, SystemTrayMenu,
};

#[macro_use]
extern crate num_derive;

mod errors;
mod monitor;

#[derive(serde::Serialize, Clone)]
struct MonitorInfo {
    id: u8,
    model: String,
    inputs: Vec<MonitorInput>,
}

enum MonitorManagerCommand {
    RefreshList(tauri::Window),
    SwitchInput((u8, MonitorInput)),
}

struct MonitorManager {
    tx: SyncSender<MonitorManagerCommand>,
}

#[tauri::command]
fn refresh_monitor_info(state: tauri::State<'_, MonitorManager>, window: tauri::Window) {
    state
        .tx
        .send(MonitorManagerCommand::RefreshList(window.clone()))
        .ok();
}

#[tauri::command]
fn switch_monitor_input(
    state: tauri::State<'_, MonitorManager>,
    monitor_idx: u8,
    input: MonitorInput,
) {
    state
        .tx
        .send(MonitorManagerCommand::SwitchInput((monitor_idx, input)))
        .ok();
}

fn spawn_monitor_manager() -> SyncSender<MonitorManagerCommand> {
    let (tx, rx): (
        SyncSender<MonitorManagerCommand>,
        Receiver<MonitorManagerCommand>,
    ) = mpsc::sync_channel(8);

    thread::spawn(move || {
        let mut monitors = vec![];

        for event in rx {
            match event {
                MonitorManagerCommand::RefreshList(window) => {
                    monitors = Monitor::get_all_monitors().map_or(vec![], |v| v);
                    let info_list = monitors
                        .iter()
                        .map(|m| MonitorInfo {
                            id: m.id,
                            model: m
                                .capabilities
                                .as_ref()
                                .map(|c| c.clone().display_model)
                                .unwrap_or("Generic Display".to_string()),
                            inputs: m.get_inputs().unwrap_or(vec![]),
                        })
                        .collect::<Vec<_>>();
                    window.emit("monitor-info", info_list).unwrap();
                }
                MonitorManagerCommand::SwitchInput((id, input)) => {
                    if let Some(monitor) = monitors.iter().find(|m| m.id == id) {
                        monitor.set_input(input).ok();
                    }
                }
            }
        }
    });

    tx
}

fn make_tray() -> SystemTray {
    let menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("refresh", "Refresh"))
        .add_item(CustomMenuItem::new("quit", "Quit"));

    SystemTray::new().with_menu(menu)
}

fn handle_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "refresh" => {
                if let Some(window) = app.get_window("main") {
                    let state = app.state::<MonitorManager>();

                    state
                        .tx
                        .send(MonitorManagerCommand::RefreshList(window.clone()))
                        .ok();
                }
            }
            "quit" => app.exit(0),
            _ => (),
        },
        SystemTrayEvent::DoubleClick { .. } => {
            if let Some(window) = app.get_window("main") {
                window.show().ok();
            }
        }
        _ => (),
    };
}

fn main() {
    let app = tauri::Builder::default()
        .manage(MonitorManager {
            tx: spawn_monitor_manager(),
        })
        .system_tray(make_tray())
        .on_system_tray_event(handle_tray_event)
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            refresh_monitor_info,
            switch_monitor_input
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|app_handle, e| match e {
        RunEvent::WindowEvent { event, .. } => match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                let app_handle = app_handle.clone();

                api.prevent_close();

                if let Some(window) = app_handle.get_window("main") {
                    window.hide().ok();
                }
            }
            _ => (),
        },
        _ => (),
    });
}
