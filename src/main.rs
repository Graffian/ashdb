

use std::{collections::HashMap,io::Write};

use chrono::Utc;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, sync::mpsc};




#[tokio::main]
async fn main(){
    let (tx , mut rx) = mpsc::channel::<String>(32);
    let mut user_input = String::new();
    let mut map = HashMap::new();
    let now = Utc::now();
    let ms = now.timestamp_millis();

    tokio::spawn(async move {
        let mut file = tokio::fs::OpenOptions::new()
                                                .create(true)
                                                .write(true)
                                                .create(true)
                                                .append(true)
                                                .open("ashdb.log").await.unwrap();
        while let Some(line)  = rx.recv().await {
            file.write_all(line.as_bytes()).await.unwrap();
        }
    });
//12 67
    loop{
        if map.is_empty(){
            let mut file = tokio::fs::OpenOptions::new()
                                                        .create(true)
                                                        .append(true)
                                                        .read(true)
                                                        .write(true)
                                                        .open("ashdb.log").await.unwrap();
            let mut buf_read_kv = String::new();
            let _read_kv = file.read_to_string(&mut buf_read_kv).await.unwrap();
            
            let kv : Vec<&str> = buf_read_kv.split_whitespace().collect();

            let mut i = 0;
            while i+4 <= kv.len(){
                if kv[i] == "SET" && kv[i+3] == "ex"{
                    map.insert(kv[i+1].to_string() , kv[i+2].to_string());
                    match kv[i+4].parse::<i64>(){
                        Ok(n) => {
                            if n < ms{
                                map.remove(kv[i+1]);
                            }
                        }
                        Err(_)=>{}
                    }
                }
                i+=5;
            }
            
        }
        user_input.clear();
        print!(" >>>>>>>>>>  ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut user_input).expect("Failed to read line");
        let arg_vec:Vec<&str> = user_input.split_whitespace().collect();

        if arg_vec[0] == "QUIT"{
            break
        }else if arg_vec[0] == "SET" && arg_vec.len() > 2{
            map.insert(arg_vec[1].to_string(), arg_vec[2].to_string());
            if arg_vec.len() == 5{
                let time_limit:i64 = arg_vec[4].parse().expect("not a valid integer");
                let time_stamp = (time_limit*1000) + ms;
                let _ = tx.send(format!("SET {} {} ex {} \n" , arg_vec[1].to_string() , arg_vec[2].to_string() , time_stamp)).await;
            }else if arg_vec.len() == 3{
                let _ = tx.send(format!("SET {} {} ex - \n" , arg_vec[1].to_string() , arg_vec[2].to_string())).await;
            }
        }else if arg_vec[0] == "SHOW" && arg_vec.len() == 1{
            compaction(&mut map).await;
            println!("{:#?}" , map);
        }else if arg_vec[0] == "GET" && arg_vec.len() == 2{
            compaction(&mut map).await;
            if map.contains_key(arg_vec[1]){
                println!("VALUE: {:#?}" , map.get(arg_vec[1]).unwrap());
            }else{
                println!("value for the key expired or was not made use SHOW command to see availaible keys")
            }
        }else if arg_vec[0] == "REMOVE" {
            map.remove_entry(arg_vec[1]);
        }
    }
}


async fn compaction(map: &mut HashMap<String , String>){
    let now = Utc::now();
    let ms = now.timestamp_millis();
    let mut file = tokio::fs::OpenOptions::new()
                                                        .create(true)
                                                        .append(true)
                                                        .read(true)
                                                        .write(true)
                                                        .open("ashdb.log").await.unwrap();
    let mut buf_read_kv = String::new();
    let _read_kv = file.read_to_string(&mut buf_read_kv).await.unwrap();
            
    let kv : Vec<&str> = buf_read_kv.split_whitespace().collect();

    let mut i = 0;
    while i+4 <= kv.len(){
        if kv[i] == "SET" && kv[i+3] == "ex"{
            map.insert(kv[i+1].to_string() , kv[i+2].to_string());
            match kv[i+4].parse::<i64>(){
                Ok(n) => {
                    if n < ms{
                        map.remove(kv[i+1]);
                    }
                }
                Err(_)=>{}
            }
        }
        i+=5;
    }
}


