use std::{
    cell::{BorrowMutError, RefCell},
    convert::TryFrom,
    string::FromUtf8Error,
};

use ddc_hi::{Ddc, Display, Handle};

use crate::{
    errors::MonitorError,
    mccs::ParserError,
    monitor::{
        capabilities::MonitorCapabilities,
        input::{get_all_inputs_from_capabilities_string, MonitorInput},
    },
};

type Result<T> = std::result::Result<T, MonitorError>;

pub struct Monitor {
    pub id: u8,
    pub capabilities: Option<MonitorCapabilities>,
    handle: RefCell<Handle>,
    inputs: Vec<MonitorInput>,
}

impl Monitor {
    pub fn get_all_monitors() -> Result<Vec<Monitor>> {
        let monitors = Display::enumerate()
            .into_iter()
            .enumerate()
            .filter_map(|(i, display)| {
                Some(Monitor {
                    id: i as u8,
                    ..display.try_into().ok()?
                })
            });

        Ok(monitors.collect())
    }

    pub fn get_inputs(&self) -> Result<Vec<MonitorInput>> {
        Ok(self.inputs.clone())
    }

    pub fn set_input(&self, input: MonitorInput) -> Result<()> {
        Ok(self
            .handle
            .try_borrow_mut()?
            .set_vcp_feature(0x60, input as u16)?)
    }
}

impl TryFrom<Display> for Monitor {
    type Error = MonitorError;

    fn try_from(mut val: Display) -> std::result::Result<Self, Self::Error> {
        let capabilities = Some(MonitorCapabilities::from_cap_string(String::from_utf8(
            val.handle.capabilities_string()?,
        )?)?);

        let inputs = get_all_inputs_from_capabilities_string(capabilities.as_ref().unwrap())?;

        Ok(Monitor {
            id: 0,
            capabilities,
            handle: RefCell::new(val.handle),
            inputs,
        })
    }
}

impl From<anyhow::Error> for MonitorError {
    fn from(val: anyhow::Error) -> Self {
        // Leak the error description
        let error: &'static mut String = Box::leak(Box::new(val.to_string()));

        Self(error.as_str())
    }
}

impl From<FromUtf8Error> for MonitorError {
    fn from(val: FromUtf8Error) -> Self {
        // Leak the error description
        let error: &'static mut String = Box::leak(Box::new(val.to_string()));

        Self(error.as_str())
    }
}

impl From<ParserError> for MonitorError {
    fn from(val: ParserError) -> Self {
        // Leak the error description
        let error: &'static mut String = Box::leak(Box::new(val.to_string()));

        Self(error.as_str())
    }
}

impl From<BorrowMutError> for MonitorError {
    fn from(val: BorrowMutError) -> Self {
        // Leak the error description
        let error: &'static mut String = Box::leak(Box::new(val.to_string()));

        Self(error.as_str())
    }
}
