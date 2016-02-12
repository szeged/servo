/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::BluetoothGATTCharacteristicBinding;
use dom::bindings::codegen::Bindings::BluetoothGATTCharacteristicBinding::BluetoothGATTCharacteristicMethods;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{JS, Root};
use dom::bindings::reflector::{Reflector, reflect_dom_object};
use dom::bluetoothcharacteristicproperties::BluetoothCharacteristicProperties;
use dom::bluetoothgattservice::BluetoothGATTService;
use util::str::DOMString;
use uuid::Uuid;
use dom::bindings::cell::DOMRefCell;

// https://webbluetoothcg.github.io/web-bluetooth/#bluetoothgattcharacteristic

#[dom_struct]
pub struct BluetoothGATTCharacteristic {
    reflector_: Reflector,
    properties: DOMRefCell<Option<JS<BluetoothCharacteristicProperties>>>,
    service: DOMRefCell<Option<JS<BluetoothGATTService>>>,
    uuid: Uuid,
    //value :ArrayBuffer,
}

impl BluetoothGATTCharacteristic {
    pub fn new_inherited(service: Option<&BluetoothGATTService>,
                         uuid: Uuid,
                         properties: Option<&BluetoothCharacteristicProperties>)
                         -> BluetoothGATTCharacteristic {
        BluetoothGATTCharacteristic {
            reflector_: Reflector::new(),
            service: DOMRefCell::new(service.map(JS::from_ref)),
            uuid: uuid,
            properties: DOMRefCell::new(properties.map(JS::from_ref)),
        }
    }

    pub fn new(global: GlobalRef,
               service: Option<&BluetoothGATTService>,
               uuid: Uuid,
               properties: Option<&BluetoothCharacteristicProperties>)
               -> Root<BluetoothGATTCharacteristic>{
        reflect_dom_object(box BluetoothGATTCharacteristic::new_inherited(service,
                                                                          uuid,
                                                                          properties
                                                                        ),
                            global,
                            BluetoothGATTCharacteristicBinding::Wrap)
    }
}

impl BluetoothGATTCharacteristicMethods for BluetoothGATTCharacteristic {
    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothgattcharacteristic-properties
    fn GetProperties(&self) -> Option<Root<BluetoothCharacteristicProperties>> {
        if let Some(ref is_prop) = self.properties.borrow().clone() {
            Some(Root::from_ref(&*is_prop))
        } else {
            None
        }
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothgattcharacteristic-service
    fn GetService(&self) -> Option<Root<BluetoothGATTService>> {
        if let Some(ref is_service) = self.service.borrow().clone() {
            Some(Root::from_ref(&*is_service))
        } else {
            None
        }
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothgattcharacteristic-uuid
    fn Uuid(&self) -> DOMString {
        DOMString::from_string(self.uuid.to_string().clone())
    }

    fn SetProperties(&self, properties: &BluetoothCharacteristicProperties){
        *self.properties.borrow_mut() = Some(JS::from_ref(properties));
    }

    fn SetService(&self, service: &BluetoothGATTService){
        *self.service.borrow_mut() = Some(JS::from_ref(service));
    }
}
