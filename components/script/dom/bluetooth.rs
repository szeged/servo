/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate uuid;
use dom::bindings::reflector::{Reflector, reflect_dom_object};
use dom::bluetoothdevice::BluetoothDevice;
use dom::bindings::codegen::Bindings::BluetoothBinding;
use dom::bindings::codegen::Bindings::BluetoothBinding::BluetoothMethods;
use dom::bindings::codegen::Bindings::BluetoothDeviceBinding;
use dom::bindings::codegen::Bindings::BluetoothDeviceBinding::{BluetoothDeviceMethods, VendorIDSource};
use util::str::DOMString;
use dom::bindings::global::{GlobalRef, global_root_from_reflector, global_root_from_object};
use dom::bindings::js::{JS, Root};
use dom::bluetoothgattremoteserver::BluetoothGATTRemoteServer;
//use uuid::Uuid;
//use std::mem;
use std::cmp::Ordering;

#[dom_struct]
pub struct Bluetooth {
    reflector_: Reflector,
    mockDevice: JS<BluetoothDevice>,
}

impl Bluetooth {
    pub fn new_inherited(global: GlobalRef) -> Bluetooth {
        Bluetooth {
        reflector_: Reflector::new(),
        mockDevice: JS::from_ref(&BluetoothDevice::new(global,
                                                       DOMString::from("DeviceID"),
                                                       DOMString::from("DeviceName"),
                                                       342_u32,
                                                       VendorIDSource::Bluetooth,
                                                       8543_u32,
                                                       1100_u32,
                                                       200_u32,
                                                       )),
        }
    }

    pub fn new(global: GlobalRef) -> Root<Bluetooth> {
        reflect_dom_object(box Bluetooth::new_inherited(global),
                           global,
                           BluetoothBinding::Wrap)
    }

    pub fn request_device(&self,
                          nameFilter: DOMString,
                          namePrefixFilter: DOMString
                          ) -> DOMString {
        let mut rvDOMString = DOMString::new();
        if nameFilter.is_empty() && namePrefixFilter.is_empty() {
            rvDOMString = DOMString::from("Error:empty nameFilter and namePrefixFilter");
        } else if nameFilter.is_empty() {
            rvDOMString = DOMString::from("Namefilter is empty");
        } else if namePrefixFilter.is_empty() {
            rvDOMString = DOMString::from("NamePrefixFilter is empty!");
        } else {
            rvDOMString = DOMString::from("NameFilter and namePrefixFilter both has a value!");
        }

        rvDOMString
    }
}


impl BluetoothMethods for Bluetooth {   
    fn RequestDevice(&self) -> Root<BluetoothDevice> {       
        Root::from_ref(&*self.mockDevice)
    }
}
