/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use bluetooth_traits::{BluetoothRequest, BluetoothResponse};
use dom::bindings::codegen::Bindings::BluetoothDeviceBinding;
use dom::bindings::codegen::Bindings::BluetoothDeviceBinding::BluetoothDeviceMethods;
use dom::bindings::codegen::Bindings::EventHandlerBinding::EventHandlerNonNull;
use dom::bindings::error::Error;
use dom::bindings::js::{JS, Root, MutHeap, MutNullableHeap};
use dom::bindings::reflector::{Reflectable, reflect_dom_object};
use dom::bindings::str::DOMString;
use dom::bluetooth::{AsyncBluetoothListener, Bluetooth, response_async};
use dom::bluetoothremotegattserver::BluetoothRemoteGATTServer;
use dom::eventtarget::EventTarget;
use dom::globalscope::GlobalScope;
use dom::promise::Promise;
use ipc_channel::ipc::IpcSender;
use js::jsapi::JSContext;
use std::cell::Cell;
use std::rc::Rc;

// https://webbluetoothcg.github.io/web-bluetooth/#bluetoothdevice
#[dom_struct]
pub struct BluetoothDevice {
    eventtarget: EventTarget,
    id: DOMString,
    name: Option<DOMString>,
    gatt: MutNullableHeap<JS<BluetoothRemoteGATTServer>>,
    context: MutHeap<JS<Bluetooth>>,
    watchingAdvertisements: Cell<bool>,
}

impl BluetoothDevice {
    pub fn new_inherited(id: DOMString,
                         name: Option<DOMString>,
                         context: &Bluetooth)
                         -> BluetoothDevice {
        BluetoothDevice {
            eventtarget: EventTarget::new_inherited(),
            id: id,
            name: name,
            gatt: Default::default(),
            context: MutHeap::new(context),
            watchingAdvertisements: Cell::new(false),
        }
    }

    pub fn new(global: &GlobalScope,
               id: DOMString,
               name: Option<DOMString>,
               context: &Bluetooth)
               -> Root<BluetoothDevice> {
        reflect_dom_object(box BluetoothDevice::new_inherited(id,
                                                              name,
                                                              context),
                           global,
                           BluetoothDeviceBinding::Wrap)
    }

    fn get_bluetooth_thread(&self) -> IpcSender<BluetoothRequest> {
        self.global().as_window().bluetooth_thread()
    }

    pub fn get_context(&self) -> Root<Bluetooth> {
        self.context.get()
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
        // TODO: Step 1 - 2: Implement the Permission API.
        self.gatt.or_init(|| {
            BluetoothRemoteGATTServer::new(&self.global(), self)
        })
    }

    #[allow(unrooted_must_root)]
    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothdevice-watchadvertisements
    fn WatchAdvertisements(&self) -> Rc<Promise> {
        let p = Promise::new(&self.global());
        let sender = response_async(&p, self);
        // TODO: Step 1.
        // Note: Steps 2 - 3 are implemented in components/bluetooth/lib.rs in watch_advertisements function
        // and in handle_response function.
        self.get_bluetooth_thread().send(
            BluetoothRequest::WatchAdvertisements(String::from(self.Id()), sender)).unwrap();
        return p;
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothdevice-unwatchadvertisements
    fn UnwatchAdvertisements(&self) -> () {
        // Step 1.
        self.watchingAdvertisements.set(false)
        // TODO: Step 2.
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothdevice-watchingadvertisements
    fn WatchingAdvertisements(&self) -> bool {
        self.watchingAdvertisements.get()
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothdeviceeventhandlers-ongattserverdisconnected
    event_handler!(gattserverdisconnected, GetOngattserverdisconnected, SetOngattserverdisconnected);
}

impl AsyncBluetoothListener for BluetoothDevice {
    fn handle_response(&self, response: BluetoothResponse, promise_cx: *mut JSContext, promise: &Rc<Promise>) {
        match response {
            // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothdevice-unwatchadvertisements
            BluetoothResponse::WatchAdvertisements(_result) => {
                // Step 3.1.
                self.watchingAdvertisements.set(true);
                // Step 3.2.
                promise.resolve_native(promise_cx, &());
            },
            _ => promise.reject_error(promise_cx, Error::Type("Something went wrong...".to_owned())),
        }
    }
}
