use std::io::{self, Write, Read};
use std::time::Duration;
use serialport::{SerialPort, available_ports, SerialPortType};

fn print_ports() -> io::Result<()> {
    let ports = available_ports()?;
    if ports.is_empty() {
        println!("Нет доступных последовательных портов.");
    } else {
        println!("Доступные COM-порты:");
        for port in ports {
            match port.port_type {
                SerialPortType::UsbPort(_) => println!("{}", port.port_name),
                SerialPortType::PciPort | SerialPortType::BluetoothPort | SerialPortType::Unknown => {}
            }
        }
    }
    Ok(())
}

fn get_ports() -> io::Result<Vec<String>> {
    let ports = available_ports()?;
    let port_names: Vec<String> = ports
        .iter()
        .filter_map(|port| {
            if let SerialPortType::UsbPort(_) = port.port_type {
                Some(port.port_name.clone())
            } else {
                None
            }
        })
        .collect();
    Ok(port_names)
}

fn read_line_int() -> Result<i32, std::num::ParseIntError> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse()
}

fn main() -> io::Result<()> {
    loop {
        print_ports()?;

        // Получаем список доступных портов
        let available_ports = get_ports()?;

        // Запрос ввода номера порта от пользователя
        println!("Введите число:");
        let com_id: i32 = loop {
            match read_line_int() {
                Ok(num) => break num,
                Err(_) => println!("Ошибка: некорректный ввод. Попробуйте снова."),
            }
        };

        let port_name = format!("COM{}", com_id);
        if !available_ports.contains(&port_name) {
            println!("Ошибка: порт {} не найден.", port_name);
            continue;
        }

        // Открываем выбранный порт
        let mut port = serialport::new(port_name, 9600)
            .timeout(Duration::from_secs(5))
            .open()?;

        // Отправляем данные в порт
        let data_to_send = "10,10,10,10,10".as_bytes();;
        port.write(data_to_send).expect("Write failed!");

        let mut serial_buf: Vec<u8> = vec![0; 4000];
        match port.read(serial_buf.as_mut_slice()) {
            Ok(nbytes) if nbytes > 0 => {
                let received_data = String::from_utf8_lossy(&serial_buf[..nbytes]);
                println!("Received: {}", received_data);
            }
            Ok(_) => {
                println!("No data received.");
            }
            Err(e) => {
                eprintln!("Failed to read from port: {}", e);
            }
        }

        drop(port);
    }
}
