/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// https://webbluetoothcg.github.io/web-bluetooth/#bluetoothgattservice

interface BluetoothGATTService {
    readonly attribute BluetoothDevice? device;
    readonly attribute boolean isPrimary;
    readonly attribute UUID uuid;

    void SetDevice(BluetoothDevice device);
    BluetoothGATTCharacteristic getCharacteristic();
    //Promise<BluetoothGATTCharacteristic>getCharacteristic(BluetoothCharacteristicUUID characteristic);
    //Promise<sequence<BluetoothGATTCharacteristic>>
    //getCharacteristics(optional BluetoothCharacteristicUUID characteristic);
    //Promise<BluetoothGATTService>getIncludedService(BluetoothServiceUUID service);
    //Promise<sequence<BluetoothGATTService>>getIncludedServices(optional BluetoothServiceUUID service);
};
