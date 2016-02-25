/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::BluetoothGATTDescriptorBinding;
use dom::bindings::codegen::Bindings::BluetoothGATTDescriptorBinding::BluetoothGATTDescriptorMethods;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{JS, Root};
use dom::bindings::reflector::{Reflector, reflect_dom_object};
use dom::bluetoothgattcharacteristic::BluetoothGATTCharacteristic;
use util::str::DOMString;
use uuid::Uuid;
use dom::bindings::cell::DOMRefCell;

// http://webbluetoothcg.github.io/web-bluetooth/#bluetoothgattdescriptor

#[dom_struct]
pub struct BluetoothGATTDescriptor {
    reflector_: Reflector,
    characteristic: DOMRefCell<Option<JS<BluetoothGATTCharacteristic>>>,
    uuid: Uuid,
    //value: ArrayBuffer,
}

impl BluetoothGATTDescriptor {
    pub fn new_inherited(characteristic: Option<&BluetoothGATTCharacteristic>,
                         uuid: Uuid)
                         -> BluetoothGATTDescriptor {
        BluetoothGATTDescriptor {
            reflector_: Reflector::new(),
            characteristic: DOMRefCell::new(characteristic.map(JS::from_ref)),
            uuid: uuid,
        }
    }

    pub fn new(global: GlobalRef,
               characteristic: Option<&BluetoothGATTCharacteristic>,
               uuid: Uuid)
               -> Root<BluetoothGATTDescriptor>{
        reflect_dom_object(box BluetoothGATTDescriptor::new_inherited(characteristic,
                                                                      uuid),
                            global,
                            BluetoothGATTDescriptorBinding::Wrap)
    }
}

impl BluetoothGATTDescriptorMethods for BluetoothGATTDescriptor {
    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothgattdescriptor-characteristic
    fn GetCharacteristic(&self) -> Option<Root<BluetoothGATTCharacteristic>> {
        if let Some(ref is_characteristic) = self.characteristic.borrow().clone() {
            Some(Root::from_ref(&*is_characteristic))
        } else {
            None
        }
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothgattdescriptor-uuid
    fn Uuid(&self) -> DOMString {
        DOMString::from_string(self.uuid.to_string())
    }

    fn SetCharacteristic(&self, characteristic: &BluetoothGATTCharacteristic){
        *self.characteristic.borrow_mut() = Some(JS::from_ref(characteristic));
    }
}
