/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::global::GlobalRef;
use dom::bindings::reflector::Reflector;
use dom::bindings::codegen::UnionTypes::StringOrUnsignedLong;
//use dom::bindings::codegen::UnionTypes::StringOrUnsignedLong::{eString, eUnsignedLong};
use util::str::DOMString;
//use std::fmt::LowerHex;
use regex::Regex;
//use dom::domexception::{DOMException, DOMErrorName};
//use dom::bindings::js;

// https://webbluetoothcg.github.io/web-bluetooth/#bluetoothuuid

pub type UUID = DOMString;

 #[dom_struct]
pub struct BluetoothUUID {
    reflector_: Reflector,
}


const BLUETOOTH_ASSIGNED_SERVICES: &'static [(&'static str,u32)] = &[
    //TODO(zakorgy) create all the services 
    //https://developer.bluetooth.org/gatt/services/Pages/ServicesHome.aspx
    ("org.bluetooth.service.alert_notification",0x1811u32),
    ("org.bluetooth.service.automation_io",0x1815u32),
    ];

const BLUETOOTH_ASSIGNED_CHARCTERISTICS: &'static [(&'static str,u32)] = &[
    //TODO(zakorgy) create all the characteristics 
    //https://developer.bluetooth.org/gatt/services/Pages/ServicesHome.aspx
    ("org.bluetooth.service.alert_notification",0x1811u32),
    ("org.bluetooth.service.automation_io",0x1815u32),
    ];

const BLUETOOTH_ASSIGNED_DESCRIPTORS: &'static [(&'static str,u32)] = &[
    //TODO(zakorgy) create all the services 
    //https://developer.bluetooth.org/gatt/services/Pages/ServicesHome.aspx
    ("org.bluetooth.service.alert_notification",0x1811u32),
    ("org.bluetooth.service.automation_io",0x1815u32),
    ];


impl BluetoothUUID {
    pub fn CanonicalUUID(_: GlobalRef, _alias: u32) -> UUID {
        let mut base_uuid = DOMString::from("00000000-0000-1000-8000-00805f9b34fb");
        base_uuid =  DOMString::from_string(base_uuid.
                                           replace("00000000",&*format!("{:08x}",&_alias)));
        base_uuid
    }

    pub fn GetService(globalref: GlobalRef,
                      name: StringOrUnsignedLong)
                      -> UUID {
    	BluetoothUUID::ResolveUUIDName(globalref,
                                       name,
                                       BLUETOOTH_ASSIGNED_SERVICES,
                                       DOMString::from("org.bluetooth.service") )
    }

    pub fn GetCharacteristic(globalref: GlobalRef,
                      name: StringOrUnsignedLong)
                      -> UUID {
        BluetoothUUID::ResolveUUIDName(globalref,
                                       name,
                                       BLUETOOTH_ASSIGNED_CHARCTERISTICS,
                                       DOMString::from("org.bluetooth.service") )
    }


    pub fn GetDescriptor(globalref: GlobalRef,
                      name: StringOrUnsignedLong)
                      -> UUID {
        BluetoothUUID::ResolveUUIDName(globalref,
                                       name,
                                       BLUETOOTH_ASSIGNED_DESCRIPTORS,
                                       DOMString::from("org.bluetooth.service") )
    }

    pub fn ResolveUUIDName(globalref: GlobalRef,
                           name: StringOrUnsignedLong,
                           assigned_numbers_table: &'static [(&'static str,u32)],
                           prefix: DOMString 
                           ) -> DOMString {

        match name {
            StringOrUnsignedLong::eUnsignedLong(unsigned32) =>{
                BluetoothUUID::CanonicalUUID(globalref, unsigned32)
            },
            StringOrUnsignedLong::eString(dstring) => {
                let regex = Regex::new("^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}")
                                       .unwrap();
                if regex.is_match(&*dstring) {
                    dstring
                } else {
                    let concatenated = format!("{}.{}",dstring,prefix);
                    let mut is_in_table = false;
                    let mut service_number:u32 = 0x00000000_u32;
                    for service in assigned_numbers_table {

                        if service.0 == concatenated {
                            is_in_table = true;
                            service_number = service.1;
                        }
                    }
                    if is_in_table {
                        BluetoothUUID::CanonicalUUID(globalref, service_number)
                    } else {
                        DOMString::from("The string did not match the expected pattern.")
                    }
                }
            },                
        }
    }
}
