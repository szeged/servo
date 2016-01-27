/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::BluetoothGATTCharacteristicBinding;
use dom::bindings::codegen::Bindings::BluetoothGATTCharacteristicBinding::BluetoothGATTCharacteristicMethods;
use dom::bindings::reflector::{Reflector, reflect_dom_object};
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{JS,Root};
//use dom::bluetoothuuid::BluetoothUUID;
//use dom::bluetoothgattservice:BluetoothGATTService;
use dom::bluetoothcharacteristicproperties::BluetoothCharacteristicProperties;
use uuid::Uuid;
use util::str::DOMString;

// https://webbluetoothcg.github.io/web-bluetooth/#bluetoothgattcharacteristic

#[dom_struct]
pub struct BluetoothGATTCharacteristic {
    reflector_: Reflector,
    properties : JS<BluetoothCharacteristicProperties>,
	//service : JS<BluetoothGATTService>,
	uuid : Uuid,
	//value : ArrayBuffer,
}

impl BluetoothGATTCharacteristic {
	pub fn new_inherited(//service: &BluetoothGATTService,
						uuid: Uuid,
						properties: &BluetoothCharacteristicProperties,
						//alue: ArrayBuffer,
						)->BluetoothGATTCharacteristic {
		BluetoothGATTCharacteristic{
			reflector_: Reflector::new(),
			//service: JS::from_ref(service),
			uuid: uuid,
			properties: JS::from_ref(properties),
			//value: value,
		}
	}

	pub fn new(global: GlobalRef,
			//service: &BluetoothGATTService,
			uuid: Uuid,
			properties: &BluetoothCharacteristicProperties,
			//value: ArrayBuffer,
			) -> Root<BluetoothGATTCharacteristic>{
		reflect_dom_object(box BluetoothGATTCharacteristic::new_inherited(//service,
																		uuid,
																		properties,
																		//value,
																		),
							global,
							BluetoothGATTCharacteristicBinding::Wrap)
	}
}

impl BluetoothGATTCharacteristicMethods for BluetoothGATTCharacteristic {

	fn Properties(&self) -> Root<BluetoothCharacteristicProperties> {
    	Root::from_ref(&*self.properties)
    }

    fn Uuid(&self) -> DOMString {
   		DOMString::from_string(self.uuid.to_string())
    }
}