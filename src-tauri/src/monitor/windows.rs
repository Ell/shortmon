use std::fmt;
use std::os::windows::raw::HANDLE;
use winapi::shared::minwindef::{BYTE, DWORD, LPARAM, LPDWORD};
use winapi::shared::windef::{HDC, HMONITOR, LPRECT};
use winapi::um::lowlevelmonitorconfigurationapi::{
    CapabilitiesRequestAndCapabilitiesReply, GetCapabilitiesStringLength, SetVCPFeature,
};
use winapi::um::physicalmonitorenumerationapi::{
    GetNumberOfPhysicalMonitorsFromHMONITOR, GetPhysicalMonitorsFromHMONITOR, PHYSICAL_MONITOR,
};
use winapi::um::winuser::EnumDisplayMonitors;

use crate::errors::MonitorError;
use crate::monitor::capabilities::MonitorCapabilities;
use crate::monitor::input::{get_all_inputs_from_capabilities_string, MonitorInput};

pub unsafe fn set_vcp_feature(
    hmonitor: HANDLE,
    code: BYTE,
    new_value: DWORD,
) -> Result<(), MonitorError> {
    let result = SetVCPFeature(hmonitor, code, new_value);

    return match result {
        1 => Ok(()),
        _ => Err(MonitorError("Failed to set value for monitor")),
    };
}

pub unsafe fn enum_display_monitors() -> Vec<HMONITOR> {
    let hdc = std::ptr::null_mut();
    let lprc_clip = std::ptr::null_mut();

    let mut monitors: Box<Vec<HMONITOR>> = Box::new(vec![]);

    let mons_ptr = Box::into_raw(monitors);
    let mons_lparam: LPARAM = std::mem::transmute(mons_ptr);

    EnumDisplayMonitors(hdc, lprc_clip, Some(lpfn_enum_callback), mons_lparam);

    monitors = Box::from_raw(mons_ptr);

    return monitors.to_vec();
}

pub unsafe fn get_number_of_physical_monitors_from_hmonitor(hmonitor: HMONITOR) -> i32 {
    let mut num_phys_monitors: Box<i32> = Box::new(0);

    let num_ptr = Box::into_raw(num_phys_monitors);
    let num_lpdword: LPDWORD = std::mem::transmute(num_ptr);

    GetNumberOfPhysicalMonitorsFromHMONITOR(hmonitor, num_lpdword);

    num_phys_monitors = Box::from_raw(num_ptr);

    return *num_phys_monitors;
}

pub unsafe fn get_physical_monitors_from_hmonitor(
    monitor: HMONITOR,
    phys_mons_size: i32,
) -> Vec<PHYSICAL_MONITOR> {
    let mut phys_mons: Vec<PHYSICAL_MONITOR> = vec![Default::default(); phys_mons_size as usize];

    GetPhysicalMonitorsFromHMONITOR(monitor, phys_mons.len() as u32, phys_mons.as_mut_ptr());

    return phys_mons;
}

pub unsafe fn get_capabilities_string_length(phys_mon: PHYSICAL_MONITOR) -> i32 {
    let mut cap_string_len: Box<i32> = Box::new(0);

    let cap_len_ptr = Box::into_raw(cap_string_len);
    let cap_lpdword: LPDWORD = std::mem::transmute(cap_len_ptr);

    GetCapabilitiesStringLength(phys_mon.hPhysicalMonitor, cap_lpdword);

    cap_string_len = Box::from_raw(cap_len_ptr);

    return *cap_string_len;
}

pub unsafe fn capabilities_request_and_capabilities_reply(
    phys_mon: PHYSICAL_MONITOR,
    cap_string_len: i32,
) -> Result<String, MonitorError> {
    let mut cap_string_buf: Vec<i8> = vec![0; cap_string_len as usize];

    CapabilitiesRequestAndCapabilitiesReply(
        phys_mon.hPhysicalMonitor,
        cap_string_buf.as_mut_ptr(),
        cap_string_len as u32,
    );

    String::from_utf8(cap_string_buf.iter().map(|&c| c as u8).collect())
        .map_err(|_| MonitorError("Unable to build cap string from buffer"))
        .map(|cap_string| String::from(cap_string.trim_matches(char::from(0))))
}

unsafe extern "system" fn lpfn_enum_callback(
    hmon: HMONITOR,
    _hdc: HDC,
    _lprect: LPRECT,
    lparam: LPARAM,
) -> i32 {
    let mons_ptr: *mut Vec<HMONITOR> = std::mem::transmute(lparam);
    let mons_ref = &mut *mons_ptr;

    mons_ref.push(hmon);

    return 1;
}

#[derive(Default, Clone)]
pub struct Monitor {
    pub id: u8,
    pub cap_string: Option<String>,
    pub capabilities: Option<MonitorCapabilities>,
    pub phys_mons: PHYSICAL_MONITOR,
    pub inputs: Vec<MonitorInput>,
}

impl fmt::Display for Monitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}>", "Monitor")
    }
}

impl Monitor {
    pub fn set_input(&self, input: MonitorInput) -> Result<(), MonitorError> {
        let hmonitor = self.phys_mons.hPhysicalMonitor;

        let code = 0x60; // Input Select VCP Code
        let input_code = input as u32;

        unsafe {
            set_vcp_feature(hmonitor, code, input_code)
                .map_err(|_| MonitorError("Failed to set VCP feature"))
        }
    }

    pub fn get_inputs(&self) -> Result<Vec<MonitorInput>, MonitorError> {
        Ok(self.inputs.clone())
    }

    pub fn get_all_monitors() -> Result<Vec<Monitor>, MonitorError> {
        unsafe {
            let display_mons = enum_display_monitors();

            let mut monitors: Vec<Monitor> = vec![];

            for (i, mon_ref) in display_mons.iter().enumerate() {
                let phys_num = get_number_of_physical_monitors_from_hmonitor(*mon_ref);
                let phys_mons = get_physical_monitors_from_hmonitor(*mon_ref, phys_num);

                for phys_mon in phys_mons {
                    let mut mon = Monitor {
                        id: i as u8,
                        ..Default::default()
                    };

                    let cap_str_len = get_capabilities_string_length(phys_mon);

                    let cap_reply_str =
                        capabilities_request_and_capabilities_reply(phys_mon, cap_str_len)
                            .unwrap_or("".to_string());

                    if cap_reply_str.is_empty() || cap_reply_str.eq("") {
                        continue;
                    }

                    mon.cap_string = Some(cap_reply_str.clone());

                    let caps = MonitorCapabilities::from_cap_string(cap_reply_str);
                    match caps {
                        Ok(result) => {
                            mon.phys_mons = phys_mon;

                            if let Ok(inputs) = get_all_inputs_from_capabilities_string(&result) {
                                mon.inputs = inputs;
                            } else {
                                mon.inputs = vec![];
                            }

                            mon.capabilities = Some(result);

                            monitors.push(mon);
                        }
                        Err(_) => {}
                    }
                }
            }

            Ok(monitors)
        }
    }
}
