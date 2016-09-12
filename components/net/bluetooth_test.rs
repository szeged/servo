/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use bluetooth_thread::BluetoothManager;
use device::bluetooth::{BluetoothDevice, BluetoothGATTCharacteristic, BluetoothGATTDescriptor, BluetoothGATTService};
use ipc_channel::ipc::IpcSender;
use net_traits::bluetooth_thread::{BluetoothError, BluetoothResult};
use std::borrow::ToOwned;
use std::string::String;

const ADAPTER_ERROR: &'static str = "No adapter found";

const WRONG_DATA_SET_ERROR: &'static str = "Wrong data set name was provided";

const FAILED_SET_ERROR: &'static str = "Failed to set an attribute for testing";

macro_rules! set_attribute_or_return_error(
    ($function:expr, $sender:expr) => (
        match $function {
            Ok(_) => (),
            Err(_) => return drop($sender.send(Err(BluetoothError::Type(FAILED_SET_ERROR.to_string())))),
        }
    );
);

pub fn test(manager: &mut BluetoothManager, data_set_name: String, sender: IpcSender<BluetoothResult<()>>) {
    match data_set_name.as_str() {
        "NotPresentAdapter" => {
            match manager.get_or_create_adapter().as_ref() {
                Some(ref adapter) => {
                    set_attribute_or_return_error!(adapter.set_name(String::from("NotPresentAdapter")), sender);
                    set_attribute_or_return_error!(adapter.set_present(false), sender);
                    set_attribute_or_return_error!(adapter.set_discoverable(true), sender);
                    set_attribute_or_return_error!(adapter.set_powered(true), sender);
                },
                None => return drop(sender.send(Err(BluetoothError::Type(ADAPTER_ERROR.to_string())))),
            }
        },
        "NotPoweredAdapter" => {
            match manager.get_or_create_adapter().as_ref() {
                Some(ref adapter) => {
                    set_attribute_or_return_error!(adapter.set_name(String::from("NotPoweredAdapter")), sender);
                    set_attribute_or_return_error!(adapter.set_discoverable(true), sender);
                },
                None => return drop(sender.send(Err(BluetoothError::Type(ADAPTER_ERROR.to_string())))),
            }
        },
        "EmptyAdapter" => {
            match manager.get_or_create_adapter().as_ref() {
                Some(ref adapter) => {
                    set_attribute_or_return_error!(adapter.set_name(String::from("EmptyAdapter")), sender);
                    set_attribute_or_return_error!(adapter.set_powered(true), sender);
                    set_attribute_or_return_error!(adapter.set_discoverable(true), sender);
                },
                None => return drop(sender.send(Err(BluetoothError::Type(ADAPTER_ERROR.to_string())))),
            }
        },
        "FailStartDiscoveryAdapter" => {
            match manager.get_or_create_adapter().as_ref() {
                Some(ref adapter) => {
                    set_attribute_or_return_error!(adapter.set_name(String::from("FailStartDiscoveryAdapter")),
                                               sender);
                    set_attribute_or_return_error!(adapter.set_discoverable(true), sender);
                },
                None => return drop(sender.send(Err(BluetoothError::Type(ADAPTER_ERROR.to_string())))),
            }
        },
        "GlucoseHeartRateAdapter" => {
            let random_id_1 = manager.generate_device_id();
            let random_id_2 = manager.generate_device_id();
            match manager.get_or_create_adapter().as_ref() {
                Some(adapter) => {
                    set_attribute_or_return_error!(adapter.set_name(String::from("GlucoseHeartRateAdapter"
                                               .to_owned())),
                                               sender);
                    set_attribute_or_return_error!(adapter.set_powered(true), sender);
                    set_attribute_or_return_error!(adapter.set_discoverable(true), sender);
                    let glucose_device = BluetoothDevice::create_device(adapter.clone(), random_id_1);
                    set_attribute_or_return_error!(glucose_device.set_name("Glucose Device".to_owned()), sender);
                    set_attribute_or_return_error!(glucose_device.set_address("00:00:00:00:00:01".to_owned()),
                                               sender);
                    // Generic Acces, Glucose UUID, Tx Power
                    set_attribute_or_return_error!(glucose_device.set_uuids(
                    vec!("00001800-0000-1000-8000-00805f9b34fb".to_owned(),
                         "00001808-0000-1000-8000-00805f9b34fb".to_owned(),
                         "00001804-0000-1000-8000-00805f9b34fb".to_owned())),
                                              sender);
                    let heart_rate_device = BluetoothDevice::create_device(adapter.clone(), random_id_2);
                    set_attribute_or_return_error!(heart_rate_device.set_name("Heart Rate Device".to_owned()),
                                               sender);
                    set_attribute_or_return_error!(heart_rate_device.set_address("00:00:00:00:00:02".to_owned()),
                                               sender);
                    set_attribute_or_return_error!(heart_rate_device.set_connectable(true), sender);
                    // Generic Acces, Heart Rate UUID
                    set_attribute_or_return_error!(heart_rate_device.set_uuids(
                    vec!("00001800-0000-1000-8000-00805f9b34fb".to_owned(),
                         "0000180d-0000-1000-8000-00805f9b34fb".to_owned())),
                                               sender);
                },
                None => return drop(sender.send(Err(BluetoothError::Type(ADAPTER_ERROR.to_string())))),
            }
        },
        "UnicodeDeviceAdapter" => {
            let random_id = manager.generate_device_id();
            match manager.get_or_create_adapter().as_ref() {
                Some(adapter) => {
                    set_attribute_or_return_error!(adapter.set_name(String::from("GlucoseHeartRateAdapter"
                                                                .to_owned())),
                                               sender);
                    set_attribute_or_return_error!(adapter.set_powered(true), sender);
                    set_attribute_or_return_error!(adapter.set_discoverable(true), sender);
                    let unicode_device = BluetoothDevice::create_device(adapter.clone(), random_id);
                    set_attribute_or_return_error!(unicode_device.set_name(
                    "❤❤❤❤❤❤❤❤❤".to_owned()),
                                               sender);
                    set_attribute_or_return_error!(unicode_device.set_address("00:00:00:00:00:03".to_owned()),
                                                   sender);
                },
                None => return drop(sender.send(Err(BluetoothError::Type(ADAPTER_ERROR.to_string())))),
            }
        },
        "BlacklistedServicesAdapter" => {
            let random_id = manager.generate_device_id();
            match manager.get_or_create_adapter().as_ref() {
                Some(adapter) => {
                    set_attribute_or_return_error!(adapter.set_name(String::from("BlacklistedServicesAdapter"
                                                                             .to_owned())),
                                                   sender);
                    set_attribute_or_return_error!(adapter.set_powered(true), sender);
                    set_attribute_or_return_error!(adapter.set_discoverable(true), sender);
                    let blacklisted_services_device = BluetoothDevice::create_device(adapter.clone(), random_id);
                    set_attribute_or_return_error!(
                        blacklisted_services_device.set_name("Blacklisted Services Device".to_owned()), sender);
                    set_attribute_or_return_error!(
                        blacklisted_services_device.set_address("00:00:00:00:00:04".to_owned()), sender);
                    set_attribute_or_return_error!(blacklisted_services_device.set_connectable(true), sender);
                    set_attribute_or_return_error!(blacklisted_services_device.set_uuids(
                        vec!("00001812-0000-1000-8000-00805f9b34fb".to_owned(),
                             "00001530-1212-efde-1523-785feabcd123".to_owned(),
                             "f000ffc0-0451-4000-b000-000000000000".to_owned())),
                                                   sender);
                    let _human_interface_service =
                        BluetoothGATTService::new_mock(blacklisted_services_device.clone(),
                                                       "00001812-0000-1000-8000-00805f9b34fb".to_owned());
                    let _firmware_update_service =
                        BluetoothGATTService::new_mock(blacklisted_services_device.clone(),
                                                       "00001530-1212-efde-1523-785feabcd123".to_owned());
                    let _over_the_air_download_service =
                        BluetoothGATTService::new_mock(blacklisted_services_device.clone(),
                                                       "f000ffc0-0451-4000-b000-000000000000".to_owned());
                },
                None => return drop(sender.send(Err(BluetoothError::Type(ADAPTER_ERROR.to_string())))),
            }
        },
        "MissingCharacteristicGenericAccessAdapter" => {
            let random_id_device = manager.generate_device_id();
            let random_id_service1 = manager.generate_device_id();
            let random_id_service2 = manager.generate_device_id();
            match manager.get_or_create_adapter().as_ref() {
                Some(adapter) => {
                    set_attribute_or_return_error!(adapter.set_name(
                    String::from("MissingCharacteristicGenericAccessAdapter".to_owned())),
                                                   sender);
                    set_attribute_or_return_error!(adapter.set_powered(true), sender);
                    set_attribute_or_return_error!(adapter.set_discoverable(true), sender);
                    let heart_rate_device = BluetoothDevice::create_device(adapter.clone(), random_id_device);
                    set_attribute_or_return_error!(heart_rate_device.set_name("Heart Rate Device".to_owned()),
                                                   sender);
                    set_attribute_or_return_error!(heart_rate_device.set_address("00:00:00:00:00:05".to_owned()),
                                                   sender);
                    set_attribute_or_return_error!(heart_rate_device.set_connectable(true), sender);
                    // Generic Acces, Heart Rate UUID
                    set_attribute_or_return_error!(heart_rate_device.set_uuids(
                        vec!("00001800-0000-1000-8000-00805f9b34fb".to_owned(),
                             "0000180d-0000-1000-8000-00805f9b34fb".to_owned())),
                                                   sender);
                    let generic_access_service =
                        BluetoothGATTService::new_mock(heart_rate_device.clone(),
                                                       random_id_service1.to_owned());
                    set_attribute_or_return_error!(generic_access_service.set_uuid("00001800-0000-1000-8000-00805f9b34fb".to_owned()), sender);
                    set_attribute_or_return_error!(generic_access_service.set_primary(true), sender);
                    let heart_rate_service =
                        BluetoothGATTService::new_mock(heart_rate_device.clone(),
                                                       random_id_service2.to_owned());
                    set_attribute_or_return_error!(heart_rate_service.set_uuid("0000180d-0000-1000-8000-00805f9b34fb".to_owned()), sender);
                    set_attribute_or_return_error!(heart_rate_service.set_primary(true), sender);
                },
                None => return drop(sender.send(Err(BluetoothError::Type(ADAPTER_ERROR.to_string())))),
            }
        },
        "MissingDescriptorGenericAccessAdapter" => {
            let random_id_device = manager.generate_device_id();
            let random_id_service1 = manager.generate_device_id();
            let random_id_service2 = manager.generate_device_id();
            let random_id_characteristic1 = manager.generate_device_id();
            let random_id_characteristic2 = manager.generate_device_id();
            let random_id_characteristic3 = manager.generate_device_id();
            let random_id_characteristic4 = manager.generate_device_id();
            let random_id_characteristic5 = manager.generate_device_id();
            match manager.get_or_create_adapter().as_ref() {
                Some(adapter) => {
                    set_attribute_or_return_error!(adapter.set_name(
                    String::from("MissingDescriptorGenericAccessAdapter".to_owned())),
                                                   sender);
                    set_attribute_or_return_error!(adapter.set_powered(true), sender);
                    set_attribute_or_return_error!(adapter.set_discoverable(true), sender);
                    let heart_rate_device = BluetoothDevice::create_device(adapter.clone(), random_id_device);
                    set_attribute_or_return_error!(heart_rate_device.set_name("Heart Rate Device".to_owned()),
                                                   sender);
                    set_attribute_or_return_error!(heart_rate_device.set_address("00:00:00:00:00:06".to_owned()),
                                                   sender);
                    set_attribute_or_return_error!(heart_rate_device.set_connectable(true), sender);
                    // Generic Acces, Heart Rate UUID
                    set_attribute_or_return_error!(heart_rate_device.set_uuids(
                        vec!("00001800-0000-1000-8000-00805f9b34fb".to_owned(),
                             "0000180d-0000-1000-8000-00805f9b34fb".to_owned())),
                                                   sender);
                    let generic_access_service =
                        BluetoothGATTService::new_mock(heart_rate_device.clone(),
                                                       random_id_service1.to_owned());
                    set_attribute_or_return_error!(generic_access_service.set_uuid("00001800-0000-1000-8000-00805f9b34fb".to_owned()), sender);
                    set_attribute_or_return_error!(generic_access_service.set_primary(true), sender);
                    let heart_rate_service =
                        BluetoothGATTService::new_mock(heart_rate_device.clone(),
                                                       random_id_service2.to_owned());
                    set_attribute_or_return_error!(heart_rate_service.set_uuid("0000180d-0000-1000-8000-00805f9b34fb".to_owned()), sender);
                    set_attribute_or_return_error!(heart_rate_service.set_primary(true), sender);

                    let device_name_characteristic =
                        BluetoothGATTCharacteristic::new_mock(generic_access_service.clone(),
                                                              random_id_characteristic1.to_owned());
                    set_attribute_or_return_error!(device_name_characteristic.set_uuid("00002a00-0000-1000-8000-00805f9b34fb".to_owned()), sender);
                    set_attribute_or_return_error!(device_name_characteristic.write_value(vec![1]), sender);

                    let pheripheral_privacy_flag_characteristic =
                        BluetoothGATTCharacteristic::new_mock(generic_access_service.clone(),
                                                              random_id_characteristic2.to_owned());
                    set_attribute_or_return_error!(pheripheral_privacy_flag_characteristic.set_uuid("00002a02-0000-1000-8000-00805f9b34fb".to_owned()), sender);
                    set_attribute_or_return_error!(pheripheral_privacy_flag_characteristic.write_value(vec![2]),
                                                   sender);

                    let heart_rate_measurement_characteristic =
                        BluetoothGATTCharacteristic::new_mock(heart_rate_service.clone(),
                                                              random_id_characteristic3.to_owned());
                    set_attribute_or_return_error!(heart_rate_measurement_characteristic.set_uuid("00002a37-0000-1000-8000-00805f9b34fb".to_owned()), sender);
                    set_attribute_or_return_error!(heart_rate_measurement_characteristic.write_value(vec![3]),
                                                   sender);

                    let body_sensor_location_characteristic_1 =
                        BluetoothGATTCharacteristic::new_mock(heart_rate_service.clone(),
                                                              random_id_characteristic4.to_owned());
                    set_attribute_or_return_error!(body_sensor_location_characteristic_1.set_uuid("00002a38-0000-1000-8000-00805f9b34fb".to_owned()), sender);
                    set_attribute_or_return_error!(body_sensor_location_characteristic_1.write_value(vec![4]),
                                                   sender);

                    let body_sensor_location_characteristic_2 =
                        BluetoothGATTCharacteristic::new_mock(heart_rate_service.clone(),
                                                              random_id_characteristic5.to_owned());
                    set_attribute_or_return_error!(body_sensor_location_characteristic_2.set_uuid("00002a38-0000-1000-8000-00805f9b34fb".to_owned()), sender);
                    set_attribute_or_return_error!(body_sensor_location_characteristic_2.write_value(vec![5]),
                                                   sender);
                },
                None => return drop(sender.send(Err(BluetoothError::Type(ADAPTER_ERROR.to_string())))),
            }
        },
        "ExcludedForWritesCharacteristicAdapter" => {
            let random_id = manager.generate_device_id();
            match manager.get_or_create_adapter().as_ref() {
                Some(adapter) => {
                    set_attribute_or_return_error!(adapter.set_name(
                    String::from("ExcludedForWritesCharacteristicAdapter".to_owned())),
                                                   sender);
                    set_attribute_or_return_error!(adapter.set_powered(true), sender);
                    set_attribute_or_return_error!(adapter.set_discoverable(true), sender);
                    let device = BluetoothDevice::create_device(adapter.clone(), random_id);
                    set_attribute_or_return_error!(device.set_name("Mock Device".to_owned()), sender);
                    set_attribute_or_return_error!(device.set_address("00:00:00:00:00:07".to_owned()), sender);
                    set_attribute_or_return_error!(device.set_connectable(true), sender);
                    // Reconnection Address, Peripheral Privacy Flag
                    set_attribute_or_return_error!(device.set_uuids(
                        vec!("00001800-0000-1000-8000-00805f9b34fb".to_owned())),
                                                   sender);
                    let service =
                        BluetoothGATTService::new_mock(device.clone(),
                                                       "00001800-0000-1000-8000-00805f9b34fb".to_owned());

                    let pheripheral_privacy_flag_characteristic =
                        BluetoothGATTCharacteristic::new_mock(service.clone(),
                                                              "00002a02-0000-1000-8000-00805f9b34fb".to_owned());
                    set_attribute_or_return_error!(pheripheral_privacy_flag_characteristic.write_value(vec![8]),
                                                   sender);
                },
                None => return drop(sender.send(Err(BluetoothError::Type(ADAPTER_ERROR.to_string())))),
            }
        },
        "BlacklistedCharacteristicsAdapter" => {
            let random_id = manager.generate_device_id();
            match manager.get_or_create_adapter().as_ref() {
                Some(adapter) => {
                    set_attribute_or_return_error!(adapter.set_name(
                    String::from("BlacklistedCharacteristicsAdapter".to_owned())),
                                                   sender);
                    set_attribute_or_return_error!(adapter.set_powered(true), sender);
                    set_attribute_or_return_error!(adapter.set_discoverable(true), sender);
                    let device = BluetoothDevice::create_device(adapter.clone(), random_id);
                    set_attribute_or_return_error!(device.set_name("Mock Device".to_owned()), sender);
                    set_attribute_or_return_error!(device.set_address("00:00:00:00:00:08".to_owned()), sender);
                    set_attribute_or_return_error!(device.set_connectable(true), sender);
                    // Reconnection Address, Serial Number String
                    set_attribute_or_return_error!(device.set_uuids(
                        vec!("00001800-0000-1000-8000-00805f9b34fb".to_owned())),
                                                   sender);
                    let service =
                        BluetoothGATTService::new_mock(device.clone(),
                                                       "00001800-0000-1000-8000-00805f9b34fb".to_owned());

                    let reconnection_address_characteristic =
                        BluetoothGATTCharacteristic::new_mock(service.clone(),
                                                              "00002a03-0000-1000-8000-00805f9b34fb".to_owned());
                    set_attribute_or_return_error!(reconnection_address_characteristic.write_value(vec![6]),
                                                   sender);

                    let serial_number_string_characteristic =
                        BluetoothGATTCharacteristic::new_mock(service.clone(),
                                                              "00002a25-0000-1000-8000-00805f9b34fb".to_owned());
                    set_attribute_or_return_error!(serial_number_string_characteristic.write_value(vec![7]),
                                                   sender);
                },
                None => return drop(sender.send(Err(BluetoothError::Type(ADAPTER_ERROR.to_string())))),
            }
        },
        "CompletedAdapter" => {
            let random_id = manager.generate_device_id();
            match manager.get_or_create_adapter().as_ref() {
                Some(adapter) => {
                    set_attribute_or_return_error!(adapter.set_name(String::from("CompletedAdapter".to_owned())),
                                                   sender);
                    set_attribute_or_return_error!(adapter.set_powered(true), sender);
                    set_attribute_or_return_error!(adapter.set_discoverable(true), sender);
                    let heart_rate_device = BluetoothDevice::create_device(adapter.clone(), random_id);
                    set_attribute_or_return_error!(heart_rate_device.set_name("Heart Rate Device".to_owned()),
                                                   sender);
                    set_attribute_or_return_error!(heart_rate_device.set_address("00:00:00:00:00:09".to_owned()),
                                                   sender);
                    set_attribute_or_return_error!(heart_rate_device.set_connectable(true), sender);
                    // Generic Acces, Heart Rate UUID
                    set_attribute_or_return_error!(heart_rate_device.set_uuids(
                        vec!("00001800-0000-1000-8000-00805f9b34fb".to_owned(),
                             "0000180d-0000-1000-8000-00805f9b34fb".to_owned())),
                                                   sender);
                    let generic_access_service =
                        BluetoothGATTService::new_mock(heart_rate_device.clone(),
                                                       "00001800-0000-1000-8000-00805f9b34fb".to_owned());
                    let heart_rate_service =
                        BluetoothGATTService::new_mock(heart_rate_device.clone(),
                                                       "0000180d-0000-1000-8000-00805f9b34fb".to_owned());

                    let device_name_characteristic =
                        BluetoothGATTCharacteristic::new_mock(generic_access_service.clone(),
                                                              "00002A00-0000-1000-8000-00805f9b34fb".to_owned());
                    set_attribute_or_return_error!(device_name_characteristic.write_value(vec![9]), sender);

                    let pheripheral_privacy_flag_characteristic =
                        BluetoothGATTCharacteristic::new_mock(generic_access_service.clone(),
                                                              "00002A02-0000-1000-8000-00805f9b34fb".to_owned());
                    set_attribute_or_return_error!(pheripheral_privacy_flag_characteristic.write_value(vec![10]),
                                                   sender);

                    let heart_rate_measurement_characteristic =
                        BluetoothGATTCharacteristic::new_mock(heart_rate_service.clone(),
                                                              "00002a37-0000-1000-8000-00805f9b34fb".to_owned());
                    set_attribute_or_return_error!(heart_rate_measurement_characteristic.write_value(vec![11]),
                                                   sender);

                    let body_sensor_location_characteristic_1 =
                        BluetoothGATTCharacteristic::new_mock(heart_rate_service.clone(),
                                                              "00002a38-0000-1000-8000-00805f9b34fb".to_owned());
                    set_attribute_or_return_error!(body_sensor_location_characteristic_1.write_value(vec![12]),
                                                   sender);

                    let body_sensor_location_characteristic_2 =
                        BluetoothGATTCharacteristic::new_mock(heart_rate_service.clone(),
                                                              "00002a38-0000-1000-8000-00805f9b34fb".to_owned());
                    set_attribute_or_return_error!(body_sensor_location_characteristic_2.write_value(vec![13]),
                                                   sender);

                    let desc_for_hrmeasurement_descriptor =
                        BluetoothGATTDescriptor::new_mock(heart_rate_measurement_characteristic.clone(),
                                                          "00002901-0000-1000-8000-00805f9b34fb".to_owned());
                    set_attribute_or_return_error!(desc_for_hrmeasurement_descriptor.write_value(vec![14]),
                                                   sender);

                    let desc_for_bslocation_descriptor =
                        BluetoothGATTDescriptor::new_mock(body_sensor_location_characteristic_1.clone(),
                                                          "00002901-0000-1000-8000-00805f9b34fb".to_owned());
                    set_attribute_or_return_error!(desc_for_bslocation_descriptor.write_value(vec![15]),
                                                   sender);
                },
                None => return drop(sender.send(Err(BluetoothError::Type(ADAPTER_ERROR.to_string())))),
            }
        },
        _ => return drop(sender.send(Err(BluetoothError::Type(WRONG_DATA_SET_ERROR.to_string())))),
    }
    return drop(sender.send(Ok(())));
}
