#![allow(clippy::let_unit_value)]
#![allow(missing_docs)]
#![allow(unused)]

use std::collections::HashMap;
use std::ffi::c_ulong;
use std::os::raw::{c_char, c_void};

use objc::rc::autoreleasepool;
use objc::runtime::{Object, BOOL, NO};
use objc::{msg_send, sel, sel_impl};
use objc_foundation::{
    object_struct, INSData, INSDictionary, INSFastEnumeration, INSObject, INSString, NSArray, NSData, NSDictionary,
    NSObject, NSString,
};
use objc_id::{Id, ShareId};

use super::delegates::{CentralDelegate, PeripheralDelegate};
use crate::btuuid::BluetoothUuidExt;
use crate::{AdvertisementData, CharacteristicProperties, ManufacturerData, Uuid};

#[allow(non_camel_case_types)]
pub type id = *mut Object;

pub type NSInteger = isize;
pub type NSUInteger = usize;

#[allow(non_upper_case_globals)]
pub const nil: *mut Object = std::ptr::null_mut();

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CBManagerState(pub NSInteger);

impl CBManagerState {
    pub const UNKNOWN: CBManagerState = CBManagerState(0);
    pub const RESETTING: CBManagerState = CBManagerState(1);
    pub const UNSUPPORTED: CBManagerState = CBManagerState(2);
    pub const UNAUTHORIZED: CBManagerState = CBManagerState(3);
    pub const POWERED_OFF: CBManagerState = CBManagerState(4);
    pub const POWERED_ON: CBManagerState = CBManagerState(5);
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CBManagerAuthorization(pub NSInteger);

impl CBManagerAuthorization {
    pub const NOT_DETERMINED: CBManagerAuthorization = CBManagerAuthorization(0);
    pub const RESTRICTED: CBManagerAuthorization = CBManagerAuthorization(1);
    pub const DENIED: CBManagerAuthorization = CBManagerAuthorization(2);
    pub const ALLOWED_ALWAYS: CBManagerAuthorization = CBManagerAuthorization(3);
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CBCentralManagerFeatures(pub NSUInteger);

impl CBCentralManagerFeatures {
    pub const EXTENDED_SCAN_AND_CONNECT: CBCentralManagerFeatures = CBCentralManagerFeatures(1);
}

#[non_exhaustive]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CBCharacteristicWriteType {
    #[default]
    WithResponse = 0,
    WithoutResponse = 1,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CBPeripheralState(pub NSInteger);

impl CBPeripheralState {
    pub const DISCONNECTED: CBPeripheralState = CBPeripheralState(0);
    pub const CONNECTING: CBPeripheralState = CBPeripheralState(1);
    pub const CONNECTED: CBPeripheralState = CBPeripheralState(2);
    pub const DISCONNECTING: CBPeripheralState = CBPeripheralState(3);
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CBCharacteristicProperties(pub NSUInteger);

impl CBCharacteristicProperties {
    pub const BROADCAST: CBCharacteristicProperties = CBCharacteristicProperties(0b0000_0001);
    pub const READ: CBCharacteristicProperties = CBCharacteristicProperties(0b0000_0010);
    pub const WRITE_WITHOUT_RESPONSE: CBCharacteristicProperties = CBCharacteristicProperties(0b0000_0100);
    pub const WRITE: CBCharacteristicProperties = CBCharacteristicProperties(0b0000_1000);
    pub const NOTIFY: CBCharacteristicProperties = CBCharacteristicProperties(0b0001_0000);
    pub const INDICATE: CBCharacteristicProperties = CBCharacteristicProperties(0b0010_0000);
    pub const AUTHENTICATED_SIGNED_WRITES: CBCharacteristicProperties = CBCharacteristicProperties(0b0100_0000);
    pub const EXTENDED_PROPERTIES: CBCharacteristicProperties = CBCharacteristicProperties(0b1000_0000);
    pub const NOTIFY_ENCRYPTION_REQUIRED: CBCharacteristicProperties = CBCharacteristicProperties(0b0001_0000_0000);
    pub const INDICATE_ENCRYPTION_REQUIRED: CBCharacteristicProperties = CBCharacteristicProperties(0b0010_0000_0000);
}

impl From<CBCharacteristicProperties> for CharacteristicProperties {
    fn from(val: CBCharacteristicProperties) -> Self {
        CharacteristicProperties::from_bits((val.0 & 0xff) as u32)
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CBError(pub NSInteger);

impl CBError {
    pub const UNKNOWN: CBError = CBError(0);
    pub const INVALID_PARAMETERS: CBError = CBError(1);
    pub const INVALID_HANDLE: CBError = CBError(2);
    pub const NOT_CONNECTED: CBError = CBError(3);
    pub const OUT_OF_SPACE: CBError = CBError(4);
    pub const OPERATION_CANCELLED: CBError = CBError(5);
    pub const CONNECTION_TIMEOUT: CBError = CBError(6);
    pub const PERIPHERAL_DISCONNECTED: CBError = CBError(7);
    pub const UUID_NOT_ALLOWED: CBError = CBError(8);
    pub const ALREADY_ADVERTISING: CBError = CBError(9);
    pub const CONNECTION_FAILED: CBError = CBError(10);
    pub const CONNECTION_LIMIT_REACHED: CBError = CBError(11);
    pub const UNKOWN_DEVICE: CBError = CBError(12);
    pub const OPERATION_NOT_SUPPORTED: CBError = CBError(13);
    pub const PEER_REMOVED_PAIRING_INFORMATION: CBError = CBError(14);
    pub const ENCRYPTION_TIMED_OUT: CBError = CBError(15);
    pub const TOO_MANY_LE_PAIRED_DEVICES: CBError = CBError(16);
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CBATTError(pub NSInteger);

impl CBATTError {
    pub const SUCCESS: CBATTError = CBATTError(0);
    pub const INVALID_HANDLE: CBATTError = CBATTError(1);
    pub const READ_NOT_PERMITTED: CBATTError = CBATTError(2);
    pub const WRITE_NOT_PERMITTED: CBATTError = CBATTError(3);
    pub const INVALID_PDU: CBATTError = CBATTError(4);
    pub const INSUFFICIENT_AUTHENTICATION: CBATTError = CBATTError(5);
    pub const REQUEST_NOT_SUPPORTED: CBATTError = CBATTError(6);
    pub const INVALID_OFFSET: CBATTError = CBATTError(7);
    pub const INSUFFICIENT_AUTHORIZATION: CBATTError = CBATTError(8);
    pub const PREPARE_QUEUE_FULL: CBATTError = CBATTError(9);
    pub const ATTRIBUTE_NOT_FOUND: CBATTError = CBATTError(10);
    pub const ATTRIBUTE_NOT_LONG: CBATTError = CBATTError(11);
    pub const INSUFFICIENT_ENCRYPTION_KEY_SIZE: CBATTError = CBATTError(12);
    pub const INVALID_ATTRIBUTE_VALUE_LENGTH: CBATTError = CBATTError(13);
    pub const UNLIKELY_ERROR: CBATTError = CBATTError(14);
    pub const INSUFFICIENT_ENCRYPTION: CBATTError = CBATTError(15);
    pub const UNSUPPORTED_GROUP_TYPE: CBATTError = CBATTError(16);
    pub const INSUFFICIENT_RESOURCES: CBATTError = CBATTError(17);
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NSStreamStatus(pub NSInteger);

impl NSStreamStatus {
    pub const NOT_OPEN: NSStreamStatus = NSStreamStatus(0);
    pub const OPENING: NSStreamStatus = NSStreamStatus(1);
    pub const OPEN: NSStreamStatus = NSStreamStatus(2);
    pub const READING: NSStreamStatus = NSStreamStatus(3);
    pub const WRITING: NSStreamStatus = NSStreamStatus(4);
    pub const AT_END: NSStreamStatus = NSStreamStatus(5);
    pub const CLOSED: NSStreamStatus = NSStreamStatus(6);
    pub const ERROR: NSStreamStatus = NSStreamStatus(7);
}


#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NSStreamEvent(pub NSInteger);

impl NSStreamEvent {
    pub const NONE: NSStreamEvent = NSStreamEvent(0);
    pub const OPEN_COMPLETED: NSStreamEvent = NSStreamEvent(1<<0);
    pub const HAS_BYTES_AVAILABLE: NSStreamEvent = NSStreamEvent(1<<1);
    pub const HAS_SPACE_AVAILABLE: NSStreamEvent = NSStreamEvent(1<<2);
    pub const ERROR_OCCURRED: NSStreamEvent = NSStreamEvent(1<<3);
    pub const END_ENCOUNTERED: NSStreamEvent = NSStreamEvent(1<<4);
}

impl AdvertisementData {
    pub(super) fn from_nsdictionary(adv_data: &ShareId<NSDictionary<NSString, NSObject>>) -> Self {
        let is_connectable = adv_data
            .object_for(&*INSString::from_str("kCBAdvDataIsConnectable"))
            .map_or(false, |val| unsafe {
                let n: BOOL = msg_send![val, boolValue];
                n != NO
            });

        let local_name = adv_data
            .object_for(&*INSString::from_str("kCBAdvDataLocalName"))
            .map(|val| unsafe { (*(val as *const NSObject).cast::<NSString>()).as_str().to_string() });

        let manufacturer_data = adv_data
            .object_for(&*INSString::from_str("kCBAdvDataManufacturerData"))
            .map(|val| unsafe { (*(val as *const NSObject).cast::<NSData>()).bytes() })
            .and_then(|val| {
                (val.len() >= 2).then(|| ManufacturerData {
                    company_id: u16::from_le_bytes(val[0..2].try_into().unwrap()),
                    data: val[2..].to_vec(),
                })
            });

        let tx_power_level: Option<i16> = adv_data
            .object_for(&*INSString::from_str("kCBAdvDataTxPowerLevel"))
            .map(|val| unsafe { msg_send![val, shortValue] });

        let service_data = if let Some(val) = adv_data.object_for(&*INSString::from_str("kCBAdvDataServiceData")) {
            unsafe {
                let val: &NSDictionary<CBUUID, NSData> = &*(val as *const NSObject).cast();
                let mut res = HashMap::with_capacity(val.count());
                for k in val.enumerator() {
                    res.insert(k.to_uuid(), val.object_for(k).unwrap().bytes().to_vec());
                }
                res
            }
        } else {
            HashMap::new()
        };

        let services = adv_data
            .object_for(&*INSString::from_str("kCBAdvDataServiceUUIDs"))
            .into_iter()
            .chain(
                adv_data
                    .object_for(&*INSString::from_str("kCBAdvDataHashedServiceUUIDs"))
                    .into_iter(),
            )
            .flat_map(|x| {
                let val: &NSArray<CBUUID> = unsafe { &*(x as *const NSObject).cast() };
                val.enumerator()
            })
            .map(CBUUID::to_uuid)
            .collect();

        AdvertisementData {
            local_name,
            manufacturer_data,
            services,
            service_data,
            tx_power_level,
            is_connectable,
        }
    }
}

#[link(name = "CoreBluetooth", kind = "framework")]
extern "C" {
    pub static _dispatch_queue_attr_concurrent: Object;
    pub fn dispatch_queue_attr_make_with_autorelease_frequency(attr: id, frequency: c_ulong) -> id;
    pub fn dispatch_queue_create(label: *const c_char, attr: id) -> id;
    pub fn dispatch_get_global_queue(identifier: isize, flags: usize) -> id;
    pub fn dispatch_release(object: id) -> c_void;
}

pub const QOS_CLASS_USER_INTERACTIVE: isize = 0x21;
pub const QOS_CLASS_USER_INITIATED: isize = 0x19;
pub const QOS_CLASS_DEFAULT: isize = 0x15;
pub const QOS_CLASS_UTILITY: isize = 0x11;
pub const QOS_CLASS_BACKGROUND: isize = 0x09;
pub const QOS_CLASS_UNSPECIFIED: isize = 0x00;

pub fn id_or_nil<T>(val: Option<&T>) -> *const T {
    match val {
        Some(x) => x,
        None => std::ptr::null(),
    }
}

pub unsafe fn option_from_ptr<T: objc::Message, O: objc_id::Ownership>(ptr: *mut T) -> Option<Id<T, O>> {
    (!ptr.is_null()).then(|| Id::from_ptr(ptr))
}

object_struct!(NSError);
object_struct!(NSUUID);
object_struct!(CBUUID);
object_struct!(CBCentralManager);
object_struct!(CBPeripheral);
object_struct!(CBService);
object_struct!(CBCharacteristic);
object_struct!(CBDescriptor);
object_struct!(CBL2CAPChannel);
object_struct!(CBPeer);
object_struct!(NSInputStream);
object_struct!(NSOutputStream);

impl NSError {
    pub fn code(&self) -> NSInteger {
        unsafe { msg_send![self, code] }
    }

    pub fn domain(&self) -> ShareId<NSString> {
        autoreleasepool(move || unsafe { Id::from_ptr(msg_send![self, domain]) })
    }

    pub fn user_info(&self) -> ShareId<NSDictionary<NSString, NSObject>> {
        autoreleasepool(move || unsafe { Id::from_ptr(msg_send![self, userInfo]) })
    }

    pub fn localized_description(&self) -> ShareId<NSString> {
        autoreleasepool(move || unsafe { Id::from_ptr(msg_send![self, localizedDescription]) })
    }

    pub fn localized_recovery_options(&self) -> Option<ShareId<NSArray<NSString>>> {
        autoreleasepool(move || unsafe { option_from_ptr(msg_send![self, localizedRecoveryOptions]) })
    }

    pub fn localized_recovery_suggestion(&self) -> Option<ShareId<NSString>> {
        autoreleasepool(move || unsafe { option_from_ptr(msg_send![self, localizedRecoverySuggestion]) })
    }

    pub fn localized_failure_reason(&self) -> Option<ShareId<NSString>> {
        autoreleasepool(move || unsafe { option_from_ptr(msg_send![self, localizedFailureReason]) })
    }

    pub fn help_anchor(&self) -> Option<ShareId<NSString>> {
        autoreleasepool(move || unsafe { option_from_ptr(msg_send![self, helpAnchor]) })
    }

    pub fn underlying_errors(&self) -> ShareId<NSArray<NSError>> {
        autoreleasepool(move || unsafe { Id::from_ptr(msg_send![self, underlyingErrors]) })
    }
}

impl NSUUID {
    pub fn from_uuid(uuid: Uuid) -> Id<Self> {
        unsafe {
            let obj: *mut Self = msg_send![Self::class(), alloc];
            Id::from_retained_ptr(msg_send![obj, initWithUUIDBytes: uuid.as_bytes()])
        }
    }

    pub fn to_uuid(&self) -> Uuid {
        let mut bytes = [0u8; 16];
        let _: () = unsafe { msg_send!(self, getUUIDBytes: &mut bytes) };
        Uuid::from_bytes(bytes)
    }
}

impl CBUUID {
    pub fn from_uuid(uuid: Uuid) -> Id<Self> {
        autoreleasepool(|| unsafe {
            let data = NSData::from_vec(uuid.as_bluetooth_bytes().to_vec());
            let obj: *mut Self = msg_send![Self::class(), UUIDWithData: &*data];
            Id::from_ptr(obj)
        })
    }

    pub fn to_uuid(&self) -> Uuid {
        autoreleasepool(move || {
            let data: ShareId<NSData> = unsafe { ShareId::from_ptr(msg_send!(self, data)) };
            Uuid::from_bluetooth_bytes(data.bytes())
        })
    }
}

impl CBCentralManager {
    pub fn with_delegate(delegate: &CentralDelegate, queue: id) -> Id<CBCentralManager> {
        unsafe {
            let obj: *mut Self = msg_send![Self::class(), alloc];
            Id::from_retained_ptr(msg_send![obj, initWithDelegate: delegate queue: queue])
        }
    }

    pub fn state(&self) -> CBManagerState {
        CBManagerState(unsafe { msg_send![self, state] })
    }

    pub fn authorization() -> CBManagerAuthorization {
        CBManagerAuthorization(unsafe { msg_send![Self::class(), authorization] })
    }

    pub fn connect_peripheral(&self, peripheral: &CBPeripheral, options: Option<&NSDictionary<NSString, NSObject>>) {
        unsafe { msg_send![self, connectPeripheral: peripheral options: id_or_nil(options)] }
    }

    pub fn cancel_peripheral_connection(&self, peripheral: &CBPeripheral) {
        unsafe { msg_send![self, cancelPeripheralConnection: peripheral] }
    }

    pub fn retrieve_connected_peripherals_with_services(
        &self,
        services: &NSArray<CBUUID>,
    ) -> Id<NSArray<CBPeripheral>> {
        autoreleasepool(move || unsafe {
            Id::from_ptr(msg_send![self, retrieveConnectedPeripheralsWithServices: services])
        })
    }

    pub fn retrieve_peripherals_with_identifiers(&self, identifiers: &NSArray<NSUUID>) -> Id<NSArray<CBPeripheral>> {
        autoreleasepool(move || unsafe {
            Id::from_ptr(msg_send![self, retrievePeripheralsWithIdentifiers: identifiers])
        })
    }

    pub fn scan_for_peripherals_with_services(
        &self,
        services: Option<&NSArray<CBUUID>>,
        options: Option<&NSDictionary<NSString, NSObject>>,
    ) {
        unsafe { msg_send![self, scanForPeripheralsWithServices: id_or_nil(services) options: id_or_nil(options)] }
    }

    pub fn stop_scan(&self) {
        unsafe { msg_send![self, stopScan] }
    }

    pub fn is_scanning(&self) -> bool {
        let res: BOOL = unsafe { msg_send![self, isScanning] };
        res != NO
    }

    pub fn supports_features(&self, features: CBCentralManagerFeatures) -> bool {
        let res: BOOL = unsafe { msg_send![self, supportsFeatures: features.0] };
        res != NO
    }

    pub fn delegate(&self) -> Option<ShareId<CentralDelegate>> {
        autoreleasepool(move || unsafe { option_from_ptr(msg_send![self, delegate]) })
    }

    pub fn register_for_connection_events_with_options(&self, options: &NSDictionary<NSString, NSObject>) {
        unsafe { msg_send![self, registerForConnectionEventsWithOptions: options] }
    }
}

impl CBPeripheral {
    pub fn identifier(&self) -> ShareId<NSUUID> {
        autoreleasepool(move || unsafe { ShareId::from_ptr(msg_send![self, identifier]) })
    }

    pub fn name(&self) -> Option<ShareId<NSString>> {
        autoreleasepool(move || unsafe { option_from_ptr(msg_send![self, name]) })
    }

    pub fn delegate(&self) -> Option<ShareId<PeripheralDelegate>> {
        autoreleasepool(move || unsafe { option_from_ptr(msg_send![self, delegate]) })
    }

    pub fn set_delegate(&self, delegate: &PeripheralDelegate) {
        unsafe { msg_send![self, setDelegate: delegate] }
    }

    pub fn services(&self) -> Option<ShareId<NSArray<CBService>>> {
        autoreleasepool(move || unsafe { option_from_ptr(msg_send![self, services]) })
    }

    pub fn open_l2cap_channel(&self, psm: u16) {
        unsafe { msg_send![self, openL2CAPChannel: psm] }
    }
    
    pub fn discover_services(&self, services: Option<&NSArray<CBUUID>>) {
        unsafe { msg_send![self, discoverServices: id_or_nil(services)] }
    }

    pub fn discover_included_services(&self, service: &CBService, services: Option<&NSArray<CBUUID>>) {
        unsafe { msg_send![self, discoverIncludedServices: id_or_nil(services) forService: service] }
    }

    pub fn discover_characteristics(&self, service: &CBService, characteristics: Option<&NSArray<CBUUID>>) {
        unsafe { msg_send![self, discoverCharacteristics: id_or_nil(characteristics) forService: service] }
    }

    pub fn discover_descriptors(&self, characteristic: &CBCharacteristic) {
        unsafe { msg_send![self, discoverDescriptorsForCharacteristic: characteristic] }
    }

    pub fn read_characteristic_value(&self, characteristic: &CBCharacteristic) {
        unsafe { msg_send![self, readValueForCharacteristic: characteristic] }
    }

    pub fn read_descriptor_value(&self, descriptor: &CBDescriptor) {
        unsafe { msg_send![self, readValueForDescriptor: descriptor] }
    }

    pub fn write_characteristic_value(
        &self,
        characteristic: &CBCharacteristic,
        value: &NSData,
        write_type: CBCharacteristicWriteType,
    ) {
        let write_type: isize = write_type as isize;
        unsafe { msg_send![self, writeValue: value forCharacteristic: characteristic type: write_type] }
    }

    pub fn write_descriptor_value(&self, descriptor: &CBDescriptor, value: &NSData) {
        unsafe { msg_send![self, writeValue: value forDescriptor: descriptor] }
    }

    pub fn set_notify(&self, characteristic: &CBCharacteristic, enabled: bool) {
        unsafe { msg_send![self, setNotifyValue: enabled as BOOL forCharacteristic: characteristic] }
    }

    pub fn state(&self) -> CBPeripheralState {
        CBPeripheralState(unsafe { msg_send![self, state] })
    }

    pub fn read_rssi(&self) {
        unsafe { msg_send![self, readRSSI] }
    }
}

impl CBService {
    pub fn uuid(&self) -> ShareId<CBUUID> {
        autoreleasepool(move || unsafe { ShareId::from_ptr(msg_send![self, UUID]) })
    }

    pub fn peripheral(&self) -> ShareId<CBPeripheral> {
        autoreleasepool(move || unsafe { ShareId::from_ptr(msg_send![self, peripheral]) })
    }

    pub fn is_primary(&self) -> bool {
        let res: BOOL = unsafe { msg_send![self, isPrimary] };
        res != NO
    }

    pub fn characteristics(&self) -> Option<ShareId<NSArray<CBCharacteristic>>> {
        autoreleasepool(move || unsafe { option_from_ptr(msg_send![self, characteristics]) })
    }

    pub fn included_services(&self) -> Option<ShareId<NSArray<CBService>>> {
        autoreleasepool(move || unsafe { option_from_ptr(msg_send![self, includedServices]) })
    }
}

impl CBCharacteristic {
    pub fn uuid(&self) -> ShareId<CBUUID> {
        autoreleasepool(move || unsafe { ShareId::from_ptr(msg_send![self, UUID]) })
    }

    pub fn service(&self) -> ShareId<CBService> {
        autoreleasepool(move || unsafe { ShareId::from_ptr(msg_send![self, service]) })
    }

    pub fn value(&self) -> Option<ShareId<NSData>> {
        autoreleasepool(move || unsafe { option_from_ptr(msg_send![self, value]) })
    }

    pub fn descriptors(&self) -> Option<ShareId<NSArray<CBDescriptor>>> {
        autoreleasepool(move || unsafe { option_from_ptr(msg_send![self, descriptors]) })
    }

    pub fn properties(&self) -> CBCharacteristicProperties {
        CBCharacteristicProperties(unsafe { msg_send![self, properties] })
    }

    pub fn is_notifying(&self) -> bool {
        let res: BOOL = unsafe { msg_send![self, isNotifying] };
        res != NO
    }

    pub fn is_broadcasting(&self) -> bool {
        let res: BOOL = unsafe { msg_send![self, isBroadcasting] };
        res != NO
    }
}

impl CBDescriptor {
    pub fn uuid(&self) -> ShareId<CBUUID> {
        autoreleasepool(move || unsafe { ShareId::from_ptr(msg_send![self, UUID]) })
    }

    pub fn characteristic(&self) -> ShareId<CBCharacteristic> {
        autoreleasepool(move || unsafe { ShareId::from_ptr(msg_send![self, characteristic]) })
    }

    pub fn value(&self) -> Option<ShareId<NSObject>> {
        autoreleasepool(move || unsafe { option_from_ptr(msg_send![self, value]) })
    }
}

impl NSInputStream {
    pub fn open(&self) -> c_void {
        unsafe { msg_send![self, open] }
    }
    pub fn close(&self) -> c_void {
        unsafe { msg_send![self, close] }
    }
    pub fn delegate(&self, id: ShareId<NSObject>) -> c_void {
        unsafe { msg_send![self, delegate: id] }
    }
    pub fn schedule_in_run_loop(&self, run_loop: NSObject, run_loop_mode: NSString) -> c_void {
        unsafe { msg_send![self, scheduleInRunLoop: run_loop forMode: run_loop_mode] }
    }
    pub fn remove_from_run_loop(&self, run_loop: NSObject, run_loop_mode: NSString) -> c_void {
        unsafe { msg_send![self, removeFromRunLoop: run_loop forMode: run_loop_mode] }
    }
    pub fn property_for_key(&self, key: NSString) -> NSString {
        unsafe { msg_send![self, propertyForKey: key] }
    }
    pub fn set_property_for_key(&self, id: ShareId<NSObject>, key: NSString) -> bool {
        unsafe { msg_send![self, setProperty: id forKey: key] }
    }
    pub fn stream_status(&self) -> NSStreamStatus {
        unsafe { msg_send![self, streamStatus] }
    }
    pub fn stream_error(&self) -> NSError {
        unsafe { msg_send![self, streamError] }
    }
    pub fn read(&self, buffer: *const u8, max_length: NSUInteger) -> NSInteger {
        unsafe { msg_send![self, read: buffer maxLength: max_length] }
    }
    pub fn get_buffer(&self, buffer: &[u8], length: &NSInteger) -> bool {
        unsafe { msg_send![self, getBuffer: buffer length: length] }
    }
    pub fn has_bytes_available(&self) -> bool {
        unsafe { msg_send![self, hasBytesAvailable] }
    }
}

impl NSOutputStream {
    pub fn open(&self) -> c_void {
        unsafe { msg_send![self, open] }
    }
    pub fn close(&self) -> c_void {
        unsafe { msg_send![self, close] }
    }
    pub fn delegate(&self, id: ShareId<NSObject>) -> c_void {
        unsafe { msg_send![self, delegate: id] }
    }
    pub fn schedule_in_run_loop(&self, run_loop: NSObject, run_loop_mode: NSString) -> c_void {
        unsafe { msg_send![self, scheduleInRunLoop: run_loop forMode: run_loop_mode] }
    }
    pub fn remove_from_run_loop(&self, run_loop: NSObject, run_loop_mode: NSString) -> c_void {
        unsafe { msg_send![self, removeFromRunLoop: run_loop forMode: run_loop_mode] }
    }
    pub fn property_for_key(&self, key: NSString) -> NSString {
        unsafe { msg_send![self, propertyForKey: key] }
    }
    pub fn set_property_for_key(&self, id: ShareId<NSObject>, key: NSString) -> bool {
        unsafe { msg_send![self, setProperty: id forKey: key] }
    }
    pub fn stream_status(&self) -> NSStreamStatus {
        unsafe { msg_send![self, streamStatus] }
    }
    pub fn stream_error(&self) -> NSError {
        unsafe { msg_send![self, streamError] }
    }
    pub fn write(&self, buffer: &[u8], max_length: NSUInteger) -> NSInteger {
        unsafe { msg_send![self, write: buffer maxLength: max_length] }
    }
    pub fn has_space_available(&self) -> bool {
        unsafe { msg_send![self, hasSpaceAvailable] }
    }
}

impl CBPeer {
    pub fn identifier(&self) -> NSUUID {
        unsafe { msg_send![self, identifier] }
    }
}

impl CBL2CAPChannel {
    pub fn input_stream(&self) -> &NSInputStream {
        unsafe { msg_send![self, inputStream] }
    }
    pub fn output_stream(&self) -> &NSOutputStream {
        unsafe { msg_send![self, outputStream] }
    }
    pub fn peer(&self) -> CBPeer {
        unsafe { msg_send![self, peer] }
    }
    pub fn psm(&self) -> u16 {
        unsafe { msg_send![self, PSM] }
    }
}

/*#[derive(Debug, Clone)]
pub enum NSStreamEvent {
    Connected,
}*/
