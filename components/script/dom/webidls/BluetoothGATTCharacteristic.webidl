/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// https://webbluetoothcg.github.io/web-bluetooth/#bluetoothgattcharacteristic

interface BluetoothGATTCharacteristic {
  readonly attribute BluetoothCharacteristicProperties properties;
  //readonly attribute BluetoothGATTService service;
  readonly attribute UUID uuid;
  //readonly attribute ArrayBuffer? value;

  // TODO(fokinv) use Promise when implemented ( https://github.com/servo/servo/issues/4282 )

  //Promise<BluetoothGATTDescriptor> getDescriptor(BluetoothDescriptorUUID descriptor);
  //Promise<sequence<BluetoothGATTDescriptor>>
  //getDescriptors(optional BluetoothDescriptorUUID descriptor);
  //Promise<DataView> readValue();
  //Promise<void> writeValue(BufferSource value);
  //Promise<void> startNotifications();
  //Promise<void> stopNotifications();
};

//BluetoothGATTCharacteristic implements EventTarget;
//BluetoothGATTCharacteristic implements CharacteristicEventHandlers;
