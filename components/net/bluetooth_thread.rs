/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use ipc_channel::ipc::{self, IpcReceiver, IpcSender};
use net_traits::bluetooth_thread::{BluetoothMethodMsg, BluetoothObjectMsg};
use std::borrow::ToOwned;
use std::string::String;
use util::thread::spawn_named;

pub trait BluetoothThreadFactory {
    fn new() -> Self;
}

impl BluetoothThreadFactory for IpcSender<BluetoothMethodMsg> {
    fn new() -> IpcSender<BluetoothMethodMsg> {
        let (sender, receiver) = ipc::channel().unwrap();
        spawn_named("BluetoothThread".to_owned(), move || {
            BluetoothManager::new(receiver).start();
        });
        sender
    }
}

pub struct BluetoothManager {
    receiver: IpcReceiver<BluetoothMethodMsg>,
}

impl BluetoothManager {
    pub fn new (receiver: IpcReceiver<BluetoothMethodMsg>) -> BluetoothManager {
        BluetoothManager {
            receiver: receiver,
        }
    }

    fn start(&mut self) {
        loop {
            match self.receiver.recv().unwrap() {
                BluetoothMethodMsg::RequestDevice(sender) => {
                    self.request_device(sender)
                }
                BluetoothMethodMsg::GATTServerConnect(sender) => {
                    self.gatt_server_connect(sender)
                }
                BluetoothMethodMsg::GetPrimaryService(sender) => {
                    self.get_primary_service(sender)
                }
                BluetoothMethodMsg::GetCharacteristic(sender) => {
                    self.get_characteristic(sender)
                }
                BluetoothMethodMsg::GetDescriptor(sender) => {
                    self.get_descriptor(sender)
                }
                BluetoothMethodMsg::ReadValue(sender) => {
                    self.read_value(sender)
                }
                BluetoothMethodMsg::WriteValue(sender) => {
                    self.write_value(sender)
                }
                BluetoothMethodMsg::Exit => {
                    break
                }
            }
        }
    }

    fn request_device(&self, sender: IpcSender<BluetoothObjectMsg>) {
        let message = BluetoothObjectMsg::BluetoothDevice {
            id: String::from("ca:fe:c0:ff:ee:00"),
            name: String::from("BLE Device (Mock)"),
            device_class: 010918,
            vendor_id_source: String::from("bluetooth"),
            vendor_id: 29,
            product_id: 4608,
            product_version: 5174,
            appearance: 832,
            tx_power: 40,
            rssi: -65
        };
        sender.send(message).unwrap();
    }

    pub fn gatt_server_connect(&self, _sender: IpcSender<BluetoothObjectMsg>) {
        unimplemented!()
    }

    pub fn get_primary_service(&self, sender: IpcSender<BluetoothObjectMsg>) {
        let message = BluetoothObjectMsg::BluetoothService {
            uuid: String::from("0000180f-0000-1000-8000-00805f9b34fb"),
            is_primary: true
        };
        sender.send(message).unwrap();
    }

    pub fn get_characteristic(&self, sender: IpcSender<BluetoothObjectMsg>) {
        let message = BluetoothObjectMsg::BluetoothCharacteristic {
            uuid: String::from("00002a19-0000-1000-8000-00805f9b34fb"),
            broadcast: true,
            read: true,
            write_without_response: false,
            write: false,
            notify: false,
            indicate: false,
            authenticated_signed_writes: false,
            reliable_write: false,
            writable_auxiliaries: false
        };
        sender.send(message).unwrap();
    }

    pub fn get_descriptor(&self, sender: IpcSender<BluetoothObjectMsg>) {
        let message = BluetoothObjectMsg::BluetoothDescriptor {
            uuid: String::from("00002902-0000-1000-8000-00805f9b34fb")
        };
        sender.send(message).unwrap();
    }

    pub fn read_value(&self, _sender: IpcSender<BluetoothObjectMsg>) {
        unimplemented!()
    }

    pub fn write_value(&self, _sender: IpcSender<BluetoothObjectMsg>) {
        unimplemented!()
    }
}
