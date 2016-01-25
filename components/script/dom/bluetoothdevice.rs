/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bluetoothadvertisingdata::BluetoothAdvertisingData;
use dom::bindings::codegen::Bindings::BluetoothDeviceBinding;
use dom::bindings::codegen::Bindings::BluetoothDeviceBinding::{BluetoothDeviceMethods, VendorIDSource};
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{JS, Root};
use dom::bindings::reflector::{Reflector, reflect_dom_object};
use util::str::DOMString;

// https://webbluetoothcg.github.io/web-bluetooth/#bluetoothdevice

#[dom_struct]
pub struct BluetoothDevice {
    reflector_: Reflector,
    id: DOMString,
    name: DOMString,
    adData: JS<BluetoothAdvertisingData>,
    deviceClass: u32,
    vendorIDSource: VendorIDSource,
    vendorID: u32,
    productID: u32,
    productVersion: u32,
    // gattServer: BluetoothGATTRemoteServer,
    // uuids: UUID[],
}

impl BluetoothDevice {
    pub fn new_inherited(id: DOMString,
                         name: DOMString,
                         adData: &BluetoothAdvertisingData,
                         deviceClass: u32,
                         vendorIDSource: VendorIDSource,
                         vendorID: u32,
                         productID: u32,
                         productVersion: u32)
                         -> BluetoothDevice {
        BluetoothDevice {
            reflector_: Reflector::new(),
            id: id,
            name: name,
            adData: JS::from_ref(adData),
            deviceClass: deviceClass,
            vendorIDSource: vendorIDSource,
            vendorID: vendorID,
            productID: productID,
            productVersion: productVersion,
        }
    }

    pub fn new(global: GlobalRef,
             id: DOMString,
             name: DOMString,
             adData: &BluetoothAdvertisingData,
             deviceClass: u32,
             vendorIDSource: VendorIDSource,
             vendorID: u32,
             productID: u32,
             productVersion: u32)
             -> Root<BluetoothDevice> {
        reflect_dom_object(box BluetoothDevice::new_inherited(id,
                                                              name,
                                                              adData,
                                                              deviceClass,
                                                              vendorIDSource,
                                                              vendorID,
                                                              productID,
                                                              productVersion),
                           global,
                           BluetoothDeviceBinding::Wrap)
    }
}

impl BluetoothDeviceMethods for BluetoothDevice {
    fn Id(&self) -> DOMString {
        self.id.clone()
    }

    fn GetName(&self) -> Option<DOMString> {
        Some(self.name.clone())
    }

    fn AdData(&self) -> Root<BluetoothAdvertisingData> {
        Root::from_ref(&*self.adData)
    }

    fn GetDeviceClass(&self) -> Option<u32> {
        Some(self.deviceClass)
    }

    fn GetVendorIDSource(&self) -> Option<VendorIDSource> {
        Some(self.vendorIDSource)
    }

    fn GetVendorID(&self) -> Option<u32> {
        Some(self.vendorID)
    }

    fn GetProductID(&self) -> Option<u32> {
        Some(self.productID)
    }

    fn GetProductVersion(&self) -> Option<u32> {
        Some(self.productVersion)
    }
}
