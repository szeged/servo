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

// https://webbluetoothcg.github.io/web-bluetooth/#bluetoothgattcharacteristic

#[dom_struct]
pub struct BluetoothGATTCharacteristic {
    reflector_: Reflector,
    properties: JS<BluetoothCharacteristicProperties>,
    service: JS<BluetoothGATTService>,
    uuid: Uuid,
    //value :ArrayBuffer,
}

impl BluetoothGATTCharacteristic {
    pub fn new_inherited(service: &BluetoothGATTService,
                        uuid: Uuid,
                        properties: &BluetoothCharacteristicProperties,
                        ) -> BluetoothGATTCharacteristic {
        BluetoothGATTCharacteristic {
            reflector_: Reflector::new(),
            service: JS::from_ref(service),
            uuid: uuid,
            properties: JS::from_ref(properties),
        }
    }

    pub fn new(global: GlobalRef,
            service: &BluetoothGATTService,
            uuid: Uuid,
            properties: &BluetoothCharacteristicProperties,
            ) -> Root<BluetoothGATTCharacteristic>{
        reflect_dom_object(box BluetoothGATTCharacteristic::new_inherited(service,
                                                                        uuid,
                                                                        properties,
                                                                        ),
                            global,
                            BluetoothGATTCharacteristicBinding::Wrap)
    }
}

impl BluetoothGATTCharacteristicMethods for BluetoothGATTCharacteristic {
    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothgattcharacteristic-properties
    fn Properties(&self) -> Root<BluetoothCharacteristicProperties> {
        Root::from_ref(&*self.properties)
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothgattcharacteristic-service
    fn Service(&self) -> Root<BluetoothGATTService> {
        Root::from_ref(&*self.service)
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothgattcharacteristic-uuid
    fn Uuid(&self) -> DOMString {
        DOMString::from_string(self.uuid.to_string().clone())
    }
}
