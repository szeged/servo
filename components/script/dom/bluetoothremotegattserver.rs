/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use bluetooth_traits::{BluetoothRequest, BluetoothResponse};
use bluetooth_traits::blocklist::{Blocklist, uuid_is_blocklisted};
use dom::bindings::codegen::Bindings::BluetoothDeviceBinding::BluetoothDeviceMethods;
use dom::bindings::codegen::Bindings::BluetoothRemoteGATTServerBinding;
use dom::bindings::codegen::Bindings::BluetoothRemoteGATTServerBinding::BluetoothRemoteGATTServerMethods;
use dom::bindings::error::Error::{self, Network, Security};
use dom::bindings::error::ErrorResult;
use dom::bindings::js::{MutJS, Root};
use dom::bindings::reflector::{DomObject, Reflector, reflect_dom_object};
use dom::bluetooth::{AsyncBluetoothListener, response_async};
use dom::bluetoothdevice::BluetoothDevice;
use dom::bluetoothuuid::{BluetoothServiceUUID, BluetoothUUID};
use dom::globalscope::GlobalScope;
use dom::promise::Promise;
use ipc_channel::ipc::{self, IpcSender};
use js::jsapi::JSContext;
use std::cell::Cell;
use std::rc::Rc;

// https://webbluetoothcg.github.io/web-bluetooth/#bluetoothremotegattserver
#[dom_struct]
pub struct BluetoothRemoteGATTServer {
    reflector_: Reflector,
    device: MutJS<BluetoothDevice>,
    connected: Cell<bool>,
}

impl BluetoothRemoteGATTServer {
    pub fn new_inherited(device: &BluetoothDevice) -> BluetoothRemoteGATTServer {
        BluetoothRemoteGATTServer {
            reflector_: Reflector::new(),
            device: MutJS::new(device),
            connected: Cell::new(false),
        }
    }

    pub fn new(global: &GlobalScope, device: &BluetoothDevice) -> Root<BluetoothRemoteGATTServer> {
        reflect_dom_object(box BluetoothRemoteGATTServer::new_inherited(device),
                           global,
                           BluetoothRemoteGATTServerBinding::Wrap)
    }

    fn get_bluetooth_thread(&self) -> IpcSender<BluetoothRequest> {
        self.global().as_window().bluetooth_thread()
    }

    pub fn set_connected(&self, connected: bool) {
        self.connected.set(connected);
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#garbage-collect-the-connection
    #[allow(unrooted_must_root)]
    fn garbage_collect_the_connection(&self) -> ErrorResult {
        // Step 1: TODO: Check if other systems using this device.

        // Step 2.
        let context = self.device.get().get_context();
        for (id, device) in context.get_device_map().borrow().iter() {
            // Step 2.1 - 2.2.
            if id == &self.Device().Id().to_string() {
                if device.get().Gatt().Connected() {
                    return Ok(());
                }
                // TODO: Step 2.3: Implement activeAlgorithms internal slot for BluetoothRemoteGATTServer.
            }
        }

        // Step 3.
        let (sender, receiver) = ipc::channel().unwrap();
        self.get_bluetooth_thread().send(
            BluetoothRequest::GATTServerDisconnect(String::from(self.Device().Id()), sender)).unwrap();
        receiver.recv().unwrap().map_err(Error::from)
    }
}

impl BluetoothRemoteGATTServerMethods for BluetoothRemoteGATTServer {
    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothremotegattserver-device
    fn Device(&self) -> Root<BluetoothDevice> {
        self.device.get()
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothremotegattserver-connected
    fn Connected(&self) -> bool {
        self.connected.get()
    }

    #[allow(unrooted_must_root)]
    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothremotegattserver-connect
    fn Connect(&self) -> Rc<Promise> {
        // Step 1.
        let p = Promise::new(&self.global());
        let sender = response_async(&p, self);

        // TODO: Step 3: Check if the UA is currently using the Bluetooth system.

        // TODO: Step 4: Implement activeAlgorithms internal slot for BluetoothRemoteGATTServer.

        // TODO: Step 5.1 - 5.2: Implement activeAlgorithms internal slot for BluetoothRemoteGATTServer.

        // Note: Steps 2, 5.1.1 and 5.1.3 are in components/bluetooth/lib.rs in the gatt_server_connect function.
        // Steps 5.2.3 - 5.2.5  are in response function.
        self.get_bluetooth_thread().send(
            BluetoothRequest::GATTServerConnect(String::from(self.Device().Id()), sender)).unwrap();
        // Step 5: return promise.
        return p;
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothremotegattserver-disconnect
    fn Disconnect(&self) -> ErrorResult {
        // TODO: Step 1: Implement activeAlgorithms internal slot for BluetoothRemoteGATTServer.

        // Step 2.
        if !self.Connected() {
            return Ok(())
        }

        // Step 3.
        self.Device().clean_up_disconnected_device();

        // Step 4 - 5:
        self.garbage_collect_the_connection()
    }

    #[allow(unrooted_must_root)]
    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothremotegattserver-getprimaryservice
    fn GetPrimaryService(&self, service: BluetoothServiceUUID) -> Rc<Promise> {
        // TODO: Step 1: Implement the Permission API and the allowedServices BluetoothDevice internal slot.
        // Subsequent steps are relative to https://webbluetoothcg.github.io/web-bluetooth/#getgattchildren
        let p = Promise::new(&self.global());
        let p_cx = p.global().get_cx();

        // Step 1.
        let uuid = match BluetoothUUID::service(service) {
            Ok(uuid) => uuid.to_string(),
            Err(e) => {
                p.reject_error(p_cx, e);
                return p;
            }
        };

        // Step 2.
        if uuid_is_blocklisted(uuid.as_ref(), Blocklist::All) {
            p.reject_error(p_cx, Security);
            return p;
        }

        // Step 3 - 4.
        if !self.Device().Gatt().Connected() {
            p.reject_error(p_cx, Network);
            return p;
        }

        // Note: Steps 5 - 7 are implemented in components/bluetooth/lib.rs in get_primary_service function
        // and in handle_response function.
        let sender = response_async(&p, self);
        self.get_bluetooth_thread().send(
            BluetoothRequest::GetPrimaryService(String::from(self.Device().Id()), uuid, sender)).unwrap();
        return p;
    }

    #[allow(unrooted_must_root)]
    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothremotegattserver-getprimaryservices
    fn GetPrimaryServices(&self, service: Option<BluetoothServiceUUID>) -> Rc<Promise> {
        // TODO: Step 1: Implement the Permission API and the allowedServices BluetoothDevice internal slot.
        // Subsequent steps are relative to https://webbluetoothcg.github.io/web-bluetooth/#getgattchildren
        let p = Promise::new(&self.global());
        let p_cx = p.global().get_cx();

        let mut uuid: Option<String> = None;
        if let Some(s) = service {
            // Step 1.
            uuid = match BluetoothUUID::service(s) {
                Ok(uuid) => Some(uuid.to_string()),
                Err(e) => {
                    p.reject_error(p_cx, e);
                    return p;
                }
            };
            if let Some(ref uuid) = uuid {
                // Step 2.
                if uuid_is_blocklisted(uuid.as_ref(), Blocklist::All) {
                    p.reject_error(p_cx, Security);
                    return p;
                }
            }
        };

        // Step 3 - 4.
        if !self.Device().Gatt().Connected() {
            p.reject_error(p_cx, Network);
            return p;
        }

        // Note: Steps 5 - 7 are implemented in components/bluetooth/lib.rs in get_primary_services function
        // and in handle_response function.
        let sender = response_async(&p, self);
        self.get_bluetooth_thread().send(
            BluetoothRequest::GetPrimaryServices(String::from(self.Device().Id()), uuid, sender)).unwrap();
        return p;
    }
}

impl AsyncBluetoothListener for BluetoothRemoteGATTServer {
    fn handle_response(&self, response: BluetoothResponse, promise_cx: *mut JSContext, promise: &Rc<Promise>) {
        let device = self.Device();
        match response {
            // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothremotegattserver-connect
            BluetoothResponse::GATTServerConnect(connected) => {
                // Step 5.2.3
                if self.Device().is_represented_device_null() {
                    if let Err(e) = self.garbage_collect_the_connection() {
                        promise.reject_error(promise_cx, Error::from(e));
                    }
                    promise.reject_error(promise_cx, Error::Network);
                }

                // Step 5.2.4.
                self.connected.set(connected);

                // Step 5.2.5.
                promise.resolve_native(promise_cx, self);
            },
            // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothremotegattserver-getprimaryservice
            // https://webbluetoothcg.github.io/web-bluetooth/#getgattchildren
            // Step 7.
            BluetoothResponse::GetPrimaryService(service) => {
                let bt_service = device.get_or_create_service(&service, &self);
                promise.resolve_native(promise_cx, &bt_service);
            },
            // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothremotegattserver-getprimaryservices
            // https://webbluetoothcg.github.io/web-bluetooth/#getgattchildren
            // Step 7.
            BluetoothResponse::GetPrimaryServices(services_vec) => {
                let mut services = vec!();
                for service in services_vec {
                    let bt_service = device.get_or_create_service(&service, &self);
                    services.push(bt_service);
                }
                promise.resolve_native(promise_cx, &services);
            },
            _ => promise.reject_error(promise_cx, Error::Type("Something went wrong...".to_owned())),
        }
    }
}
