/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::BluetoothBinding;
use dom::bindings::codegen::Bindings::BluetoothBinding::BluetoothMethods;
use dom::bindings::codegen::Bindings::BluetoothDeviceBinding::{VendorIDSource};
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{JS, Root};
use dom::bindings::reflector::{Reflectable, Reflector, reflect_dom_object};
use dom::bluetoothadvertisingdata::BluetoothAdvertisingData;
use dom::bluetoothcharacteristicproperties::BluetoothCharacteristicProperties;
use dom::bluetoothdevice::BluetoothDevice;
use dom::bluetoothgattcharacteristic::BluetoothGATTCharacteristic;
use dom::bluetoothgattdescriptor::BluetoothGATTDescriptor;
use dom::bluetoothgattremoteserver::BluetoothGATTRemoteServer;
use dom::bluetoothgattservice::BluetoothGATTService;
use util::str::DOMString;
use uuid::Uuid;

use blurz::bluetooth_adapter::BluetoothAdapter as BTAdapter;
use blurz::bluetooth_device::BluetoothDevice as BTDevice;

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
                                                            69_i8)),
    mockProperties: JS::from_ref(&BluetoothCharacteristicProperties::new(global,
                                                                         true,
                                                                         true,
                                                                         false,
                                                                         true,
                                                                         false,
                                                                         true,
                                                                         false,
                                                                         true,
                                                                         false)),
    mockServer: JS::from_ref(&BluetoothGATTRemoteServer::new(global,
                                                             None,
                                                             true)),
    mockCharacteristic: JS::from_ref(&BluetoothGATTCharacteristic::new(global,
                                                                       None,
                                                                       Uuid::new_v4(),
                                                                       None)),
    mockDescriptor: JS::from_ref(&BluetoothGATTDescriptor::new(global,
                                                               None,
                                                               Uuid::new_v4())),
    mockService: JS::from_ref(&BluetoothGATTService::new(global,
                                                         None,
                                                         true,
                                                         Uuid::new_v4())),
        }
    }

    pub fn new(global: GlobalRef) -> Root<Bluetooth> {
        reflect_dom_object(box Bluetooth::new_inherited(global),
                           global,
                           BluetoothBinding::Wrap)
    }
}

impl BluetoothMethods for Bluetooth {
//https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetooth-requestdevice
    /*fn RequestMockDevice(&self) -> Root<BluetoothDevice> {
        Root::from_ref(&*self.mockDevice)
    }*/

    fn RequestDevice(&self) -> Root<BluetoothDevice> {
        let adapter: BTAdapter = BTAdapter::init().unwrap();
        let device: BTDevice = adapter.get_first_device().unwrap();
        BluetoothDevice::new(self.global().r(),
                             DOMString::from(device.address()),
                             DOMString::from(device.name()),
                             None,
                             device.class(),
                             VendorIDSource::Bluetooth,
                             device.vendor_id(),
                             device.product_id(),
                             device.product_version(),
                             None,
                             )
    }
}
