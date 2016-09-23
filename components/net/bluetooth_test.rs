/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use bluetooth_thread::BluetoothManager;
use device::bluetooth::{BluetoothAdapter, BluetoothDevice, BluetoothGATTCharacteristic, BluetoothGATTService};
use ipc_channel::ipc::IpcSender;
use net_traits::bluetooth_thread::{BluetoothError, BluetoothResult};
use rand::{self, Rng};
use std::borrow::ToOwned;
use std::cell::RefCell;
use std::collections::HashSet;
use std::error::Error;
use std::string::String;

thread_local!(pub static CACHED_IDS: RefCell<HashSet<String>> = RefCell::new(HashSet::new()));

const ADAPTER_ERROR: &'static str = "No adapter found";
const WRONG_DATA_SET_ERROR: &'static str = "Wrong data set name was provided";
const FAILED_SET_ERROR: &'static str = "Failed to set an attribute for testing";

// Adapter names

const PRESENT_ADAPTER: &'static str = "PresentAdapter";
const NOT_PRESENT_ADAPTER: &'static str = "NotPresentAdapter";
const POWERED_ADAPTER: &'static str = "PoweredAdapter";
const NOT_POWERED_ADAPTER: &'static str = "NotPoweredAdapter";
const FAIL_START_DISCOVERY_ADAPTER: &'static str = "FailStartDiscoveryAdapter";
const EMPTY_ADAPTER: &'static str = "EmptyAdapter";
const GLUCOSE_HEART_RATE_ADAPTER: &'static str = "GlucoseHeartRateAdapter";
const UNICODE_DEVICE_ADAPTER: &'static str = "UnicodeDeviceAdapter";
const MISSING_SERVICE_HEART_RATE_ADAPTER: &'static str = "MissingServiceHeartRateAdapter";
const MISSING_CHARACTERISTIC_HEART_RATE_ADAPTER: &'static str = "MissingCharacteristicHeartRateAdapter";
const HEART_RATE_ADAPTER: &'static str = "HeartRateAdapter";
const EMPTY_NAME_HEART_RATE_ADAPTER: &'static str = "EmptyNameHeartRateAdapter";
const NO_NAME_HEART_RATE_ADAPTER: &'static str = "NoNameHeartRateAdapter";
const TWO_HEART_RATE_SERVICES_ADAPTER: &'static str = "TwoHeartRateServicesAdapter";
const BLACKLIST_TEST_ADAPTER: &'static str = "BlacklistTestAdapter";

// Device names

const CONNECTABLE_DEVICE_NAME: &'static str = "Connectable Device";
const EMPTY_DEVICE_NAME: &'static str = "";
const GLUCOSE_DEVICE_NAME: &'static str = "Glucose Device";
const HEART_RATE_DEVICE_NAME: &'static str = "Heart Rate Device";
const UNICODE_DEVICE_NAME: &'static str = "❤❤❤❤❤❤❤❤❤";

// Device addresses

const CONNECTABLE_DEVICE_ADDRESS: &'static str = "00:00:00:00:00:05";
const GLUCOSE_DEVICE_ADDRESS: &'static str = "00:00:00:00:00:02";
const HEART_RATE_DEVICE_ADDRESS: &'static str = "00:00:00:00:00:04";
const UNICODE_DEVICE_ADDRESS: &'static str = "00:00:00:00:00:03";

// Service UUIDs

const BLACKLIST_TEST_SERVICE_UUID: &'static str = "611c954a-263b-4f4a-aab6-01ddb953f985";
const DEVICE_INFORMATION_UUID: &'static str = "0000180A-0000-1000-8000-00805f9b34fb";
const GENERIC_ACCESS_SERVICE_UUID: &'static str = "00001800-0000-1000-8000-00805f9b34fb";
const GLUCOSE_SERVICE_UUID: &'static str = "00001808-0000-1000-8000-00805f9b34fb";
const HEART_RATE_SERVICE_UUID: &'static str = "0000180d-0000-1000-8000-00805f9b34fb";
const HUMAN_INTERFACE_DEVICE_SERVICE_UUID: &'static str = "00001812-0000-1000-8000-00805f9b34fb";
const REQUEST_DISCONNECTION_SERVICE_UUID: &'static str = "00000001-0000-1000-8000-00805f9b34fb";
const TX_POWER_SERVICE_UUID: &'static str = "00001804-0000-1000-8000-00805f9b34fb";

// Characteristic UUIDs

const BODY_SENSOR_LOCATION_CHARACTERISTIC_UUID: &'static str = "00002a38-0000-1000-8000-00805f9b34fb";
const BLACKLIST_EXCLUDE_READS_CHARACTERISTIC_UUID: &'static str = "bad1c9a2-9a5b-4015-8b60-1579bbbf2135";
const DEVICE_NAME_CHARACTERISTIC_UUID: &'static str = "00002A00-0000-1000-8000-00805f9b34fb";
const HEART_RATE_MEASUREMENT_CHARACTERISTIC_UUID: &'static str = "00002a37-0000-1000-8000-00805f9b34fb";
const PHERIPHERAL_PRIVACY_FLAG_CHARACTERISTIC_UUID: &'static str = "00002A02-0000-1000-8000-00805f9b34fb";
const REQUEST_DISCONNECTION_CHARACTERISTIC_UUID: &'static str = "00000002-0000-1000-8000-00805f9b34fb";
const SERIAL_NUMBER_STRING_UUID: &'static str = "00002a25-0000-1000-8000-00805f9b34fb";

pub fn generate_id() -> String {
    let mut id;
    let mut t = false;
        let mut rng = rand::thread_rng();
        loop {
            id = rng.gen::<u32>().to_string();
            CACHED_IDS.with(|cache| if !cache.borrow().contains(&id) {
                cache.borrow_mut().insert(id.clone());
                t = true;
            });
            if t {
                break;
            }
        }
    id
}

pub fn set_attribute_or_return_error(result: Result<(), Box<Error>>, sender: &IpcSender<BluetoothResult<()>>) {
    match result {
        Ok(_) => (),
        Err(_) => return drop(sender.send(Err(BluetoothError::Type(FAILED_SET_ERROR.to_string())))),
    }
}

// Set the adapter's name, is_powered and is_discoverable attributes
pub fn set_adapter(adapter: &BluetoothAdapter, adapter_name: String, sender: &IpcSender<BluetoothResult<()>>) {
    set_attribute_or_return_error(adapter.set_name(adapter_name), sender);
    set_attribute_or_return_error(adapter.set_powered(true), sender);
    set_attribute_or_return_error(adapter.set_discoverable(true), sender);
}

// Create Devices

pub fn create_heart_rate_device(adapter: &BluetoothAdapter,
                                sender: &IpcSender<BluetoothResult<()>>)
                                -> BluetoothDevice {
    let heart_rate_device = BluetoothDevice::create_device(adapter.clone(), generate_id());
    set_attribute_or_return_error(heart_rate_device.set_address(HEART_RATE_DEVICE_ADDRESS.to_owned()), sender);
    set_attribute_or_return_error(heart_rate_device.set_connectable(true), sender);
    set_attribute_or_return_error(heart_rate_device.set_uuids(vec![GENERIC_ACCESS_SERVICE_UUID.to_owned(),
                                                                   HEART_RATE_SERVICE_UUID.to_owned()]),
                                  sender);
    heart_rate_device
}

// Create Services

pub fn create_generic_access_service(device: BluetoothDevice,
                                     sender: &IpcSender<BluetoothResult<()>>)
                                     -> BluetoothGATTService {
    let generic_access_service = BluetoothGATTService::create_service(device,
                                                             generate_id().to_owned());
    set_attribute_or_return_error(generic_access_service.set_uuid(GENERIC_ACCESS_SERVICE_UUID.to_owned()), sender);
    set_attribute_or_return_error(generic_access_service.set_primary(true), sender);
    generic_access_service
}

pub fn create_heart_rate_service(device: BluetoothDevice,
                                 sender: &IpcSender<BluetoothResult<()>>)
                                 -> BluetoothGATTService {
    let heart_rate_service = BluetoothGATTService::create_service(device, generate_id().to_owned());
    set_attribute_or_return_error(heart_rate_service.set_uuid(HEART_RATE_SERVICE_UUID.to_owned()), sender);
    set_attribute_or_return_error(heart_rate_service.set_primary(true), sender);
    heart_rate_service
}

// Create Characteristics

pub fn create_device_name(service: BluetoothGATTService,
                                         sender: &IpcSender<BluetoothResult<()>>)
                                         -> BluetoothGATTCharacteristic {
    let device_name = BluetoothGATTCharacteristic::create_characteristic(service, generate_id().to_owned());
    set_attribute_or_return_error(device_name.set_uuid(DEVICE_NAME_CHARACTERISTIC_UUID.to_owned()), sender);
    device_name
}

pub fn create_pheripheral_privacy_flag(service: BluetoothGATTService,
                                                      sender: &IpcSender<BluetoothResult<()>>)
                                                      -> BluetoothGATTCharacteristic {
    let pheripheral_privacy_flag = BluetoothGATTCharacteristic::create_characteristic(service,
                                                                                      generate_id().to_owned());
    set_attribute_or_return_error(pheripheral_privacy_flag.set_uuid(
        PHERIPHERAL_PRIVACY_FLAG_CHARACTERISTIC_UUID.to_owned()),
                                  sender);
    pheripheral_privacy_flag
}

pub fn create_heart_rate_measurement(service: BluetoothGATTService,
                                                    sender: &IpcSender<BluetoothResult<()>>)
                                                    -> BluetoothGATTCharacteristic {
    let heart_rate_measurement = BluetoothGATTCharacteristic::create_characteristic(service, generate_id().to_owned());
    set_attribute_or_return_error(heart_rate_measurement.set_uuid(
        HEART_RATE_MEASUREMENT_CHARACTERISTIC_UUID.to_owned()), sender);
    set_attribute_or_return_error(heart_rate_measurement.write_value(vec![3]), sender);
    heart_rate_measurement
}

pub fn create_body_sensor_location(service: BluetoothGATTService,
                                                  sender: &IpcSender<BluetoothResult<()>>)
                                                  -> BluetoothGATTCharacteristic {
    let body_sensor_location = BluetoothGATTCharacteristic::create_characteristic(service, generate_id().to_owned());
    set_attribute_or_return_error(body_sensor_location.set_uuid(
        BODY_SENSOR_LOCATION_CHARACTERISTIC_UUID.to_owned()), sender);
    body_sensor_location
}

pub fn test(manager: &mut BluetoothManager, data_set_name: String, sender: IpcSender<BluetoothResult<()>>) {
    match manager.get_or_create_adapter().as_ref() {
        Some(adapter) => {
            match data_set_name.as_str() {
                PRESENT_ADAPTER => {
                    set_adapter(adapter, PRESENT_ADAPTER.to_owned(), &sender);
                    set_attribute_or_return_error(adapter.set_present(true), &sender);
                },
                NOT_PRESENT_ADAPTER => {
                    set_adapter(adapter, NOT_PRESENT_ADAPTER.to_owned(), &sender);
                    set_attribute_or_return_error(adapter.set_present(false), &sender);
                },
                POWERED_ADAPTER => {
                    set_adapter(adapter, POWERED_ADAPTER.to_owned(), &sender);
                },
                NOT_POWERED_ADAPTER => {
                    set_attribute_or_return_error(adapter.set_name(String::from(NOT_POWERED_ADAPTER)), &sender);
                    set_attribute_or_return_error(adapter.set_discoverable(true), &sender);
                },
                FAIL_START_DISCOVERY_ADAPTER => {
                    set_attribute_or_return_error(adapter.set_name(String::from(FAIL_START_DISCOVERY_ADAPTER)),
                                                  &sender);
                    set_attribute_or_return_error(adapter.set_discoverable(true), &sender);
                },
                EMPTY_ADAPTER => {
                    set_adapter(adapter, EMPTY_ADAPTER.to_owned(), &sender);
                },
                GLUCOSE_HEART_RATE_ADAPTER => {
                    set_adapter(adapter, GLUCOSE_HEART_RATE_ADAPTER.to_owned(), &sender);

                    // Glucose Device
                    let glucose_device = BluetoothDevice::create_device(adapter.clone(), generate_id());
                    set_attribute_or_return_error(glucose_device.set_name(GLUCOSE_DEVICE_NAME.to_owned()), &sender);
                    set_attribute_or_return_error(glucose_device.set_address(GLUCOSE_DEVICE_ADDRESS.to_owned()),
                                                  &sender);
                    set_attribute_or_return_error(glucose_device.set_uuids(vec![GLUCOSE_SERVICE_UUID.to_owned(),
                                                                                TX_POWER_SERVICE_UUID.to_owned()]),
                                                  &sender);

                    // Heart Rate Device
                    let heart_rate_device = create_heart_rate_device(adapter, &sender);
                    set_attribute_or_return_error(heart_rate_device.set_name(HEART_RATE_DEVICE_NAME.to_owned()),
                                                  &sender);
                },
                UNICODE_DEVICE_ADAPTER => {
                    set_adapter(adapter, UNICODE_DEVICE_ADAPTER.to_owned(), &sender);

                    // Unicode Device
                    let unicode_device = BluetoothDevice::create_device(adapter.clone(), generate_id());
                    set_attribute_or_return_error(unicode_device.set_name(UNICODE_DEVICE_NAME.to_owned()), &sender);
                    set_attribute_or_return_error(unicode_device.set_address(UNICODE_DEVICE_ADDRESS.to_owned()),
                                                  &sender);
                },
                MISSING_SERVICE_HEART_RATE_ADAPTER => {
                    set_adapter(adapter, MISSING_SERVICE_HEART_RATE_ADAPTER.to_owned(), &sender);

                    // Heart Rate Device
                    let heart_rate_device = create_heart_rate_device(adapter, &sender);
                    set_attribute_or_return_error(heart_rate_device.set_name(HEART_RATE_DEVICE_NAME.to_owned()),
                                                  &sender);
                },
                MISSING_CHARACTERISTIC_HEART_RATE_ADAPTER => {
                    set_adapter(adapter, MISSING_CHARACTERISTIC_HEART_RATE_ADAPTER.to_owned(), &sender);

                    // Heart Rate Device
                    let heart_rate_device = create_heart_rate_device(adapter, &sender);
                    set_attribute_or_return_error(heart_rate_device.set_name(HEART_RATE_DEVICE_NAME.to_owned()),
                                                  &sender);

                    // Generic Access Service
                    let generic_access_service = create_generic_access_service(heart_rate_device.clone(), &sender);

                    // Heart Rate Service
                    let heart_rate_service = create_heart_rate_service(heart_rate_device.clone(), &sender);
                },
                HEART_RATE_ADAPTER => {
                    set_adapter(adapter, HEART_RATE_ADAPTER.to_owned(), &sender);

                    // Heart Rate Device
                    let heart_rate_device = create_heart_rate_device(adapter, &sender);
                    set_attribute_or_return_error(heart_rate_device.set_name(HEART_RATE_DEVICE_NAME.to_owned()),
                                                  &sender);

                    // Generic Access Service
                    let generic_access_service = create_generic_access_service(heart_rate_device.clone(), &sender);

                    // Device Name Characteristic
                    let device_name = create_device_name(generic_access_service.clone(), &sender);
                    set_attribute_or_return_error(device_name.write_value(vec![9]), &sender);

                    // Pheripheral Privacy Flag Characteristic
                    let pheripheral_privacy_flag = create_pheripheral_privacy_flag(generic_access_service.clone(),
                                                                                  &sender);

                    // Heart Rate Service
                    let heart_rate_service = create_heart_rate_service(heart_rate_device.clone(), &sender);

                    // Heart Rate Measurement Characteristic
                    let heart_rate_measurement = create_heart_rate_measurement(heart_rate_service.clone(), &sender);

                    // Body Sensor Location Characteristic 1
                    let body_sensor_location_1 = create_body_sensor_location(heart_rate_service.clone(), &sender);
                    set_attribute_or_return_error(body_sensor_location_1.write_value(vec![1]), &sender);

                    // Body Sensor Location Characteristic 2
                    let body_sensor_location_2 = create_body_sensor_location(heart_rate_service.clone(), &sender);
                    set_attribute_or_return_error(body_sensor_location_2.write_value(vec![2]), &sender);
                },
                EMPTY_NAME_HEART_RATE_ADAPTER => {
                    set_adapter(adapter, EMPTY_NAME_HEART_RATE_ADAPTER.to_owned(), &sender);

                    // Heart Rate Device
                    let heart_rate_device = create_heart_rate_device(adapter, &sender);
                    set_attribute_or_return_error(heart_rate_device.set_name(EMPTY_DEVICE_NAME.to_owned()), &sender);

                    // Generic Access Service
                    let generic_access_service = create_generic_access_service(heart_rate_device.clone(), &sender);

                    // Device Name Characteristic
                    let device_name = create_device_name(generic_access_service.clone(), &sender);

                    // Pheripheral Privacy Flag
                    let pheripheral_privacy_flag = create_pheripheral_privacy_flag(generic_access_service.clone(),
                                                                                   &sender);

                    // Heart Rate Service
                    let heart_rate_service = create_heart_rate_service(heart_rate_device.clone(), &sender);

                    // Heart Rate Measurement Characteristic
                    let heart_rate_measurement = create_heart_rate_measurement(heart_rate_service.clone(), &sender);

                    // Body Sensor Location Characteristic 1
                    let body_sensor_location_1 = create_body_sensor_location(heart_rate_service.clone(), &sender);
                    set_attribute_or_return_error(body_sensor_location_1.write_value(vec![1]), &sender);

                    // Body Sensor Location Characteristic 2
                    let body_sensor_location_2 = create_body_sensor_location(heart_rate_service.clone(), &sender);
                    set_attribute_or_return_error(body_sensor_location_2.write_value(vec![2]), &sender);
                },
                NO_NAME_HEART_RATE_ADAPTER => {
                    set_adapter(adapter, NO_NAME_HEART_RATE_ADAPTER.to_owned(), &sender);

                    // Heart Rate Device
                    let heart_rate_device = create_heart_rate_device(adapter, &sender);

                    // Generic Access Service
                    let generic_access_service = create_generic_access_service(heart_rate_device.clone(), &sender);

                    // Device Name Characteristic
                    let device_name = create_device_name(generic_access_service.clone(), &sender);

                    // Pheripheral Privacy Flag Characteristic
                    let pheripheral_privacy_flag = create_pheripheral_privacy_flag(generic_access_service.clone(),
                                                                                   &sender);

                    // Heart Rate Service
                    let heart_rate_service = create_heart_rate_service(heart_rate_device.clone(), &sender);

                    // Heart Rate Measurement Characteristic
                    let heart_rate_measurement = create_heart_rate_measurement(heart_rate_service.clone(), &sender);

                    // Body Sensor Location Characteristic 1
                    let body_sensor_location_1 = create_body_sensor_location(heart_rate_service.clone(), &sender);
                    set_attribute_or_return_error(body_sensor_location_1.write_value(vec![1]), &sender);

                    // Body Sensor Location Characteristic 2
                    let body_sensor_location_2 = create_body_sensor_location(heart_rate_service.clone(), &sender);
                    set_attribute_or_return_error(body_sensor_location_2.write_value(vec![2]), &sender);
                },
                TWO_HEART_RATE_SERVICES_ADAPTER => {
                    set_adapter(adapter, TWO_HEART_RATE_SERVICES_ADAPTER.to_owned(), &sender);

                    // Heart Rate Device
                    let heart_rate_device = create_heart_rate_device(adapter, &sender);
                    set_attribute_or_return_error(heart_rate_device.set_name(HEART_RATE_DEVICE_NAME.to_owned()),
                                                  &sender);
                    set_attribute_or_return_error(heart_rate_device.set_uuids(vec![
                        GENERIC_ACCESS_SERVICE_UUID.to_owned(),
                        HEART_RATE_SERVICE_UUID.to_owned(),
                        HEART_RATE_SERVICE_UUID.to_owned()]),
                                                  &sender);

                    // Generic Access Service
                    let generic_access_service = create_generic_access_service(heart_rate_device.clone(), &sender);

                    // Device Name Characteristic
                    let device_name = create_device_name(generic_access_service.clone(), &sender);
                    set_attribute_or_return_error(device_name.write_value(vec![9]), &sender);

                    // Pheripheral Privacy Flag Characteristic
                    let pheripheral_privacy_flag = create_pheripheral_privacy_flag(generic_access_service.clone(),
                                                                                  &sender);

                    // Heart Rate Service 1
                    let heart_rate_service_1 = create_heart_rate_service(heart_rate_device.clone(), &sender);

                    // Heart Rate Service 2
                    let heart_rate_service_2 = create_heart_rate_service(heart_rate_device.clone(), &sender);

                    // Heart Rate Measurement Characteristic
                    let heart_rate_measurement = create_heart_rate_measurement(heart_rate_service_1.clone(), &sender);

                    // Body Sensor Location Characteristic 1
                    let body_sensor_location_1 = create_body_sensor_location(heart_rate_service_1.clone(), &sender);
                    set_attribute_or_return_error(body_sensor_location_1.write_value(vec![1]), &sender);

                    // Body Sensor Location Characteristic 2
                    let body_sensor_location_2 = create_body_sensor_location(heart_rate_service_2.clone(), &sender);
                    set_attribute_or_return_error(body_sensor_location_2.write_value(vec![2]), &sender);
                },
                BLACKLIST_TEST_ADAPTER => {
                    set_adapter(adapter, BLACKLIST_TEST_ADAPTER.to_owned(), &sender);

                    // Connectable Device
                    let connectable_device = BluetoothDevice::create_device(adapter.clone(), generate_id());
                    set_attribute_or_return_error(
                        connectable_device.set_name(CONNECTABLE_DEVICE_NAME.to_owned()), &sender);
                    set_attribute_or_return_error(
                        connectable_device.set_address(CONNECTABLE_DEVICE_ADDRESS.to_owned()), &sender);
                    set_attribute_or_return_error(connectable_device.set_connectable(true), &sender);
                    set_attribute_or_return_error(connectable_device.set_uuids(vec![
                        BLACKLIST_TEST_SERVICE_UUID.to_owned(),
                        DEVICE_INFORMATION_UUID.to_owned(),
                        GENERIC_ACCESS_SERVICE_UUID.to_owned(),
                        HEART_RATE_SERVICE_UUID.to_owned(),
                        HUMAN_INTERFACE_DEVICE_SERVICE_UUID.to_owned()]),
                                                  &sender);

                    // Blacklist Test Service
                    let blacklist_test_service = BluetoothGATTService::create_service(connectable_device.clone(),
                                                                                      generate_id().to_owned());
                    set_attribute_or_return_error(blacklist_test_service.set_uuid(BLACKLIST_TEST_SERVICE_UUID
                                                                                  .to_owned()),
                                                  &sender);

                    // Blacklist Exclude Reads Characteristic
                    let blacklist_exclude_reads_characteristic =
                        BluetoothGATTCharacteristic::create_characteristic(blacklist_test_service.clone(),
                                                                           generate_id().to_owned());
                    set_attribute_or_return_error(blacklist_exclude_reads_characteristic
                                                  .set_uuid(BLACKLIST_EXCLUDE_READS_CHARACTERISTIC_UUID.to_owned()),
                                                  &sender);

                    // Device Information Service
                    let device_information_service = BluetoothGATTService::create_service(connectable_device.clone(),
                                                                                          generate_id().to_owned());
                    set_attribute_or_return_error(device_information_service.set_uuid(DEVICE_INFORMATION_UUID
                                                                                      .to_owned()),
                                                  &sender);

                    // Serial Number String Characteristic
                    let serial_number_string =
                        BluetoothGATTCharacteristic::create_characteristic(blacklist_test_service.clone(),
                                                                           generate_id().to_owned());
                    set_attribute_or_return_error(serial_number_string.set_uuid(SERIAL_NUMBER_STRING_UUID.to_owned()),
                                                  &sender);

                    // Generic Access Service
                    let generic_access_service = create_generic_access_service(connectable_device.clone(), &sender);

                    // Device Name Characteristic
                    let device_name = create_device_name(generic_access_service.clone(), &sender);
                    set_attribute_or_return_error(device_name.write_value(vec![9]), &sender);

                    // Pheripheral Privacy Flag Characteristic
                    let pheripheral_privacy_flag = create_pheripheral_privacy_flag(generic_access_service.clone(),
                                                                                  &sender);

                    // Heart Rate Service
                    let heart_rate_service = create_heart_rate_service(connectable_device.clone(), &sender);

                    // Heart Rate Measurement Characteristic
                    let heart_rate_measurement = create_heart_rate_measurement(heart_rate_service.clone(), &sender);

                    // Body Sensor Location Characteristic 1
                    let body_sensor_location_1 = create_body_sensor_location(heart_rate_service.clone(), &sender);
                    set_attribute_or_return_error(body_sensor_location_1.write_value(vec![1]), &sender);

                    // Body Sensor Location Characteristic 2
                    let body_sensor_location_2 = create_body_sensor_location(heart_rate_service.clone(), &sender);
                    set_attribute_or_return_error(body_sensor_location_2.write_value(vec![2]), &sender);

                    // Human Interface Device Service
                    let human_interface_device_service =
                        BluetoothGATTService::create_service(connectable_device.clone(), generate_id().to_owned());
                    set_attribute_or_return_error(human_interface_device_service.set_uuid(
                        HUMAN_INTERFACE_DEVICE_SERVICE_UUID.to_owned()),
                                                  &sender);
                },
                _ => return drop(sender.send(Err(BluetoothError::Type(WRONG_DATA_SET_ERROR.to_string())))),
            }
        }
        None => {
            return drop(sender.send(Err(BluetoothError::Type(ADAPTER_ERROR.to_string()))))
        },
    }
    return drop(sender.send(Ok(())));
}

// |PresentAdapter|
// Devices added:
//  None.
// Mock Functions:
//  - IsPresent: Returns true

// |NotPresentAdapter|
// Devices added:
//  None.
// Mock Functions:
//  - IsPresent: Returns false

// |PoweredAdapter|
// Devices added:
//  None.
// Mock Functions:
//  - IsPowered: Returns true

// |NotPoweredAdapter|
// Devices added:
//  None.
// Mock Functions:
//  - IsPowered: Returns false

// |FailStartDiscoveryAdapter|
// Devices added:
//  None.
// Mock Functions:
//  - StartDiscoverySessionWithFilter:
//      Results in error.

// |EmptyAdapter|
// Devices Added:
//  None.
// Mock Functions:
//  - StartDiscoverySessionWithFilter:
//      Successfully runs the DiscoverySession.

// |GlucoseHeartRateAdapter|
// Devices added:
//  - |GlucoseDevice|
//  - |HeartRateDevice|

// |GetUnicodeDeviceAdapter|
// Internal structure
//  - UnicodeDevice
//    - GetName(): Returns "❤❤❤❤❤❤❤❤❤"

// |MissingServiceHeartRateAdapter|
// Internal Structure:
//   - Heart Rate Device
//      - UUIDs:
//         - Generic Access UUID (0x1800)
//         - Heart Rate UUID (0x180d)

// |MissingCharacteristicHeartRateAdapter|
// The services in this adapter do not contain any characteristics.
// Internal Structure:
//   - Heart Rate Device
//      - UUIDs:
//         - Generic Access UUID (0x1800)
//         - Heart Rate UUID (0x180d)
//      - Services:
//         - Generic Access Service
//         - Heart Rate Service

// |HeartRateAdapter|
// Internal Structure:
//   - Heart Rate Device
//      - UUIDs:
//         - Generic Access UUID (0x1800)
//         - Heart Rate UUID (0x180d)
//      - Services:
//         - Generic Access Service - Characteristics as described in
//           GetGenericAccessService.
//         - Heart Rate Service - Characteristics as described in
//           GetHeartRateService.

// |EmptyNameHeartRateAdapter|
// Internal Structure:
//   - Heart Rate Device
//     It's name is an empty string
//      - UUIDs:
//         - Generic Access UUID (0x1800)
//         - Heart Rate UUID (0x180d)
//      - Services:
//         - Generic Access Service - Characteristics as described in
//           GenericAccessService.
//            - gap.device_name returns an empty string.
//         - Heart Rate Service - Characteristics as described in
//           HeartRateService.

// |NoNameHeartRateAdapter|
// Internal Structure:
//   - Heart Rate Device
//      - GetName returns NULL.
//      - UUIDs:
//         - Generic Access UUID (0x1800)
//         - Heart Rate UUID (0x180d)
//      - Services:
//         - Generic Access Service - Characteristics as described in
//           GetGenericAccessService.
//            - gap.device_name returns an empty string.
//         - Heart Rate Service - Characteristics as described in
//           GetHeartRateService.

// |TwoHeartRateServicesAdapter|
// Internal Structure:
//   - Heart Rate Device
//      - UUIDs:
//         - Generic Access UUID (0x1800)
//         - Heart Rate UUID (0x180d)
//         - Heart Rate UUID (0x180d)
//      - Services:
//         - Generic Access Service - Characteristics as described in
//           GenericAccessService.
//         - Heart Rate Service - Heart Rate Measurement (0x2a37) & Body
//           Sensor Location (0x2a38).
//         - Heart Rate Service - Body Sensor Location (0x2a38).

// |BlacklistTestAdapter|
// Internal Structure:
//   - |ConnectableDevice|(adapter, "Blacklist Test Device", uuids)
//      - UUIDs:
//         - Blacklist Test Service UUID
//           (611c954a-263b-4f4a-aab6-01ddb953f985)
//         - Device Information UUID (0x180a)
//         - Generic Access UUID (0x1800)
//         - Heart Rate UUID (0x180d)
//         - Human Interface Device UUID (0x1812) (a blacklisted service)
//      - Services:
//         - Blacklist Test Service - Characteristics as described in
//           BlacklistTestService.
//         - Device Information Service - Characteristics as described in
//           DeviceInformationService.
//         - Generic Access Service - Characteristics as described in
//           GenericAccessService.
//         - Heart Rate Service - Characteristics as described in
//           HeartRateService.
//         - Human Interface Device Service - No characteristics needed
//           because the service is blacklisted.

// |GlucoseDevice|
// Name: "Glucose Device"
// Address: "00:00:00:00:00:02"
// UUIDs added:
//   - Generic Access (0x1800)
//   - Glucose UUID (0x1808)
//   - Tx Power (0x1804)
// Services added:
// None.

// |ConnectableDevice|
// Name: "Connectable Device"
// Address: "00:00:00:00:00:05"
// UUIDs added:
// None.
// Services added:
// None.
// Successfully connects

// |HeartRateDevice|
// Name: "Heart Rate Device"
// Address: "00:00:00:00:00:04"
// UUIDs added:
//   - Generic Access (0x1800)
//   - Heart Rate UUID (0x180D)
// Services added:
// None. Each user of the HeartRateDevice is in charge of adding the
// relevant services, characteristics and descriptors.

// |DeviceInformationService|
// Internal Structure:
//  - Characteristics:
//     - Serial Number String: (0x2a25) (a blacklisted characteristic)
//        - Mock Functions:
//           - Read: Fails test.

// |BlacklistTestService|
// Internal Structure:
//  - Characteristics:
//     - Blacklist Exclude Reads Characteristic:
//       (bad1c9a2-9a5b-4015-8b60-1579bbbf2135)
//        - Mock Functions:
//           - Read: Fails test.
//           - Write: Calls success.

// |GenericAccessService|
// Internal Structure:
//  - Characteristics:
//     - Device Name: (0x2A00)
//        - Mock Functions:
//           - Read: successfully reads the value.
//           - Write: Calls success.
//     - Peripheral Privacy Flag: (0x2A02) (blacklisted for writes)
//        - Mock Functions:
//           - Read: successfully reads the value.
//           - Write: Fails test.
//           - GetProperties: Returns

// |HeartRateService|
// Internal Structure:
//  - Characteristics:
//     - Heart Rate Measurement (0x2a37)
//     - Body Sensor Location (0x2a38)
//        - Mock Functions:
//           - Read: successfully reads the value
//     - Body Sensor Location (0x2a38)
//        - Mock Functions:
//           - Read: successfully reads the value
