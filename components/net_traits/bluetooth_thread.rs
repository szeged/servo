/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
use ipc_channel::ipc::IpcSender;

#[derive(Deserialize, Serialize)]
pub enum BluetoothMethodMsg {
    RequestDevice(IpcSender<BluetoothObjectMsg>),
    GATTServerConnect(IpcSender<BluetoothObjectMsg>),
    GetPrimaryService(IpcSender<BluetoothObjectMsg>),
    GetCharacteristic(IpcSender<BluetoothObjectMsg>),
    GetDescriptor(IpcSender<BluetoothObjectMsg>),
    ReadValue(IpcSender<BluetoothObjectMsg>),
    WriteValue(IpcSender<BluetoothObjectMsg>),
    Exit,
}

#[derive(Deserialize, Serialize)]
pub enum BluetoothObjectMsg {
    BluetoothDevice {
        // Bluetooth Device properties
        id: String,
        name: String,
        device_class: u32,
        vendor_id_source: String,
        vendor_id: u32,
        product_id: u32,
        product_version: u32,
        // Advertisiong Data properties
        appearance: u16,
        tx_power: i8,
        rssi: i8
    },
    BluetoothCharacteristic {
        // Characteristic
        uuid: String,
        // Characteristic properties
        broadcast: bool,
        read: bool,
        write_without_response: bool,
        write: bool,
        notify: bool,
        indicate: bool,
        authenticated_signed_writes: bool,
        reliable_write: bool,
        writable_auxiliaries: bool },
    BluetoothService {
        uuid: String,
        is_primary: bool
    },
    BluetoothDescriptor {
        uuid: String
    },
    Error {
        error: String
    },
}
