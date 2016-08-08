/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use bluetooth_blacklist::{Blacklist, uuid_is_blacklisted};
use dom::bindings::codegen::Bindings::BluetoothDeviceBinding::BluetoothDeviceMethods;
use dom::bindings::codegen::Bindings::BluetoothRemoteGATTServerBinding;
use dom::bindings::codegen::Bindings::BluetoothRemoteGATTServerBinding::BluetoothRemoteGATTServerMethods;
use dom::bindings::error::Error::{self, Security};
use dom::bindings::error::{Fallible, ErrorResult};
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{JS, MutHeap, Root};
use dom::bindings::reflector::{Reflectable, Reflector, reflect_dom_object};
use dom::bindings::str::DOMString;
use dom::bluetoothdevice::BluetoothDevice;
use dom::bluetoothremotegattservice::BluetoothRemoteGATTService;
use dom::bluetoothuuid::{BluetoothServiceUUID, BluetoothUUID};
use ipc_channel::ipc::{self, IpcSender};
use net_traits::bluetooth_thread::BluetoothMethodMsg;
use std::cell::Cell;

// https://webbluetoothcg.github.io/web-bluetooth/#bluetoothremotegattserver
#[dom_struct]
pub struct BluetoothRemoteGATTServer {
    reflector_: Reflector,
    device: MutHeap<JS<BluetoothDevice>>,
    connected: Cell<bool>,
}

impl BluetoothRemoteGATTServer {
    pub fn new_inherited(device: &BluetoothDevice) -> BluetoothRemoteGATTServer {
        BluetoothRemoteGATTServer {
            reflector_: Reflector::new(),
            device: MutHeap::new(device),
            connected: Cell::new(false),
        }
    }

    pub fn new(global: GlobalRef, device: &BluetoothDevice) -> Root<BluetoothRemoteGATTServer> {
        reflect_dom_object(box BluetoothRemoteGATTServer::new_inherited(device),
        global,
        BluetoothRemoteGATTServerBinding::Wrap)
    }

    fn get_bluetooth_thread(&self) -> IpcSender<BluetoothMethodMsg> {
        let global_root = self.global();
        let global_ref = global_root.r();
        global_ref.as_window().bluetooth_thread()
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

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothremotegattserver-connect
    fn Connect(&self) -> Fallible<Root<BluetoothRemoteGATTServer>> {
        // Note: Step 1-4.1 and a part of Step 4.2
        // are implemented in components/net/bluetooth_thread.rs in gatt_server_connect function.
        let (sender, receiver) = ipc::channel().unwrap();
        self.get_bluetooth_thread().send(
            BluetoothMethodMsg::GATTServerConnect(String::from(self.Device().Id()), sender)).unwrap();
        let server = receiver.recv().unwrap();
        // Step 4.
        // TODO(#4282): Resolve and reject promise.

        match server {
            Ok(connected) => {
                // Step 4.2.
                self.connected.set(connected);

                // Step 4.3.
                Ok(Root::from_ref(self))
            },
            Err(error) => {
                // Note: Errors from Steps 1, 3, 4.1.
                Err(Error::from(error))
            },
        }
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothremotegattserver-disconnect
    fn Disconnect(&self) -> ErrorResult {
        // Note: Step 1, and a part of Step 3
        // are implemented in components/net/bluetooth_thread.rs in gatt_server_disconnect function.
        let (sender, receiver) = ipc::channel().unwrap();
        self.get_bluetooth_thread().send(
            BluetoothMethodMsg::GATTServerDisconnect(String::from(self.Device().Id()), sender)).unwrap();
        let server = receiver.recv().unwrap();

        // TODO(#4282): Step 2: activeAlgorithms.

        match server {
            Ok(connected) => {
                // Step 3.
                self.connected.set(connected);

                //TODO: Step 4: Fire gattserverdisconnected event.

                //TODO: Step 5-6: representedDevice and same device identification.

                Ok(())
            },
            Err(error) => {
                Err(Error::from(error))
            },
        }
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothremotegattserver-getprimaryservice
    fn GetPrimaryService(&self, service: BluetoothServiceUUID) -> Fallible<Root<BluetoothRemoteGATTService>> {
        // Step 2.1.
        // TODO(#4282): Reject promise.
        let uuid = try!(BluetoothUUID::GetService(self.global().r(), service)).to_string();
        // Step 2.2.
        // TODO(#4282): Reject promise.
        if uuid_is_blacklisted(uuid.as_ref(), Blacklist::All) {
            return Err(Security)
        }

        // Note: Step 1, 2.4 and a part of Step 2.5 are implemented
        // in components/net/bluetooth_thread.rs in get_primary_service function.
        let (sender, receiver) = ipc::channel().unwrap();
        self.get_bluetooth_thread().send(
            BluetoothMethodMsg::GetPrimaryService(String::from(self.Device().Id()), uuid, sender)).unwrap();
        let service = receiver.recv().unwrap();

        // Step 2.5.
        // TODO(#4282): Transforming promise.
        match service {
            Ok(service) => {
                Ok(BluetoothRemoteGATTService::new(self.global().r(),
                                                   &self.device.get(),
                                                   DOMString::from(service.uuid),
                                                   service.is_primary,
                                                   service.instance_id))
            },
            Err(error) => {
                Err(Error::from(error))
            },
        }
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothremotegattserver-getprimaryservices
    fn GetPrimaryServices(&self,
                          service: Option<BluetoothServiceUUID>)
                          -> Fallible<Vec<Root<BluetoothRemoteGATTService>>> {
        // Step 2.1.
        // TODO(#4282): Reject promise.
        let mut uuid: Option<String> = None;
        if let Some(s) = service {
            uuid = Some(try!(BluetoothUUID::GetService(self.global().r(), s)).to_string());
            if let Some(ref uuid) = uuid {
                // Step 2.2.
                // TODO(#4282): Reject promise.
                if uuid_is_blacklisted(uuid.as_ref(), Blacklist::All) {
                    return Err(Security)
                }
            }
        };

        // Note: Step 1, 2.4 and a part of Step 2.5 are implemented
        // in components/net/bluetooth_thread.rs in get_primary_services function.
        let (sender, receiver) = ipc::channel().unwrap();
        self.get_bluetooth_thread().send(
            BluetoothMethodMsg::GetPrimaryServices(String::from(self.Device().Id()), uuid, sender)).unwrap();
        let services_vec = receiver.recv().unwrap();

        // Step 2.5.
        // TODO(#4282): Transforming promise.
        match services_vec {
            Ok(service_vec) => {
                Ok(service_vec.into_iter()
                              .map(|service| BluetoothRemoteGATTService::new(self.global().r(),
                                                                             &self.device.get(),
                                                                             DOMString::from(service.uuid),
                                                                             service.is_primary,
                                                                             service.instance_id))
                              .collect())
            },
            Err(error) => {
                Err(Error::from(error))
            },
        }
    }
}
