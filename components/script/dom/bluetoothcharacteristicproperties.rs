/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::BluetoothCharacteristicPropertiesBinding;
use dom::bindings::codegen::Bindings::BluetoothCharacteristicPropertiesBinding::BluetoothCharacteristicPropertiesMethods;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::reflector::{Reflector, reflect_dom_object};

// https://webbluetoothcg.github.io/web-bluetooth/#characteristicproperties

 #[dom_struct]
pub struct BluetoothCharacteristicProperties {
    reflector_: Reflector,
    broadcast: bool,
    read: bool,
    writeWithoutResponse: bool,
    write: bool,
    notify: bool,
    indicate: bool,
    authenticatedSignedWrites: bool,
    reliableWrite: bool,
    writableAuxiliaries: bool,
}

impl BluetoothCharacteristicProperties {
    pub fn new_inherited(broadcast: bool,
                         read: bool,
                         writeWithoutResponse: bool,
                         write: bool,
                         notify: bool,
                         indicate: bool,
                         authenticatedSignedWrites: bool,
                         reliableWrite: bool,
                         writableAuxiliaries: bool)
                         -> BluetoothCharacteristicProperties {
        BluetoothCharacteristicProperties {
            reflector_: Reflector::new(),
            broadcast: broadcast,
            read: read,
            writeWithoutResponse: writeWithoutResponse,
            write: write,
            notify: notify,
            indicate: indicate,
            authenticatedSignedWrites: authenticatedSignedWrites,
            reliableWrite: reliableWrite,
            writableAuxiliaries: writableAuxiliaries,
        }
    }

    pub fn new(global: GlobalRef,
               broadcast: bool,
               read: bool,
               writeWithoutResponse: bool,
               write: bool,
               notify: bool,
               indicate: bool,
               authenticatedSignedWrites: bool,
               reliableWrite: bool,
               writableAuxiliaries: bool)
               -> Root<BluetoothCharacteristicProperties> {
        reflect_dom_object(box BluetoothCharacteristicProperties::new_inherited(broadcast,
                                                                                read,
                                                                                writeWithoutResponse,
                                                                                write,
                                                                                notify,
                                                                                indicate,
                                                                                authenticatedSignedWrites,
                                                                                reliableWrite,
                                                                                writableAuxiliaries),
                           global,
                           BluetoothCharacteristicPropertiesBinding::Wrap)
        }
    }

impl BluetoothCharacteristicPropertiesMethods for BluetoothCharacteristicProperties {
    fn Broadcast(&self) -> bool {
        self.broadcast
    }

    fn Read(&self) -> bool {
        self.read
    }

    fn WriteWithoutResponse(&self) -> bool {
        self.writeWithoutResponse
    }

    fn Write(&self) -> bool {
        self.write
    }

    fn Notify(&self) -> bool {
        self.notify
    }

    fn Indicate(&self) -> bool {
        self.indicate
    }

    fn AuthenticatedSignedWrites(&self) -> bool {
        self.authenticatedSignedWrites
    }

    fn ReliableWrite(&self) -> bool {
        self.reliableWrite
    }

    fn WritableAuxiliaries(&self) -> bool {
        self.writableAuxiliaries
    }
}
