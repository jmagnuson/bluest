[![crates.io][crates-badge]][crates-url] [![docs.rs][docs-badge]][docs-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/bluest
[crates-url]: https://crates.io/crates/bluest
[docs-badge]: https://docs.rs/bluest/badge.svg
[docs-url]: https://docs.rs/bluest
[actions-badge]: https://github.com/alexmoon/bluest/workflows/CI/badge.svg
[actions-url]: https://github.com/alexmoon/bluest/actions?query=workflow%3ACI+branch%3Amain

# Bluest — Cross-platform Bluetooth LE crate for Rust

<!-- cargo-rdme start -->

Bluest is a cross-platform [Bluetooth Low Energy] (BLE) library for [Rust]. It currently supports Windows (version
10 and later), MacOS/iOS, and Linux. Android support is planned.

The goal of Bluest is to create a *thin* abstraction on top of the platform-specific Bluetooth APIs in order to
provide safe, cross-platform access to Bluetooth LE devices. The crate currently supports the GAP Central and
GATT Client roles. Peripheral and Server roles are not supported.

[Rust]: https://www.rust-lang.org/
[Bluetooth Low Energy]: https://www.bluetooth.com/specifications/specs/

## Usage

```rust
let adapter = Adapter::default().await.ok_or("Bluetooth adapter not found")?;
adapter.wait_available().await?;

println!("starting scan");
let mut scan = adapter.scan(&[]).await?;
println!("scan started");
while let Some(discovered_device) = scan.next().await {
   println!(
       "{}{}: {:?}",
       discovered_device.device.name().as_deref().unwrap_or("(unknown)"),
       discovered_device
           .rssi
           .map(|x| format!(" ({}dBm)", x))
           .unwrap_or_default(),
       discovered_device.adv_data.services
   );
}
```

## Overview

The primary functions provided by Bluest are:

- Device discovery:
  - [Scanning][Adapter::scan] for devices and receiving advertisements
  - Finding [connected devices][Adapter::connected_devices]
  - [Opening][Adapter::open_device] previously found devices
  - [Connecting][Adapter::connect_device] to discovered devices
- Accessing remote GATT services:
  - Discovering device [services][Device::discover_services]
  - Discovering service [characteristics][Service::discover_characteristics]
  - Discovering characteristic [descriptors][Characteristic::discover_descriptors]
  - [Read][Characteristic::read], [write][Characteristic::write] (including
    [write without response][Characteristic::write_without_response]), and
    [notify/indicate][Characteristic::notify] operations on remote characteristics
  - [Read][Descriptor::read] and [write][Descriptor::write] operations on characteristic descriptors

## Platform specifics

Because Bluest aims to provide a thin abstraction over the platform-specific APIs, the available APIs represent the
lowest common denominator of APIs among the supported platforms. In most cases Apple's CoreBluetooth API is the
most restricted and therefore impose the limit on what can be supported in a cross platform library. For example,
CoreBluetooth never exposes the Bluetooth address of devices to applications, therefore there is no method on
`Device` for retrieving an address or even any Bluetooth address struct in the crate.

The underlying APIs for accessing services, characteristics, and descriptors are all pretty similar and should
behave consistently across platforms. However errors may not be consistent from platform to platform. For example,
Linux's bluez API does not return the underlying Bluetooth protocol error in a useful way, whereas the other
platforms do. Where it is possible to return a meaningful error, Bluest will attempt to do so. In other cases,
Bluest may return an error with a [`kind`][Error::kind] of [`Other`][error::ErrorKind::Other] and you would need to
look at the platform-specific [`source`][std::error::Error::source] of the error for more information.

The more significant area of platform differences is in discovering and connecting to devices.

Each platform has its own methods for identifying, scanning for, and connecting to devices. Again, since
CoreBluetooth is the most restrictive in the methods it provides for filtering scan results and identifying devices
to connect to, the Bluest API largely follows those limitations (e.g. scanning can be filtered only by a set of
service UUIDs). The implementations for other platforms have been poly-filled to match those APIs.

Connecting and disconnecting from devices is another area of API differences that cannot be as easily poly-filled.
To ensure proper cross-platform behavior, you should always call [`connect_device`][Adapter::connect_device] before
calling any methods which may require a connection. When you have finished using a device you should call
[`disconnect_device`][Adapter::disconnect_device] and then drop the `Device` and all its child objects to ensure
the OS will properly release any associated resources.

### MacOS/iOS (CoreBluetooth)

Connections to devices are managed by the `Adapter` instance. You must call
[`connect_device`][Adapter::connect_device] before calling any methods on a `Device` (or child objects)
that require a connection or an error will be returned. Because the `Adapter` manages the connections, all
connections will be closed when the `Adapter` is dropped, therefore you must ensure the `Adapter` lives as long as
you need a connection to any of its devices.

When you call [`disconnect_device`][Adapter::disconnect_device], access to that device will be terminated
for the application. If no other applications running on the system have connected to the device, the underlying
hardware connection will be closed.

### Windows (WinRT)

Connections to devices are managed automatically by the OS. Calls to [`connect_device`][Adapter::connect_device]
and [`disconnect_device`][Adapter::disconnect_device] are no-ops that immediately return success. The actual
connection will be made as soon as a method is called on a `Device` that requires a connection (typically
[`discover_services`][Device::discover_services]). That connection will be maintained as long as the `Device`
instance or any child instance lives.

## Feature flags

The `serde` feature is available to enable serializing/deserializing device
identifiers.

## Examples

Examples demonstrating basic usage are available in the [examples folder].

[examples folder]: https://github.com/alexmoon/bluest/tree/master/bluest/examples

<!-- cargo-rdme end -->

Refer to the [API documentation] for more details.

[API documentation]: https://docs.rs/bluest
[Adapter::scan]: https://docs.rs/bluest/latest/bluest/struct.Adapter.html#method.scan
[Adapter::connected_devices]: https://docs.rs/bluest/latest/bluest/struct.Adapter.html#method.connected_devices
[Adapter::open_device]: https://docs.rs/bluest/latest/bluest/struct.Adapter.html#method.open_device
[Adapter::connect_device]: https://docs.rs/bluest/latest/bluest/struct.Adapter.html#method.connect_device
[Adapter::disconnect_device]: https://docs.rs/bluest/latest/bluest/struct.Adapter.html#method.disconnect_device
[Device::discover_services]: https://docs.rs/bluest/latest/bluest/struct.Device.html#method.discover_services
[Service::discover_characteristics]: https://docs.rs/bluest/latest/bluest/struct.Service.html#method.discover_characteristics
[Characteristic::discover_descriptors]: https://docs.rs/bluest/latest/bluest/struct.Characteristic.html#method.discover_descriptors
[Characteristic::read]: https://docs.rs/bluest/latest/bluest/struct.Characteristic.html#method.read
[Characteristic::write]: https://docs.rs/bluest/latest/bluest/struct.Characteristic.html#method.write
[Characteristic::write_without_response]: https://docs.rs/bluest/latest/bluest/struct.Characteristic.html#method.write_without_response
[Characteristic::notify]: https://docs.rs/bluest/latest/bluest/struct.Characteristic.html#method.notify
[Descriptor::read]: https://docs.rs/bluest/latest/bluest/struct.Descriptor.html#method.read
[Descriptor::write]: https://docs.rs/bluest/latest/bluest/struct.Descriptor.html#method.write
[Error::kind]: https://docs.rs/bluest/latest/bluest/error/struct.Error.html#method.kind
[error::ErrorKind::Other]: https://docs.rs/bluest/latest/bluest/error/enum.ErrorKind.html#variant.Other
[std::error::Error::source]: https://doc.rust-lang.org/stable/std/error/trait.Error.html#method.source
