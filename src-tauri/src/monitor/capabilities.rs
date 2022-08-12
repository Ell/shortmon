use crate::monitor::mccs::{ParserError, VCPCommand};

use super::mccs::{extract_atom, extract_vcp_commands, parse_cap_string};

#[derive(Default, Debug, Clone)]
pub struct MonitorCapabilities {
    pub protocol_class: String,
    pub display_type: String,
    pub commands: Vec<VCPCommand>,
    pub vcp_codes: Vec<VCPCommand>,
    pub display_model: String,
    pub mccs_version: String,
}

impl MonitorCapabilities {
    pub fn from_cap_string(cap_string: String) -> Result<MonitorCapabilities, ParserError> {
        let pairs = parse_cap_string(cap_string)?;

        let mut caps = MonitorCapabilities {
            ..Default::default()
        };

        for (key, value) in pairs {
            match key.as_str() {
                "prot" => caps.protocol_class = extract_atom(value),
                "type" => caps.display_type = extract_atom(value),
                "cmds" => caps.commands = extract_vcp_commands(value),
                "vcp" => caps.vcp_codes = extract_vcp_commands(value),
                "model" => caps.display_model = extract_atom(value),
                "mccs_ver" => caps.mccs_version = extract_atom(value),
                _ => {}
            }
        }

        return Ok(caps);
    }
}
