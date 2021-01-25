mod cocktailmaker;
use cocktailmaker::CocktailMachine;
mod lcd;
use lcd::Lcd;

use isahc::prelude::*;
use serde_json:: Value;
use azure_iot_sdk::{IoTHubClient, DeviceKeyTokenSource, MessageType, Message, DirectMethodResponse};
use std::str;

#[tokio::main]
async fn main() -> azure_iot_sdk::Result<()> 
{
    let name = isahc::get("https://cocktailmakerfunction.azurewebsites.net/api/machine/0/name").expect("request failed").text().expect("no text found");
    let mut machine = CocktailMachine::new(26, 19, 13, 6, 5, 21, 20, 16, 24, true, 12, false, 18, 23);
    let mut l = Lcd::new(1, 27, 17);
    l.write_scroll("name:", &name);

    let iothub_hostname = "";
    let device_id = "";
    let key = "";
    let token_source = DeviceKeyTokenSource::new(iothub_hostname, device_id, key).unwrap();
    let mut send_client = IoTHubClient::new(iothub_hostname, device_id.into(), token_source).await?;
    println!("Client aangemaakt");

    let mut recv_client = send_client.clone();
    let mut recv = recv_client.get_receiver().await;
    let receive_loop = async 
    {
        while let Some(msg) = recv.recv().await 
        {
            match msg 
            {
                MessageType::C2DMessage(msg) => 
                {
                    let vec = msg.body;
                    let s = str::from_utf8(&vec).unwrap();
                    println!("Received message: {:?}", s);
                },
                MessageType::DesiredPropertyUpdate(msg) => println!("Received property update: {:?}", msg),
                MessageType::DirectMethod(msg) => 
                {
                    let method = msg.method_name;
                    let vec = &msg.message.body;
                    let s = str::from_utf8(&vec).unwrap();
                    let json: Value = serde_json::from_str(s).expect("could not parse");
                    if method == String::from("make")
                    {
                        println!("making cocktail");
                        machine.make_cocktail(json["code"].as_str().unwrap());
                    }
                    else if method == String::from("name")
                    {
                        println!("updating screen");
                        l.write_scroll("name:", json["name"].as_str().unwrap());
                    }
                    recv_client
                        .respond_to_direct_method(DirectMethodResponse::new(
                            msg.request_id,
                            0,
                            Some(std::str::from_utf8(&msg.message.body).unwrap().to_string()),
                        ))
                        .await
                        .unwrap();
                },
                MessageType::ErrorReceive(er) => eprintln!("Received error: {:?}", er)
            }
        }
    };

    let msg = Message::new(b"awake".to_vec());
    let sender = send_client.send_message(msg);
    futures::join!(receive_loop, sender);
    Ok(())
}