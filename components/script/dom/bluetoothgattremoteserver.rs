/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::BluetoothGATTRemoteServerBinding;
use dom::bindings::codegen::Bindings::BluetoothGATTRemoteServerBinding::BluetoothGATTRemoteServerMethods;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{JS, Root};
use dom::bindings::reflector::{Reflector, reflect_dom_object};
use dom::bluetoothdevice::BluetoothDevice;
use std::cell::Cell;

#[dom_struct]
pub struct BluetoothGATTRemoteServer {
    reflector_: Reflector,
    device: JS<BluetoothDevice>,
    connected: Cell<bool>,
}

impl BluetoothGATTRemoteServer {
    pub fn new_inherited(device: &BluetoothDevice, is_connected: bool) -> BluetoothGATTRemoteServer {
        BluetoothGATTRemoteServer {
            reflector_: Reflector::new(),
            device: JS::from_ref(device),
            connected: Cell::new(is_connected),
        }
    }

    pub fn new(global: GlobalRef, device: &BluetoothDevice, connected: bool) -> Root<BluetoothGATTRemoteServer> {
        reflect_dom_object(box BluetoothGATTRemoteServer::new_inherited(
                           device,
                           connected),
        global,
        BluetoothGATTRemoteServerBinding::Wrap)
    }
}

impl BluetoothGATTRemoteServerMethods for BluetoothGATTRemoteServer {
    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothgattremoteserver-device
    fn Device(&self) -> Root<BluetoothDevice> {
        Root::from_ref(&*self.device)
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothgattremoteserver-connected
    fn Connected(&self) -> bool {
        self.connected.get()
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothgattremoteserver-disconnect
    fn Disconnect(&self) -> () {
        //  FIXME (zakorgy)
            ()
    }
}