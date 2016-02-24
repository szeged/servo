/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[cfg(target_os = "linux")]
use blurz::bluetooth_adapter::BluetoothAdapter as BluetoothAdapterBluez;
#[cfg(target_os = "linux")]
use blurz::bluetooth_device::BluetoothDevice as BluetoothDeviceBluez;

#[derive(Clone, Debug)]
pub struct BluetoothAdapter {
    #[cfg(target_os = "linux")]
    adapter: Option<BluetoothAdapterBluez>,
    initialized: bool,
    devices: Vec<BluetoothDevice>,
    object_path: String,
}

#[derive(Clone, Debug)]
pub struct BluetoothDevice {
    adapter_path: String,
    #[cfg(target_os = "linux")]
    device: BluetoothDeviceBluez,
}

#[derive(Debug)]
struct BluetoothDiscoverySession<'a> {
    adapter: &'a BluetoothAdapter
}

#[derive(Debug)]
struct BluetoothDiscoveryFilter {
    rssi: i16,
    pathloss: u16,
}

impl BluetoothAdapter {
    #[cfg(target_os = "linux")]
    pub fn create_adapter() -> BluetoothAdapter {
        let mut adapter = BluetoothAdapter::new();

        let bluez_adapter: BluetoothAdapterBluez = BluetoothAdapterBluez::init().unwrap();
        adapter.set_adapter(bluez_adapter);
        adapter
    }

    #[cfg(target_os = "linux")]
    fn new() -> BluetoothAdapter {
        BluetoothAdapter {
            adapter: None,
            initialized: false,
            object_path: "".to_string(),
            devices: Vec::new(),
        }
    }

    #[cfg(target_os = "linux")]
    fn set_adapter(&mut self, adapter: BluetoothAdapterBluez) {
        self.adapter = Some(adapter.clone());
        self.initialized = true;
        self.object_path = self.get_adapter().get_object_path();

        let devices = self.get_adapter().get_devices();

        for device in devices {
            self.device_added(device.clone());
        }
    }

    #[cfg(target_os = "linux")]
    fn get_adapter(&self) -> BluetoothAdapterBluez {
        self.adapter.clone().unwrap()
    }

    #[cfg(target_os = "linux")]
    fn device_added(&mut self, object_path: String) {
        let adapter_path = self.get_object_path();
        self.devices.push(
            BluetoothDevice::new(adapter_path,
                                 BluetoothDeviceBluez::create_device(
                                    object_path.clone())))
    }

    #[cfg(target_os = "linux")]
    pub fn get_object_path(&self) -> String {
        self.get_adapter().get_object_path()
    }

    #[cfg(target_os = "linux")]
    pub fn get_address(&self) -> String {
        self.get_adapter().get_address()
    }

    #[cfg(target_os = "linux")]
    pub fn get_name(&self) -> String {
        self.get_adapter().get_name()
    }

    #[cfg(target_os = "linux")]
    pub fn get_alias(&self) -> String {
        self.get_adapter().get_alias()
    }

    #[cfg(target_os = "linux")]
    pub fn set_alias(&self, value: String) {
        self.get_adapter().set_alias(value);
    }

    #[cfg(target_os = "linux")]
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    #[cfg(target_os = "linux")]
    pub fn is_present(&self) -> bool {
        !(self.object_path.is_empty())
    }

    #[cfg(target_os = "linux")]
    pub fn is_powered(&self) -> bool {
        self.get_adapter().is_powered()
    }

    #[cfg(target_os = "linux")]
    pub fn set_powered(&self, value: bool) {
        self.get_adapter().set_powered(value);
    }

    #[cfg(target_os = "linux")]
    pub fn is_discoverable(&self) -> bool {
        self.get_adapter().is_discoverable()
    }

    #[cfg(target_os = "linux")]
    pub fn set_discoverable(&self, value: bool) {
        self.get_adapter().set_discoverable(value);
    }

    #[cfg(target_os = "linux")]
    pub fn is_discovering(&self) -> bool {
        self.get_adapter().is_discovering()
    }

    #[cfg(target_os = "linux")]
    pub fn start_discovery_session(&self) {

    }

    #[cfg(target_os = "linux")]
    pub fn stop_discovery_session(&self) {

    }

    #[cfg(target_os = "linux")]
    pub fn get_devices(&self) -> Vec<BluetoothDevice>{
        self.devices.clone()
    }

    #[cfg(target_os = "linux")]
    pub fn get_device(&self, address: String) -> Option<BluetoothDevice> {
        for device in &self.devices {
            if device.get_address() == address {
                return Some(device.clone());
            }
        }
        None
    }

    /*#[cfg(target_os = "android")]
    pub fn create_adapter() -> BluetoothAdapter {

    }*/
}

impl BluetoothDevice {
    #[cfg(target_os = "linux")]
    pub fn new(adapter_path: String, device: BluetoothDeviceBluez) -> BluetoothDevice {
        BluetoothDevice {
            adapter_path: adapter_path,
            device: device,
        }
    }

    #[cfg(target_os = "linux")]
    pub fn get_object_path(&self) -> String {
        self.get_device().get_object_path()
    }

    #[cfg(target_os = "linux")]
    fn get_device(&self) -> BluetoothDeviceBluez {
        self.device.clone()
    }

    #[cfg(target_os = "linux")]
    pub fn get_address(&self) -> String {
        self.get_device().get_address()
    }

    #[cfg(target_os = "linux")]
    pub fn get_name(&self) -> String {
        self.get_device().get_name()
    }

    #[cfg(target_os = "linux")]
    pub fn get_class(&self) -> u32 {
        self.get_device().get_class()
    }

    #[cfg(target_os = "linux")]
    pub fn get_vendor_id(&self) -> u32 {
        self.get_device().get_vendor_id()
    }

    #[cfg(target_os = "linux")]
    pub fn get_product_id(&self) -> u32 {
        self.get_device().get_product_id()
    }

    #[cfg(target_os = "linux")]
    pub fn get_device_id(&self) -> u32 {
        self.get_device().get_device_id()
    }

    #[cfg(target_os = "linux")]
    pub fn is_pairable(&self) -> bool {
        self.get_device().is_pairable()
    }

    #[cfg(target_os = "linux")]
    pub fn is_paired(&self) -> bool {
        self.get_device().is_paired()
    }

    #[cfg(target_os = "linux")]
    pub fn is_connectable(&self) -> bool {
        self.get_device().is_connectable()
    }

    #[cfg(target_os = "linux")]
    pub fn is_connected(&self) -> bool {
        self.get_device().is_connected()
    }

    #[cfg(target_os = "linux")]
    pub fn is_trustable(&self) -> bool {
        self.get_device().is_trustable()
    }

    #[cfg(target_os = "linux")]
    pub fn get_uuids(&self) -> Vec<String> {
        self.get_device().get_uuids()
    }

    #[cfg(target_os = "linux")]
    pub fn get_inquiry_rssi(&self) -> i32 {
        self.get_device().get_rssi()
    }

    #[cfg(target_os = "linux")]
    pub fn get_inquiry_tx_power(&self) -> i32 {
        0
    }

    #[cfg(target_os = "linux")]
    pub fn create_gatt_connection(&self) {
        self.get_device().connect();
    }
}

