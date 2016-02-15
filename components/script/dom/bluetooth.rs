/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::reflector::{Reflector, reflect_dom_object};
use dom::bluetoothdevice::BluetoothDevice;
use dom::bindings::codegen::Bindings::BluetoothDeviceBinding::{VendorIDSource};
use dom::bindings::codegen::Bindings::BluetoothBinding;
use dom::bindings::codegen::Bindings::BluetoothBinding::BluetoothMethods;
use util::str::DOMString;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{JS, Root};
use dom::bluetoothgattremoteserver::BluetoothGATTRemoteServer;
use dom::bluetoothadvertisingdata::BluetoothAdvertisingData;
use dom::bluetoothcharacteristicproperties::BluetoothCharacteristicProperties;
use dom::bluetoothgattcharacteristic::BluetoothGATTCharacteristic;
use dom::bluetoothgattdescriptor::BluetoothGATTDescriptor;
use dom::bluetoothgattservice::BluetoothGATTService;
use uuid::Uuid;
use dom::window::Window;

lazy_static! {
    pub static ref GLOBALREF: Option<&'static Window> = None;
}


#[dom_struct]
pub struct Bluetooth {
    reflector_: Reflector,
    mockDevice: JS<BluetoothDevice>,
    mockServer: JS<BluetoothGATTRemoteServer>,
    mockAdData: JS<BluetoothAdvertisingData>,
    mockProperties: JS<BluetoothCharacteristicProperties>,
    mockCharacteristic: JS<BluetoothGATTCharacteristic>,
    mockDescriptor: JS<BluetoothGATTDescriptor>,
    mockService: JS<BluetoothGATTService>,
}

impl Bluetooth {
	pub fn new_inherited(global: GlobalRef) -> Bluetooth {
		Bluetooth {
		reflector_: Reflector::new(),
		mockDevice: JS::from_ref(&BluetoothDevice::new(global,
                                                       DOMString::from("DeviceID"),
                                                       DOMString::from("DeviceName"),
                                                       None,
                                                       342_u32,
                                                       VendorIDSource::Bluetooth,
                                                       8543_u32,
                                                       1100_u32,
                                                       200_u32,
                                                       None,
                                                       )),
    mockAdData: JS::from_ref(&BluetoothAdvertisingData::new(global,
                                                            1234_u16,
                                                            13_i8,
                                                            69_i8,)),
    mockProperties: JS::from_ref(&BluetoothCharacteristicProperties::new(global,
                                                                         true,
                                                                         true,
                                                                         false,
                                                                         true,
                                                                         false,
                                                                         true,
                                                                         false,
                                                                         true,
                                                                         false,)),
    mockServer: JS::from_ref(&BluetoothGATTRemoteServer::new(global,
                                                             None,
                                                             true,)),
    mockCharacteristic: JS::from_ref(&BluetoothGATTCharacteristic::new(global,
                                                                       None,
                                                                       Uuid::new_v4(),
                                                                       None,)),
    mockDescriptor: JS::from_ref(&BluetoothGATTDescriptor::new(global,
                                                               None,
                                                               Uuid::new_v4(),)),
    mockService: JS::from_ref(&BluetoothGATTService::new(global,
                                                         None,
                                                         true,
                                                         Uuid::new_v4(),)),
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
