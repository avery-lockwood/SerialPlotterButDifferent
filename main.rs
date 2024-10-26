use std::time::Duration;
use serialport;

use serde_json::{Result, Value};
use chrono::{DateTime,Utc};


pub struct SensorData<'a>
{
    sensortype: &'a str,
    sensorvalue: f32,
    time: DateTime<Utc>
}
//#[derive(Debug)]
impl<'a> SensorData<'a>
{
    pub fn new() -> SensorData<'a>
    {
        SensorData{sensortype: "", sensorvalue: 0.0, time: Utc::now()}
    }

}

fn main() -> Result<()> 
{
 
    let port_name = "COM19";
    let baud_rate = 9600;
    // Open the serial port
    let mut port = serialport::new(port_name, baud_rate)
    .timeout(Duration::from_millis(10))
    .open().expect("Failed to open port");

    // Buffer to store read data
    let mut buffer: Vec<u8> = vec![0; 128];
    let mut i = 0;
    loop 
    {
        match port.read(&mut buffer) 
        {
            Ok(bytes_read) => 
            {
                
                if bytes_read > 0 
                {
                    i +=1;
                    let data = &buffer[..bytes_read];
                  
                    let string_data = String::from_utf8_lossy(data);
                    println!("Read {} bytes: {:?}", bytes_read, string_data.trim());
                    //let sensordata: SensorData = serde_json::from_str(&string_data)?;
                    if i >= 10
                    {
                        //println!("Read {} bytes: {:?}", bytes_read, string_data.trim());
                        let v: Value = serde_json::from_str(&string_data.trim())?;  
                        println!("{}",v); 
                        println!("{}", Utc::now());
                         let mut sensordata = SensorData::new();
                        sensordata.sensortype = v.get("sensorType").cloned().unwrap().as_str().unwrap();
                        
                    }
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => 
            {
            // Handle timeout (no data received within the specified timeout duration)
               // println!("Timeout");
            }
            Err(e) => 
            {
                // Handle other read errors
                eprintln!("Error reading from serial port: {}", e);
                break;
            }
        }
    }

    Ok(())
}

