/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::BluetoothGATTRemoteServerBinding;
use dom::bindings::codegen::Bindings::BluetoothGATTRemoteServerBinding::BluetoothGATTRemoteServerMethods;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{JS, Root};
use dom::bindings::reflector::{Reflector, reflect_dom_object};
use dom::bluetoothdevice::BluetoothDevice;

#[dom_struct]
pub struct BluetoothGATTRemoteServer {
    reflector_: Reflector,
    device: JS<BluetoothDevice>,
    connected: bool,
}

impl BluetoothGATTRemoteServer {
    pub fn new_inherited(device: &BluetoothDevice, connected: bool) -> BluetoothGATTRemoteServer {
        BluetoothGATTRemoteServer {
            reflector_: Reflector::new(),
            device: JS::from_ref(device),
            connected: connected,
        }
    }

    pub fn new(global: GlobalRef, device: &BluetoothDevice, connected: bool) -> Root<BluetoothGATTRemoteServer> {
        reflect_dom_object(box BluetoothGATTRemoteServer::new_inherited(
                            device,
                            connected),
        global,
        BluetoothGATTRemoteServerBinding::Wrap)
    }

    fn disconnect(&self) -> () {
        //  FIXME (zakorgy)
        //  Set this.connected to false.
        //  In parallel: if, for all BluetoothDevices device in the whole UA with device@[[representedDevice]]
        //  the same device as this.device@[[representedDevice]],
        //  device.gattServer === null or !device.gattServer.connected,
        //  the UA MAY destroy device@[[representedDevice]]’s ATT Bearer.
        //  https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothgattremoteserver-disconnect
        //  https://webbluetoothcg.github.io/web-bluetooth/#same-device
            ()
    }
}

impl BluetoothGATTRemoteServerMethods for BluetoothGATTRemoteServer {
    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothgattremoteserver-device
    fn Device(&self) -> Root<BluetoothDevice> {
        Root::from_ref(&*self.device)
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothgattremoteserver-connected
    fn Connected(&self) -> bool {
        self.connected
    }
}
