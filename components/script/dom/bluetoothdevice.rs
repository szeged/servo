/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::BluetoothDeviceBinding;
use dom::bindings::codegen::Bindings::BluetoothDeviceBinding::{BluetoothDeviceMethods, VendorIDSource};
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{JS, Root};
use dom::bindings::reflector::{Reflector, reflect_dom_object};
use dom::bluetoothadvertisingdata::BluetoothAdvertisingData;
use dom::bluetoothgattremoteserver::BluetoothGATTRemoteServer;
use util::str::DOMString;

// https://webbluetoothcg.github.io/web-bluetooth/#bluetoothdevice

pub type Uuid = DOMString;

#[dom_struct]
pub struct BluetoothDevice {
    reflector_: Reflector,
    id: DOMString,
    name: DOMString,
    //adData: JS<BluetoothAdvertisingData>,
    deviceClass: u32,
    vendorIDSource: VendorIDSource,
    vendorID: u32,
    productID: u32,
    productVersion: u32,
    //gattServer: Option<JS<BluetoothGATTRemoteServer>>,
    //TODO:uuids: Vec<u32>,
}

impl BluetoothDevice {
    pub fn new_inherited(id: DOMString,
                         name: DOMString,
                         //adData: &BluetoothAdvertisingData,
                         deviceClass: u32,
                         vendorIDSource: VendorIDSource,
                         vendorID: u32,
                         productID: u32,
                         productVersion: u32
                         //gattServer: Option<&BluetoothGATTRemoteServer>
                         )
                         -> BluetoothDevice {
        BluetoothDevice {
            reflector_: Reflector::new(),
            id: id,
            name: name,
            //adData: JS::from_ref(adData),
            deviceClass: deviceClass,
            vendorIDSource: vendorIDSource,
            vendorID: vendorID,
            productID: productID,
            productVersion: productVersion,
            //gattServer: gattServer.map(JS::from_ref),
        }
    }

    pub fn new(global: GlobalRef,
             id: DOMString,
             name: DOMString,
             //adData: &BluetoothAdvertisingData,
             deviceClass: u32,
             vendorIDSource: VendorIDSource,
             vendorID: u32,
             productID: u32,
             productVersion: u32,
             //gattServer: Option<&BluetoothGATTRemoteServer>
             )
             -> Root<BluetoothDevice> {
        reflect_dom_object(box BluetoothDevice::new_inherited(id,
                                                              name,
                                                              //adData,
                                                              deviceClass,
                                                              vendorIDSource,
                                                              vendorID,
                                                              productID,
                                                              productVersion
                                                              //gattServer
                                                              ),
                           global,
                           BluetoothDeviceBinding::Wrap)
    }
}

impl BluetoothDeviceMethods for BluetoothDevice {
     // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothdevice-id
    fn Id(&self) -> DOMString {
        self.id.clone()
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothdevice-name
    fn GetName(&self) -> Option<DOMString> {
        Some(self.name.clone())
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#addata
    /*fn AdData(&self) -> Root<BluetoothAdvertisingData> {
        Root::from_ref(&*self.adData)
    }*/

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothdevice-deviceclass
    fn GetDeviceClass(&self) -> Option<u32> {
        Some(self.deviceClass)
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothdevice-vendoridsource
    fn GetVendorIDSource(&self) -> Option<VendorIDSource> {
        Some(self.vendorIDSource)
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothdevice-vendorid
    fn GetVendorID(&self) -> Option<u32> {
        Some(self.vendorID)
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothdevice-productid
    fn GetProductID(&self) -> Option<u32> {
        Some(self.productID)
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothdevice-productversion
    fn GetProductVersion(&self) -> Option<u32> {
        Some(self.productVersion)
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#gattserver
    /*fn GetGattServer(&self) -> Option<Root<BluetoothGATTRemoteServer>> {
        if let Some(ref is_server) = self.gattServer.clone() {
            Some(Root::from_ref(&is_server))
        } else {
            None
        }
    }*/
}
