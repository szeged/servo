/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use bluetooth_blacklist::{Blacklist, uuid_is_blacklisted};
use dom::bindings::cell::DOMRefCell;
use dom::bindings::codegen::Bindings::BluetoothDeviceBinding::BluetoothDeviceMethods;
use dom::bindings::codegen::Bindings::BluetoothRemoteGATTCharacteristicBinding::
    BluetoothRemoteGATTCharacteristicMethods;
use dom::bindings::codegen::Bindings::BluetoothRemoteGATTDescriptorBinding;
use dom::bindings::codegen::Bindings::BluetoothRemoteGATTDescriptorBinding::BluetoothRemoteGATTDescriptorMethods;
use dom::bindings::codegen::Bindings::BluetoothRemoteGATTServerBinding::BluetoothRemoteGATTServerMethods;
use dom::bindings::codegen::Bindings::BluetoothRemoteGATTServiceBinding::BluetoothRemoteGATTServiceMethods;
use dom::bindings::error::Error::{self, InvalidModification, Network, Security};
use dom::bindings::error::{Fallible, ErrorResult};
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{JS, MutHeap, Root};
use dom::bindings::reflector::{Reflectable, Reflector, reflect_dom_object};
use dom::bindings::str::{ByteString, DOMString};
use dom::bluetoothremotegattcharacteristic::{BluetoothRemoteGATTCharacteristic, MAXIMUM_ATTRIBUTE_LENGTH};
use ipc_channel::ipc::{self, IpcSender};
use net_traits::bluetooth_thread::BluetoothMethodMsg;

// http://webbluetoothcg.github.io/web-bluetooth/#bluetoothremotegattdescriptor
#[dom_struct]
pub struct BluetoothRemoteGATTDescriptor {
    reflector_: Reflector,
    characteristic: MutHeap<JS<BluetoothRemoteGATTCharacteristic>>,
    uuid: DOMString,
    value: DOMRefCell<Option<ByteString>>,
    instanceID: String,
}

impl BluetoothRemoteGATTDescriptor {
    pub fn new_inherited(characteristic: &BluetoothRemoteGATTCharacteristic,
                         uuid: DOMString,
                         instanceID: String)
                         -> BluetoothRemoteGATTDescriptor {
        BluetoothRemoteGATTDescriptor {
            reflector_: Reflector::new(),
            characteristic: MutHeap::new(characteristic),
            uuid: uuid,
            value: DOMRefCell::new(None),
            instanceID: instanceID,
        }
    }

    pub fn new(global: GlobalRef,
               characteristic: &BluetoothRemoteGATTCharacteristic,
               uuid: DOMString,
               instanceID: String)
               -> Root<BluetoothRemoteGATTDescriptor>{
        reflect_dom_object(box BluetoothRemoteGATTDescriptor::new_inherited(characteristic,
                                                                            uuid,
                                                                            instanceID),
                            global,
                            BluetoothRemoteGATTDescriptorBinding::Wrap)
    }

    fn get_bluetooth_thread(&self) -> IpcSender<BluetoothMethodMsg> {
        let global_root = self.global();
        let global_ref = global_root.r();
        global_ref.as_window().bluetooth_thread()
    }

    fn get_instance_id(&self) -> String {
        self.instanceID.clone()
    }
}

impl BluetoothRemoteGATTDescriptorMethods for BluetoothRemoteGATTDescriptor {
    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothremotegattdescriptor-characteristic
    fn Characteristic(&self) -> Root<BluetoothRemoteGATTCharacteristic> {
       self.characteristic.get()
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothremotegattdescriptor-uuid
    fn Uuid(&self) -> DOMString {
        self.uuid.clone()
    }

     // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothremotegattdescriptor-value
    fn GetValue(&self) -> Option<ByteString> {
        self.value.borrow().clone()
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothremotegattdescriptor-readvalue
    fn ReadValue(&self) -> Fallible<ByteString> {
        // Step 1.
        // TODO(#4282): Reject promise.
        if uuid_is_blacklisted(self.uuid.as_ref(), Blacklist::Reads) {
            return Err(Security)
        }
        // Step 2.
        // TODO(#4282): Reject promise.
        if !self.Characteristic().Service().Device().Gatt().Connected() {
            return Err(Network)
        }

        // TODO(#4282): Step 4: connection-checking-wraper.

        // Note: Step 4.1 is implemented in components/net/bluetooth_thread.rs in write_value function.
        let (sender, receiver) = ipc::channel().unwrap();
        self.get_bluetooth_thread().send(
            BluetoothMethodMsg::ReadValue(self.get_instance_id(), sender)).unwrap();
        let result = receiver.recv().unwrap();
        let value = match result {
            Ok(val) => {
                ByteString::new(val)
            },
            // Step 4.2.
            // TODO(#4282): Reject promise.
            Err(error) => {
                return Err(Error::from(error))
            },
        };

        // TODO(#4282): Step 4.3.1: activeAlgorithms.

        // Step 4.3.2.
        // TODO(#9530, #5014): DataView, ArrayBuffer.
        *self.value.borrow_mut() = Some(value.clone());

        // Step 4.3.3.
        // TODO(#4282): Resolve promise.
        Ok(value)
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothremotegattdescriptor-writevalue
    fn WriteValue(&self, value: Vec<u8>) -> ErrorResult {
        // Step 1.
        // TODO(#4282): Reject promise.
        if uuid_is_blacklisted(self.uuid.as_ref(), Blacklist::Writes) {
            return Err(Security)
        }

        // TODO(#5014): Step 3: ArrayBuffer.

        // Step 4.
        // TODO(#4282): Reject promise.
        if value.len() > MAXIMUM_ATTRIBUTE_LENGTH {
            return Err(InvalidModification)
        }

        // Step 5.
        // TODO(#4282): Reject promise.
        if !self.Characteristic().Service().Device().Gatt().Connected() {
            return Err(Network)
        }

        // TODO(#4282): Step 6: connection-checking-wraper.

        // Note: Step 6.1 is implemented in components/net/bluetooth_thread.rs in write_value function.
        let (sender, receiver) = ipc::channel().unwrap();
        self.get_bluetooth_thread().send(
            BluetoothMethodMsg::WriteValue(self.get_instance_id(), value, sender)).unwrap();
        let result = receiver.recv().unwrap();

        // TODO(#4282): Step 6.3.1: activeAlgorithms.

        match result {
            // Step 6.3.3.
            // TODO(#4282): Resolve promise.
            Ok(_) => Ok(()),
            // TODO(#9530, #5014): Step 6.3.2: DataView, ArrayBuffer.

            // Step 6.2.
            // TODO(#4282): Reject promise.
            Err(error) => {
                Err(Error::from(error))
            },
        }
    }
}
