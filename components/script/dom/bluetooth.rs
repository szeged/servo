/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::BluetoothBinding;
use dom::bindings::codegen::Bindings::BluetoothBinding::BluetoothMethods;
use dom::bindings::codegen::Bindings::BluetoothDeviceBinding::VendorIDSource;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::reflector::{Reflectable, Reflector, reflect_dom_object};
use dom::bluetoothadvertisingdata::BluetoothAdvertisingData;
use dom::bluetoothdevice::BluetoothDevice;
use ipc_channel::ipc;
use net_traits::bluetooth_thread::{BluetoothMethodMsg, BluetoothObjectMsg};
use util::str::DOMString;

// https://webbluetoothcg.github.io/web-bluetooth/#bluetooth
#[dom_struct]
pub struct Bluetooth {
    reflector_: Reflector,
}

impl Bluetooth {
    pub fn new_inherited() -> Bluetooth {
        Bluetooth {
            reflector_: Reflector::new(),
        }
    }

    pub fn new(global: GlobalRef) -> Root<Bluetooth> {
        reflect_dom_object(box Bluetooth::new_inherited(),
                           global,
                           BluetoothBinding::Wrap)
    }
}

impl BluetoothMethods for Bluetooth {

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetooth-requestdevice
    fn RequestDevice(&self) -> Option<Root<BluetoothDevice>> {
        let (sender, receiver) = ipc::channel().unwrap();
        self.global().r().as_window().bluetooth_thread().send(BluetoothMethodMsg::RequestDevice(sender)).unwrap();
        let device = receiver.recv().unwrap();
        match device {
            BluetoothObjectMsg::BluetoothDevice {
                id,
                name,
                device_class,
                vendor_id_source,
                vendor_id,
                product_id,
                product_version,
                appearance,
                tx_power,
                rssi,
            } => {
                let ad_data = &BluetoothAdvertisingData::new(self.global().r(),
                                                             appearance,
                                                             tx_power,
                                                             rssi);
                let vendor_id_source = match vendor_id_source.as_ref() {
                    "bluetooth" => VendorIDSource::Bluetooth,
                    "usb" => VendorIDSource::Usb,
                    _ => VendorIDSource::Unknown,
                };
                Some(BluetoothDevice::new(self.global().r(),
                                          DOMString::from(id),
                                          DOMString::from(name),
                                          ad_data,
                                          device_class,
                                          vendor_id_source,
                                          vendor_id,
                                          product_id,
                                          product_version))
            },
            BluetoothObjectMsg::Error {
                error
            } => {
                println!("{}", error);
                None
            },
            _ => unreachable!()
        }
    }
}
