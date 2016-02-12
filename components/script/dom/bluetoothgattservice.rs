/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate uuid;
use dom::bindings::codegen::Bindings::BluetoothGATTServiceBinding;
use dom::bindings::codegen::Bindings::BluetoothGATTServiceBinding::BluetoothGATTServiceMethods;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{JS, Root};
use dom::bindings::reflector::{Reflector, reflect_dom_object};
use dom::bluetoothdevice::BluetoothDevice;
use util::str::DOMString;
use dom::bindings::codegen::UnionTypes::StringOrUnsignedLong::{eString, eUnsignedLong};
use dom::bluetoothuuid;
use uuid::Uuid;
use dom::bindings::cell::DOMRefCell;

#[dom_struct]
pub struct BluetoothGATTService {
    reflector_: Reflector,
    device: DOMRefCell<Option<JS<BluetoothDevice>>>,
    isPrimary: bool,
    uuid: Uuid,
}

impl BluetoothGATTService {
    pub fn new_inherited(device: Option<&BluetoothDevice>, isPrimary: bool, uuid: Uuid) -> BluetoothGATTService {
        BluetoothGATTService {
            reflector_: Reflector::new(),
            device: DOMRefCell::new(device.map(JS::from_ref)),
            isPrimary: isPrimary,
            uuid: uuid,
        }
    }

    pub fn new(global: GlobalRef, device: Option<&BluetoothDevice>, isPrimary: bool, uuid: Uuid) -> Root<BluetoothGATTService> {
        reflect_dom_object(box BluetoothGATTService::new_inherited(
                           device,
                           isPrimary,
                           uuid),
        global,
        BluetoothGATTServiceBinding::Wrap)
    }
}

impl BluetoothGATTServiceMethods for BluetoothGATTService {
    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothgattservice-device
    fn GetDevice(&self) -> Option<Root<BluetoothDevice>> {
        if let Some(ref is_device) = self.device.borrow().clone() {
            Some(Root::from_ref(&*is_device))
        } else {
            None
        }
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothgattservice-uuid
    fn IsPrimary(&self) -> bool {
        self.isPrimary
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothgattservice-uuid
    fn Uuid(&self) -> DOMString {
        DOMString::from_string(self.uuid.to_simple_string().clone())
    }

    fn SetDevice(&self, device: &BluetoothDevice){
        *self.device.borrow_mut() = Some(JS::from_ref(device));
    }
}
