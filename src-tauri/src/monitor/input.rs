use crate::errors::MonitorError;
use crate::monitor::capabilities::MonitorCapabilities;
use std::fmt;

#[derive(Debug, FromPrimitive, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub enum MonitorInput {
    AnalogVideo1 = 1,
    AnalogVideo2,
    DVI1,
    DVI2,
    CompositeVideo1,
    CompositeVideo2,
    SVideo1,
    SVideo2,
    Tuner1,
    Tuner2,
    Tuner3,
    ComponentVideo1,
    ComponentVideo2,
    ComponentVideo3,
    DisplayPort1,
    DisplayPort2,
    HDMI1,
    HDMI2,
    Unknown,
    Reserved,
}

impl fmt::Display for MonitorInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use MonitorInput::*;

        let s = match self {
            AnalogVideo1 => "Analog Video 1",
            AnalogVideo2 => "Analog Video 2",
            DVI1 => "DVI 1",
            DVI2 => "DVI 2",
            CompositeVideo1 => "Composite Video 1",
            CompositeVideo2 => "Composite Video 2",
            SVideo1 => "SVideo 1",
            SVideo2 => "SVideo 2",
            Tuner1 => "Tuner 1",
            Tuner2 => "Tuner 2",
            Tuner3 => "Tuner 3",
            ComponentVideo1 => "Component Video 1",
            ComponentVideo2 => "Component Video 2",
            ComponentVideo3 => "Component Video 3",
            DisplayPort1 => "DP 1",
            DisplayPort2 => "DP 2",
            HDMI1 => "HDMI 1",
            HDMI2 => "HDMI 2",
            _ => "Unknown",
        };

        write!(f, "{}", s)
    }
}

pub fn get_all_inputs_from_capabilities_string(
    capabilities: &MonitorCapabilities,
) -> Result<Vec<MonitorInput>, MonitorError> {
    let input_values = capabilities.vcp_codes.iter().find_map(|cmd| {
        let has_command = &cmd.command == "60";

        match has_command {
            true => Some(&cmd.values),
            false => None,
        }
    });

    if let Some(values) = input_values {
        let inputs: Vec<_> = values
            .iter()
            .map(|value| match &value.command[..] {
                "01" => MonitorInput::AnalogVideo1,
                "02" => MonitorInput::AnalogVideo2,
                "03" => MonitorInput::DVI1,
                "04" => MonitorInput::DVI2,
                "05" => MonitorInput::CompositeVideo1,
                "06" => MonitorInput::CompositeVideo2,
                "07" => MonitorInput::SVideo1,
                "08" => MonitorInput::SVideo2,
                "09" => MonitorInput::Tuner1,
                "0A" => MonitorInput::Tuner2,
                "0B" => MonitorInput::Tuner3,
                "0C" => MonitorInput::ComponentVideo1,
                "0D" => MonitorInput::ComponentVideo2,
                "0E" => MonitorInput::ComponentVideo3,
                "0F" => MonitorInput::DisplayPort1,
                "10" => MonitorInput::DisplayPort2,
                "11" => MonitorInput::HDMI1,
                "12" => MonitorInput::HDMI2,
                _ => MonitorInput::Unknown,
            })
            .collect();

        return Ok(inputs);
    }

    Ok(vec![])
}
