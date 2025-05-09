use std::sync::mpsc::TryIter;
use crate::devices::DevicesDisplayInfo;
use crate::devices::{Devices, JoystickState};
use crate::registrar;
use crate::event::RawEvent;
use std::sync::mpsc::TryRecvError;
use crate::rawinput::{get_event, get_joystick_state};
use std::time::{SystemTime, UNIX_EPOCH};
use winapi::shared::minwindef::UINT;
use winapi::shared::windef::HWND;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::{
    CreateWindowExW, DefWindowProcW, RegisterClassExW, CW_USEDEFAULT, HWND_MESSAGE, WNDCLASSEXW,
};

use std::collections::VecDeque;
use std::ffi::OsStr;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use std::thread;
use std::thread::JoinHandle;
use std::collections::HashSet;
use std::iter::FromIterator;

use std::sync::mpsc::{channel, Receiver, Sender};

enum Command {
    Register(DeviceType),
    FilterDevices(Vec<String>),
    UnfilterDevices,
    GetEvent,
    GetJoystickState(usize),
    Finish,
    PrintDeviceList,
    GetDeviceList,
    GetDeviceStats,
}

/// Types of Raw Input Device
#[derive(PartialEq, Eq, Clone, Hash)]
pub enum DeviceType {
    Mice,
    Keyboards,
    Joysticks(XInputInclude),
}

/// Denotes if Xbox360 controllers should be used
/// Please Note: Rawinput support for official Xbox360 controllers
/// is very limited (triggers share same axis, no support for
/// rumble or the central X button)
/// Please see https://en.wikipedia.org/wiki/DirectInput#Xbox_360_Controller_support
/// for more details
#[derive(PartialEq, Eq, Clone, Hash)]
pub enum XInputInclude {
    True,
    False,
}

#[derive(Default)]
pub struct DeviceStats {
    pub number_of_mice: usize,
    pub number_of_keyboards: usize,
    pub number_of_joysticks: usize,
}

/// Manages Raw Input Processing
pub struct RawInputManager {
    joiner: Option<JoinHandle<()>>,
    sender: Sender<Command>,
    receiver: Receiver<RawEvent>,
    joystick_receiver: Receiver<Option<JoystickState>>,
    device_info_receiver: Receiver<DevicesDisplayInfo>,
    device_stats_receiver: Receiver<DeviceStats>,
}

impl RawInputManager {
    pub fn new() -> Result<RawInputManager, &'static str> {
        let (tx, rx) = channel();
        let (tx2, rx2) = channel();
        let (tx_joy, rx_joy) = channel();
        let (tx_devices, rx_devices) = channel();
        let (tx_stats, rx_stats) = channel();

        let joiner = thread::spawn(move || {
            let hwnd = setup_message_window();
            let mut event_queue = VecDeque::new();
            let mut devices = Devices::new();
            let mut exit = false;
            let mut registrar = registrar::RawInputRegistrar::new();
            while !exit {
                match rx.try_recv() {
                    Err(TryRecvError::Disconnected) => {
                        panic!("Multinput Thread Unexpectedly Disconnected!")
                    }
                    Err(TryRecvError::Empty) => {
                        std::thread::sleep(std::time::Duration::from_nanos(1));
                    }
                    Ok(Command::Register(thing)) => {
                        devices = registrar.register_devices(hwnd, thing).unwrap();
                    }
                    Ok(Command::FilterDevices(strings)) => {
                        devices.filter_device_map(HashSet::from_iter(strings.into_iter()));
                    }
                    Ok(Command::UnfilterDevices) => {
                        devices.reset_device_map();
                    }
                    Ok(Command::GetEvent) => {
                        if let Some(event) = get_event(&mut event_queue, &mut devices) {
                            tx2.send(event).unwrap()
                        }
                    }
                    Ok(Command::Finish) => {
                        exit = true;
                    }
                    Ok(Command::GetJoystickState(id)) => {
                        tx_joy.send(get_joystick_state(&devices, id)).unwrap()
                    }
                    Ok(Command::PrintDeviceList) => print_raw_device_list(&devices),
                    Ok(Command::GetDeviceList) => tx_devices.send(devices.clone().into()).unwrap(),
                    Ok(Command::GetDeviceStats) => tx_stats.send(get_device_stats(&devices)).unwrap(),
                };
            }
        });
        Ok(RawInputManager {
            joiner: Some(joiner),
            sender: tx,
            receiver: rx2,
            joystick_receiver: rx_joy,
            device_stats_receiver: rx_stats,
            device_info_receiver: rx_devices
        })
    }

    /// Allows Raw Input devices of type device_type to be received from the Input Manager
    pub fn register_devices(&mut self, device_type: DeviceType) {
        self.sender.send(Command::Register(device_type)).unwrap();
    }

    /// Filters events returned to the list of names provided by the device_names list
    /// Warning: you still need to register the corresponding device types beforehand for this to work!
    pub fn filter_devices(&mut self, device_names: Vec<String>) {
        self.sender.send(Command::FilterDevices(device_names)).unwrap();
    }

    /// Undoes the application of filter_devices()
    pub fn unfilter_devices(&mut self) {
        self.sender.send(Command::UnfilterDevices).unwrap();
    }

    /// Get Event from the Input Manager
    pub fn get_event(&mut self) -> Option<RawEvent> {
        self.sender.send(Command::GetEvent).unwrap();
        match self.receiver.try_recv() {
            Ok(event) => Some(event),
            _ => None 
        }
    }

    /// Get All Events from the Input Manager
    pub fn get_events(&mut self) -> TryIter<RawEvent> {
        self.sender.send(Command::GetEvent).unwrap();
        self.receiver.try_iter()
    }

    /// Get Joystick State from the Input Manager
    pub fn get_joystick_state(&mut self, id: usize) -> Option<JoystickState> {
        self.sender.send(Command::GetJoystickState(id)).unwrap();
        self.joystick_receiver.recv().unwrap()
    }

    /// Print List of Potential Input Devices
    pub fn print_device_list(&self) {
        self.sender.send(Command::PrintDeviceList).unwrap();
    }

    /// Get Device Stats (number of connected devices)
    pub fn get_device_stats(&self) -> DeviceStats {
        self.sender.send(Command::GetDeviceStats).unwrap();
        self.device_stats_receiver.recv().unwrap()
    }

    /// Get Device list
    pub fn get_device_list(&self) -> DevicesDisplayInfo {
            self.sender.send(Command::GetDeviceList).unwrap();
            self.device_info_receiver.recv().unwrap()
    }
}

impl Drop for RawInputManager {
    fn drop(&mut self) {
        self.sender.send(Command::Finish).unwrap();
        self.joiner.take().unwrap().join().unwrap();
    }
}

fn setup_message_window() -> HWND {
    let hwnd: HWND;
    unsafe {
        let hinstance = GetModuleHandleW(ptr::null());
        if hinstance == ptr::null_mut() {
            panic!("Instance Generation Failed");
        }

        let current_time = SystemTime::now();
        let classname_str = format!(
            "RawInput Hidden Window - {:?}",
            current_time.duration_since(UNIX_EPOCH).unwrap()
        );

        let classname = OsStr::new(&classname_str)
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<_>>();

        let wcex = WNDCLASSEXW {
            cbSize: (mem::size_of::<WNDCLASSEXW>()) as UINT,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hbrBackground: ptr::null_mut(),
            hCursor: ptr::null_mut(),
            hIcon: ptr::null_mut(),
            hIconSm: ptr::null_mut(),
            hInstance: hinstance,
            lpfnWndProc: Some(DefWindowProcW),
            lpszClassName: classname.as_ptr(),
            lpszMenuName: ptr::null_mut(),
            style: 0,
        };
        let a = RegisterClassExW(&wcex);
        if a == 0 {
            panic!("Registering WindowClass Failed!");
        }

        hwnd = CreateWindowExW(
            0,
            classname.as_ptr(),
            classname.as_ptr(),
            0,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            HWND_MESSAGE,
            ptr::null_mut(),
            hinstance,
            ptr::null_mut(),
        );
        if hwnd.is_null() {
            panic!("Window Creation Failed!");
        }
    }
    hwnd
}

/// Prints a list of all available raw input devices
#[allow(unused_variables)]
fn print_raw_device_list(devices: &Devices) {
    for mouse in &devices.mice {
    }
    for keyboard in &devices.keyboards {
    }
    for joystick in &devices.joysticks {
    }
}

fn get_device_stats(devices: &Devices) -> DeviceStats {
    DeviceStats {
        number_of_mice: devices.mice.len(),
        number_of_keyboards: devices.keyboards.len(),
        number_of_joysticks: devices.joysticks.len(),
    }
}
