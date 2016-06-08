/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use core::ops::Deref;
use dom::bindings::codegen::Bindings::BluetoothAdvertisingEventBinding;
use dom::bindings::codegen::Bindings::BluetoothAdvertisingEventBinding::BluetoothAdvertisingEventMethods;
use dom::bindings::codegen::Bindings::EventBinding::EventMethods;
use dom::bindings::error::Fallible;
use dom::bindings::global::GlobalRef;
use dom::bindings::inheritance::Castable;
use dom::bindings::js::{JS, Root, MutHeap};
use dom::bindings::reflector::reflect_dom_object;
use dom::bindings::str::DOMString;
use dom::bluetoothdevice::BluetoothDevice;
use dom::event::{Event, EventBubbles, EventCancelable};
use string_cache::Atom;

// https://webbluetoothcg.github.io/web-bluetooth/#bluetoothadvertisingdata
#[dom_struct]
pub struct BluetoothAdvertisingEvent {
    event: Event,
    device: MutHeap<JS<BluetoothDevice>>,
    name: Option<DOMString>,
    appearance: Option<u16>,
    txPower: Option<i8>,
    rssi: Option<i8>,
}

impl BluetoothAdvertisingEvent {
    pub fn new_inherited(device: &BluetoothDevice,
                         name: Option<DOMString>,
                         appearance: Option<u16>,
                         txPower: Option<i8>,
                         rssi: Option<i8>)
                         -> BluetoothAdvertisingEvent {
        BluetoothAdvertisingEvent {
            event: Event::new_inherited(),
            device: MutHeap::new(device),
            name: name,
            appearance: appearance,
            txPower: txPower,
            rssi: rssi,
        }
    }

    pub fn new(global: GlobalRef,
               type_: Atom,
               bubbles: EventBubbles,
               cancelable: EventCancelable,
               device: &BluetoothDevice,
               name: Option<DOMString>,
               appearance: Option<u16>,
               txPower: Option<i8>,
               rssi: Option<i8>)
               -> Root<BluetoothAdvertisingEvent> {
        let event = box BluetoothAdvertisingEvent::new_inherited(device, name, appearance, txPower, rssi);
        let ev = reflect_dom_object(event, global, BluetoothAdvertisingEventBinding::Wrap);
        {
            let event = ev.upcast::<Event>();
            event.init_event(type_,
                             bool::from(bubbles),
                             bool::from(cancelable));
        }
        ev
    }

    pub fn Constructor(global: GlobalRef,
                       type_: DOMString,
                       init: &BluetoothAdvertisingEventBinding::BluetoothAdvertisingEventInit)
                       -> Fallible<Root<BluetoothAdvertisingEvent>> {
        let bubbles = EventBubbles::from(init.parent.bubbles);
        let cancelable = EventCancelable::from(init.parent.cancelable);
        Ok(BluetoothAdvertisingEvent::new(global,
                           Atom::from(type_),
                           bubbles,
                           cancelable,
                           init.device.deref(),
                           init.name.clone(),
                           init.appearance,
                           init.txPower,
                           init.rssi))
    }
}

impl BluetoothAdvertisingEventMethods for BluetoothAdvertisingEvent {
    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothadvertisingevent-device
    fn Device(&self) -> Root<BluetoothDevice> {
        self.device.get()
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothadvertisingevent-name
    fn GetName(&self) -> Option<DOMString> {
        self.name.clone()
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothadvertisingdata-appearance
    fn GetAppearance(&self) -> Option<u16> {
        self.appearance
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothadvertisingdata-txpower
    fn GetTxPower(&self) -> Option<i8> {
        self.txPower
    }

    // https://webbluetoothcg.github.io/web-bluetooth/#dom-bluetoothadvertisingdata-rssi
    fn GetRssi(&self) -> Option<i8> {
        self.rssi
    }

    // https://dom.spec.whatwg.org/#dom-event-istrusted
    fn IsTrusted(&self) -> bool {
        self.event.IsTrusted()
    }
}
