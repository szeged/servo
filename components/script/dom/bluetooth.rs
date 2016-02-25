/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use device::bluetooth::BluetoothAdapter as BTAdapter;
use device::bluetooth::BluetoothDevice as BTDevice;
use dom::bindings::codegen::Bindings::BluetoothBinding;
use dom::bindings::codegen::Bindings::BluetoothBinding::BluetoothMethods;
use dom::bindings::codegen::Bindings::BluetoothDeviceBinding::{VendorIDSource};
use dom::bindings::codegen::UnionTypes::StringOrUnsignedLong;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::reflector::{Reflector, Reflectable, reflect_dom_object};
use dom::bluetoothdevice::BluetoothDevice;
use dom::bluetoothuuid::BluetoothUUID;
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
    fn RequestDevice(&self) -> Root<BluetoothDevice> {
        let adapter: BTAdapter = BTAdapter::create_adapter();
        let devices: Vec<BTDevice> = adapter.get_devices();
        let device: &BTDevice = devices.get(0).unwrap();
        BluetoothDevice::new(self.global().r(),
                             DOMString::from(device.get_address()),
                             DOMString::from(device.get_name()),
                             None,
                             device.get_class(),
                             VendorIDSource::Bluetooth,
                             device.get_vendor_id(),
                             device.get_product_id(),
                             device.get_device_id(),
                             None,
                             )
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetooth-requestdevice
    // FIXME: This method is only for test purposes!(zakorgy)
    fn CreateDevice(&self,
                    device_id: DOMString,
                    device_name: DOMString,
                    //ad_data: None
                    device_class: u32,
                    vendor_id_source_string: DOMString,
                    vendor_id: u32,
                    product_id: u32,
                    product_version: u32
                    //gattServer: None
                    ) -> Root<BluetoothDevice> {
        let vendor_id_source = match vendor_id_source_string.trim() {
            "Bluetooth" => VendorIDSource::Bluetooth,
            "Usb" => VendorIDSource::Usb,
            _ => panic!("Wrong type of vendor id seource!")
        };

        Root::from_ref(&BluetoothDevice::new(self.global().r(),
                                             device_id,
                                             device_name,
                                             None,
                                             device_class,
                                             vendor_id_source,
                                             vendor_id,
                                             product_id,
                                             product_version,
                                             None
                                             ))
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#example-d5112950
    fn CanonicalTest(&self, alias: u32) -> DOMString {
        BluetoothUUID::CanonicalUUID(self.global().r(), alias)
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#example-d5112950
    fn GetServiceTest(&self, name: StringOrUnsignedLong) -> DOMString {
        BluetoothUUID::GetService(self.global().r(), name)
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#example-d5112950
    fn GetCharacteristicTest(&self, name: StringOrUnsignedLong) -> DOMString {
        BluetoothUUID::GetCharacteristic(self.global().r(), name)
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#example-d5112950
    fn GetDescriptorTest(&self, name: StringOrUnsignedLong) -> DOMString {
        BluetoothUUID::GetDescriptor(self.global().r(), name)
    }
}
