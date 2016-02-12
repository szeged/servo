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
use dom::bindings::cell::DOMRefCell;

#[dom_struct]
pub struct BluetoothGATTRemoteServer {
    reflector_: Reflector,
    device: DOMRefCell<Option<JS<BluetoothDevice>>>,
    connected: Cell<bool>,
}

impl BluetoothGATTRemoteServer {
    pub fn new_inherited(device: Option<&BluetoothDevice>, is_connected: bool) -> BluetoothGATTRemoteServer {
        BluetoothGATTRemoteServer {
            reflector_: Reflector::new(),
            device: DOMRefCell::new(device.map(JS::from_ref)),
            connected: Cell::new(is_connected),
        }
    }

    pub fn new(global: GlobalRef, device: Option<&BluetoothDevice>, connected: bool) -> Root<BluetoothGATTRemoteServer> {
        reflect_dom_object(box BluetoothGATTRemoteServer::new_inherited(
                           device,
                           connected),
        global,
        BluetoothGATTRemoteServerBinding::Wrap)
    }

    
    /*pub fn setDevice(&mut self, device: &BluetoothDevice) -> Root<BluetoothGATTRemoteServer>{
        self.device = Some(JS::from_ref(device));
        Root::from_ref(&self)
    }*/
}

impl BluetoothGATTRemoteServerMethods for BluetoothGATTRemoteServer {
    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothgattremoteserver-device
    fn GetDevice(&self) -> Option<Root<BluetoothDevice>> {
        if let Some(ref is_device) = self.device.borrow().clone() {
            Some(Root::from_ref(&*is_device))
        } else {
            None
        }
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothgattremoteserver-connected
    fn Connected(&self) -> bool {
        self.connected.get()
    }

    fn Connect(&self) -> () {
        //FIXME
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothgattremoteserver-disconnect
    fn Disconnect(&self) -> () {
        self.connected.set(!self.Connected());
        //  FIXME (zakorgy)
            ()
    }

    fn SetDevice(&self, device: &BluetoothDevice){
        *self.device.borrow_mut() = Some(JS::from_ref(device));
    }
}
