mod config;

use std::collections::HashMap;

use anyhow::Context as _;
use evdev::uinput::VirtualDeviceBuilder;
use evdev::{AttributeSet, EventType, InputEvent, InputEventKind, Key};

fn main() -> anyhow::Result<()> {
    let config_path = std::env::args()
        .into_iter()
        .nth(1)
        .context("Expected a path to the config file")?;

    let config = std::fs::read_to_string(config_path).context("Couldn’t read config file")?;
    let config: config::Config = ron::from_str(&config).context("Invalid config")?;

    let mut input_device =
        evdev::Device::open(config.device).context("Couldn’t open input device")?;

    println!("Input Device:\n{}", input_device);

    let mut vdevices = HashMap::new();

    for (name, definition) in config.vdevices {
        let mut keys = AttributeSet::<Key>::new();
        for key in definition.keys {
            keys.insert(key);
        }

        let device = VirtualDeviceBuilder::new()?
            .name(&name.0)
            .with_keys(&keys)?
            .build()?;

        let path = device.get_syspath()?;
        println!("{} is available as {}", name.0, path.display());

        vdevices.insert(name, device);
    }

    let mut current_mode = config
        .modes
        .get(&config.default_mode)
        .context("Default mode not found")?;

    println!("Waiting for Ctrl-C...");

    loop {
        let events: Vec<InputEvent> = input_device
            .fetch_events()
            .context("Failed to fetch input devices events")?
            .into_iter()
            .collect();

        for (vdevice_name, mapping) in &current_mode.direct {
            let device = vdevices
                .get_mut(vdevice_name)
                .context("Virtual device not found")?;

            let mut mapped_events = Vec::new();

            for ev in &events {
                if let Some(mapped) = mapping.get(&ev.kind()) {
                    if let InputEventKind::Key(key) = mapped {
                        mapped_events.push(InputEvent::new(EventType::KEY, key.code(), ev.value()));
                    }
                }
            }

            device
                .emit(&mapped_events)
                .context("Couldn’t emit events for virtual device")?;
        }

        let mut change_mode = None;

        for ev in events {
            if let Some(action) = current_mode.custom.get(&ev.kind()) {
                match action {
                    config::CustomAction::ChangeMode(new_mode) => change_mode = Some(new_mode),
                }
            }
        }

        if let Some(new_mode) = change_mode {
            current_mode = config
                .modes
                .get(&new_mode)
                .with_context(|| format!("{} mode not found", new_mode.0))?;

            println!("Mode changed to {}", new_mode.0);
        }
    }
}
