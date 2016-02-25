/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use device::bluetooth::BluetoothAdapter as BTAdapter;
use device::bluetooth::BluetoothDevice as BTDevice;
use dom::bindings::cell::DOMRefCell;
use dom::bindings::codegen::Bindings::BluetoothDeviceBinding::{VendorIDSource};
use dom::bindings::codegen::Bindings::BluetoothGATTRemoteServerBinding;
use dom::bindings::codegen::Bindings::BluetoothGATTRemoteServerBinding::BluetoothGATTRemoteServerMethods;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{JS, Root};
use dom::bindings::reflector::{Reflectable, Reflector, reflect_dom_object};
use dom::bluetoothdevice::BluetoothDevice;
use dom::bluetoothgattservice::BluetoothGATTService;
use std::cell::Cell;
use util::str::DOMString;
use uuid::Uuid;

// https://webbluetoothcg.github.io/web-bluetooth/#bluetoothremotegattserver
#[dom_struct]
pub struct BluetoothGATTRemoteServer {
    reflector_: Reflector,
    device: DOMRefCell<Option<JS<BluetoothDevice>>>,
    connected: Cell<bool>,
}

impl BluetoothGATTRemoteServer {
    pub fn new_inherited(device: Option<&BluetoothDevice>, is_connected: bool) -> BluetoothGATTRemoteServer {
        BluetoothGATTRemoteServer {
            reflector_: Reflector::new(),
            device: DOMRefCell::new(device.map(JS::from_ref)),
            connected: Cell::new(is_connected),
        }
    }

    pub fn new(global: GlobalRef, device: Option<&BluetoothDevice>, connected: bool) -> Root<BluetoothGATTRemoteServer> {
        reflect_dom_object(box BluetoothGATTRemoteServer::new_inherited(
                           device,
                           connected),
        global,
        BluetoothGATTRemoteServerBinding::Wrap)
    }
}

impl BluetoothGATTRemoteServerMethods for BluetoothGATTRemoteServer {

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothgattremoteserver-device
    fn GetDevice(&self) -> Option<Root<BluetoothDevice>> {
        if let Some(ref is_device) = self.device.borrow().clone() {
            Some(Root::from_ref(&*is_device))
        } else {
            None
        }
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothgattremoteserver-connected
    fn Connected(&self) -> bool {
        self.connected.get()
    }
    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothremotegattserver-connect
    fn Connect(&self) -> () {
        if !self.connected.get() {
            self.connected.set(true);
        }
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothgattremoteserver-disconnect
    fn Disconnect(&self) -> () {
        self.connected.set(!self.Connected());
        //  FIXME (zakorgy)
            ()
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#bluetoothremotegattserver
    // FIXME: In the spec there is no example for this method!(zakorgy)
    fn SetDevice(&self, device: &BluetoothDevice) {
        *self.device.borrow_mut() = Some(JS::from_ref(device));
    }
    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothremotegattserver-getprimaryservice
    fn GetPrimaryService(&self) -> Root<BluetoothGATTService> {
        let adapter: BTAdapter = BTAdapter::create_adapter();
        let devices: Vec<BTDevice> = adapter.get_devices();
        let device: &BTDevice = devices.get(0).unwrap();
        let btdevice = BluetoothDevice::new(self.global().r(),
                             DOMString::from(device.get_address()),
                             DOMString::from(device.get_name()),
                             None,
                             device.get_class(),
                             VendorIDSource::Bluetooth,
                             device.get_vendor_id(),
                             device.get_product_id(),
                             device.get_device_id(),
                             None,
                             );
        let uuid: Uuid = Uuid::new_v4(); //<- ennek a mezőnek kellene valahogy értéket adni
                                         //Stringet tudunk castolni BluetoothUUID::GetService()-el

        BluetoothGATTService::new(self.global().r(),
                                  Some(&*btdevice),
                                  true,
                                  uuid)
    }
}
