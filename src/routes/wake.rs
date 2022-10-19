use rocket::State;
// Path: src/routes/wake.rs
use wake_on_lan;
use crate::{util::net::parse_mac_address, App};

#[post("/<name_or_id>")]
pub fn device(app:&State<App>,name_or_id: &str) -> String {
	// Load the configuration file
	let config = &app.config;
	// Find the device in the configuration file
	let device = config.devices.iter().find(|device| device.name == name_or_id || device.id == name_or_id);
	// Check if the device was found
	if device.is_none() {
		return format!("Device {} not found", name_or_id);
	}
	// Convert the MAC address to a byte array
	let mac_string = &device.unwrap().mac;
	let mac = parse_mac_address(&mac_string).unwrap();
	// Create the magic packet
	let magic_packet = wake_on_lan::MagicPacket::new(&mac);
	// Wake the device
	let result = magic_packet.send();
	// Check if the device was woken
	if result.is_err() {
		return format!("Failed to wake device {}", name_or_id);
	}
	// Return the result
	format!("Sent WakeOnLan packet to device {}", device.unwrap().name)
}
