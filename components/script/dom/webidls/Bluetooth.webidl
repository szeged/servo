/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// https://webbluetoothcg.github.io/web-bluetooth/#bluetooth

interface Bluetooth {
    // TODO(dati) use Promise when implemented ( https://github.com/servo/servo/issues/4282 )
    //    Promise<BluetoothDevice> requestDevice(RequestDeviceOptions options);
    //    BluetoothDevice requestDevice(RequestDeviceOptions options);
    BluetoothDevice requestDevice ();
    BluetoothDevice createDevice(
        DOMString device_id,
        DOMString device_name,
        unsigned long device_class,
        DOMString vendor_id_source_string,
        unsigned long vendor_id,
        unsigned long product_id,
        unsigned long product_version
        );
};

Bluetooth implements TestBluetoothUuidMethods;

[NoInterfaceObject]
interface TestBluetoothUuidMethods {
    DOMString canonicalTest(unsigned long alias);
    DOMString getServiceTest((DOMString or unsigned long) name);
    DOMString getCharacteristicTest((DOMString or unsigned long) name);
    DOMString getDescriptorTest((DOMString or unsigned long) name);
};

// Bluetooth implements EventTarget;
// Bluetooth implements CharacteristicEventHandlers;
// Bluetooth implements ServiceEventHandlers;
