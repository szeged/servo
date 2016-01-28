/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//https://webbluetoothcg.github.io/web-bluetooth/#bluetoothadvertisingdata

interface BluetoothAdvertisingData {
    readonly attribute unsigned short? appearance;
    readonly attribute byte? txPower;
    readonly attribute byte? rssi;
    // readonly attribute Map manufacturerData;
    // readonly attribute Map serviceData;
};

// TODO(dati)
// Map spec: https://tc39.github.io/ecma262/#sec-map-constructor
// manufacturerData maps unsigned short Company Identifier Codes to DataViews.
// serviceData maps UUIDs to DataViews.
// DataView spec: https://tc39.github.io/ecma262/#sec-dataview-constructor
