/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::global::GlobalRef;
use dom::bindings::reflector::Reflector;
use dom::bindings::codegen::UnionTypes::StringOrUnsignedLong;
use util::str::DOMString;

// https://webbluetoothcg.github.io/web-bluetooth/#bluetoothuuid

 #[dom_struct]
pub struct BluetoothUUID {
    reflector_: Reflector,
}

impl BluetoothUUID {
    pub fn CanonicalUUID(_: GlobalRef, _alias: u32) -> DOMString {
        DOMString::new()
    }

    pub fn GetDescriptor(_: GlobalRef, name: StringOrUnsignedLong) -> DOMString {
    	DOMString::new()
    }

    pub fn GetService(_: GlobalRef, name: StringOrUnsignedLong) -> DOMString {
    	DOMString::new()
    }
}
