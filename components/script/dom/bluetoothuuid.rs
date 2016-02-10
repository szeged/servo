/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::global::GlobalRef;
use dom::bindings::reflector::Reflector;
use dom::bindings::codegen::UnionTypes::StringOrUnsignedLong;
use dom::bindings::codegen::UnionTypes::StringOrUnsignedLong::{eString, eUnsignedLong};
use util::str::DOMString;
use std::fmt::LowerHex;
use regex::Regex;
use dom::domexception::{DOMException, DOMErrorName};
use dom::bindings::js::{JS, Root};

// https://webbluetoothcg.github.io/web-bluetooth/#bluetoothuuid

pub type UUID = DOMString;

 #[dom_struct]
pub struct BluetoothUUID {
    reflector_: Reflector,
}


const ASSIGNED_NUMBERS_TABLE: &'static [(&'static str,u32)] = &[
    //TODO(zakorgy) create all the services 
    //https://developer.bluetooth.org/gatt/services/Pages/ServicesHome.aspx
    ("org.bluetooth.service.alert_notification",0x1811u32),
    ("org.bluetooth.service.automation_io",0x1815u32),
    ];

impl BluetoothUUID {
    //Fixme(zakorgy) a formátumot átállítai 8 számjegyűre beillesztés előtt a format-ban!!!!
    pub fn CanonicalUUID(_: GlobalRef, _alias: u32) -> UUID {
        let mut base_uuid = DOMString::from("00000000-0000-1000-8000-00805f9b34fb");
        base_uuid =  DOMString::from_string(base_uuid.
                                           replace("00000000",&*format!("{:x}",&_alias)));
        base_uuid
    }

    pub fn GetDescriptor(_: GlobalRef, name: StringOrUnsignedLong) -> UUID {
    	match name {
            StringOrUnsignedLong::eString(s) => s,
            StringOrUnsignedLong::eUnsignedLong(u) => DOMString::new(),
        }
    }

    pub fn GetService(_: GlobalRef, name: StringOrUnsignedLong) -> UUID {
    	match name {
            StringOrUnsignedLong::eString(s) => s,
            StringOrUnsignedLong::eUnsignedLong(u) => DOMString::new(),
        }
    }

    pub fn GetCharacteristic(_: GlobalRef, name: StringOrUnsignedLong) -> UUID {
        match name {
            StringOrUnsignedLong::eString(s) => s,
            StringOrUnsignedLong::eUnsignedLong(u) => DOMString::new(),
        }
    }

    pub fn ResolveUUIDName(globalref: GlobalRef,
                           name: StringOrUnsignedLong,
                           assigned_numbers_table: Vec<&str>,
                           prefix: DOMString 
                           ) -> Result<UUID, Root<DOMException>> {

        match name {
            StringOrUnsignedLong::eUnsignedLong(unsigned32) =>{
                Ok(BluetoothUUID::CanonicalUUID(globalref, unsigned32))
            },
            StringOrUnsignedLong::eString(dstring) => {
                let regex = Regex::new("^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}")
                                       .unwrap();
                if regex.is_match(&*dstring) {

                    Ok(dstring)

                } else {
                    let concatenated = format!("{}.{}",dstring,prefix);

                    let mut is_in_table = false;
                    let mut service_number:u32 = 0x00000000_u32;

                    for service in ASSIGNED_NUMBERS_TABLE {

                        if service.0 == concatenated {
                            is_in_table = true;
                            service_number = service.1;
                        }
                    }

                    if is_in_table {
                        Ok(BluetoothUUID::CanonicalUUID(globalref, service_number))
                    } else {
                        Err(DOMException::new(globalref, DOMErrorName::SyntaxError))
                    }
                }
            },                
        }
    }
}
