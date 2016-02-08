/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::reflector::{Reflector, reflect_dom_object};
use dom::bluetoothdevice::BluetoothDevice;
use dom::bindings::codegen::Bindings::BluetoothDeviceBinding::{BluetoothDeviceMethods, VendorIDSource};
use dom::bindings::codegen::Bindings::BluetoothBinding;
use dom::bindings::codegen::Bindings::BluetoothBinding::BluetoothMethods;
use util::str::DOMString;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{JS, Root};

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
	fn RequestDevice(&self) -> Root<BluetoothDevice> {
		let nd = newdevice();
		Root::from_ref(&nd)
	}
}
fn newdevice() -> BluetoothDevice {
	let id: DOMString = DOMString::from_string("12321323-213213213-d123asdasd".to_string());
 	let name: DOMString = DOMString::from_string("banana".to_string());
 	let idsource:VendorIDSource = VendorIDSource::Bluetooth;
 	let device: BluetoothDevice = BluetoothDevice::new_inherited(id,name,1,idsource,1,1,1,None);
 	return device
}