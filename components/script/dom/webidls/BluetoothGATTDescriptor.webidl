/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// http://webbluetoothcg.github.io/web-bluetooth/#bluetoothgattdescriptor

interface BluetoothGATTDescriptor {
  readonly attribute BluetoothGATTCharacteristic? characteristic;
  readonly attribute UUID uuid;
  //readonly attribute ArrayBuffer? value;

  void SetCharacteristic(BluetoothGATTCharacteristic characteristic);

  // TODO(fokinv) use Promise when implemented ( https://github.com/servo/servo/issues/4282 )

  //Promise<DataView> readValue();
  //Promise<void> writeValue(BufferSource value);
};
