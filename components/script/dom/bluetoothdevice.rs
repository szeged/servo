/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::BluetoothDeviceBinding;
use dom::bindings::codegen::Bindings::BluetoothDeviceBinding::BluetoothDeviceMethods;
use dom::bindings::error::Error::Type;
use dom::bindings::error::ErrorResult;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{JS, Root, MutNullableHeap};
use dom::bindings::reflector::{Reflectable, Reflector, reflect_dom_object};
use dom::bindings::str::DOMString;
use dom::bluetoothremotegattserver::BluetoothRemoteGATTServer;
use ipc_channel::ipc::{self, IpcSender};
use net_traits::bluetooth_thread::BluetoothMethodMsg;
use std::cell::Cell;

const WRONG_FUNC_CALL: &'static str = "The device already (un)watches the Advertisements.";

// https://webbluetoothcg.github.io/web-bluetooth/#bluetoothdevice
#[dom_struct]
pub struct BluetoothDevice {
    reflector_: Reflector,
    id: DOMString,
    name: Option<DOMString>,
    gatt: MutNullableHeap<JS<BluetoothRemoteGATTServer>>,
    watchingAdvertisements: Cell<bool>,
}

impl BluetoothDevice {
    pub fn new_inherited(id: DOMString,
                         name: Option<DOMString>)
                         -> BluetoothDevice {
        BluetoothDevice {
            reflector_: Reflector::new(),
            id: id,
            name: name,
            gatt: Default::default(),
            watchingAdvertisements: Cell::new(false),
        }
    }

    pub fn new(global: GlobalRef,
               id: DOMString,
               name: Option<DOMString>)
               -> Root<BluetoothDevice> {
        reflect_dom_object(box BluetoothDevice::new_inherited(id,
                                                              name),
                           global,
                           BluetoothDeviceBinding::Wrap)
    }

    fn get_bluetooth_thread(&self) -> IpcSender<BluetoothMethodMsg> {
        let global_root = self.global();
        let global_ref = global_root.r();
        global_ref.as_window().bluetooth_thread()
    }
}

impl BluetoothDeviceMethods for BluetoothDevice {
     // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothdevice-id
    fn Id(&self) -> DOMString {
        self.id.clone()
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothdevice-name
    fn GetName(&self) -> Option<DOMString> {
        self.name.clone()
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothdevice-gatt
    fn Gatt(&self) -> Root<BluetoothRemoteGATTServer> {
        self.gatt.or_init(|| BluetoothRemoteGATTServer::new(self.global().r(), self))
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothdevice-watchadvertisements
    fn WatchAdvertisements(&self) -> ErrorResult {
        if self.watchingAdvertisements.get() {
            return Err(Type(WRONG_FUNC_CALL.to_owned()));
        }
        let (sender, receiver) = ipc::channel().unwrap();
        self.get_bluetooth_thread().send(
            BluetoothMethodMsg::WatchAdvertisements(self.id.to_string(), sender)).unwrap();
        let result = receiver.recv().unwrap();
        match result {
            Ok(is_watching) => {
                self.watchingAdvertisements.set(is_watching);
                Ok(())
            },
            Err(error) => {
                Err(Type(error))
            },
        }
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothdevice-unwatchadvertisements
    fn UnwatchAdvertisements(&self) -> ErrorResult {
        if !self.watchingAdvertisements.get() {
            return Err(Type(WRONG_FUNC_CALL.to_owned()));
        }
        let (sender, receiver) = ipc::channel().unwrap();
        self.get_bluetooth_thread().send(
            BluetoothMethodMsg::UnwatchAdvertisements(self.id.to_string(), sender)).unwrap();
        let result = receiver.recv().unwrap();
        match result {
            Ok(is_watching) => {
                self.watchingAdvertisements.set(is_watching);
                Ok(())
            },
            Err(error) => {
                Err(Type(error))
            },
        }
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothdevice-watchingadvertisements
    fn WatchingAdvertisements(&self) -> bool {
        self.watchingAdvertisements.get()
    }
}
