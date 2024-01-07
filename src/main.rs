use blurz::{
    BluetoothAdapter, BluetoothDevice, BluetoothGATTCharacteristic, BluetoothGATTService,
    BluetoothSession,
};
use std::error::Error;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let session = BluetoothSession::create_session(None).unwrap();
    let adapter = BluetoothAdapter::init(&session).unwrap();
    let device_name = "Czar's Kyria";
    let batteries = HashMap::from([(1, "Right"), (2, "Left")]);
    let device = adapter
        .get_device_list()
        .unwrap()
        .into_iter()
        .find(|d| {
            BluetoothDevice::new(&session, d.clone())
                .get_name()
                .map_or(false, |name| name == device_name)
        })
        .map(|d| BluetoothDevice::new(&session, d))
        .unwrap();
    let gatt_services = device.get_gatt_services().unwrap();
    let mut count = 0;
    for service_path in gatt_services {
        let service = BluetoothGATTService::new(&session, service_path.to_string());
        if service.get_uuid().unwrap().starts_with("0000180f") {
            let characteristics = service.get_gatt_characteristics().unwrap();
            let char_path = &characteristics[0];
            let characteristic = BluetoothGATTCharacteristic::new(&session, char_path.to_string());
            if let Ok(characteristic) = characteristic.read_value(None) {
                count += 1;
                println!("Battery {}: {}%", batteries[&count], characteristic[0]);
            }
        }
    }
    Ok(())
}
