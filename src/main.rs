use clap::{Command, Arg};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Weather{
	latitude: f64,
    longitude: f64,
	elevation: f64,
    generationtime_ms: f64,
	daily_units: DailyUnits,
	daily: Daily
}

#[derive(Serialize, Deserialize, Debug)]
struct DailyUnits{
    weathercode: String,
    time: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Daily{
    weathercode: Vec<i32>,
    time: Vec<String>
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>{
    
    
    // オプション1(都市)
    let opt_city: Arg = Arg::new("city") //プログラムで取得するためのラベル
        .long("city") //コマンドで指定するための文字    
        .short('c') //コマンドで指定するための文字（省略形）
        .takes_value(true) //値を受け取るか
        .default_value("tokyo"); //デフォルト値

    // // オプション2
    // let opt2: Arg = Arg::new("opt2") 
    //     .long("opt2") 
    //     .short('o')
    // .required(true); //必須かどうか
    //     .default_value("Alice"); //デフォルト値

    // // オプション3
    // let positional_arg: Arg = Arg::new("text")
    //     .required(true);//long・shortを使用しない場合は位置で判別するらしい

    // コマンド名
    let command: Command = Command::new("tune-weather")
        .author("author:tunemage")
        .version("v1.0.0")
        .about("tunemage's first CLI Application written in Rust ")
        //オプションを渡す
        .arg(opt_city);

    // コマンドを実行する処理
    match command.try_get_matches() {
        Ok(m) => {
            //コマンドから都市名を取得
            let _city = m.value_of("city").unwrap();
            let (_latitude,_longitude) = get_latitude_and_longitude(_city);
            let target_url = "https://api.open-meteo.com/v1/forecast?latitude=".to_owned() + _latitude + "&longitude="+ _longitude +"&daily=weathercode&timezone=Asia%2FTokyo";
    
            // APIからデータを取得
            let mut res = surf::get(target_url).await?;
            let body_string = res.body_string().await?;
            //JSONをパース
            let weather:Weather = serde_json::from_str::<Weather>(&body_string)?;
            //直近分を出力
            for count in 0..5 {
                println!("{}: {}", weather.daily.time.get(count).unwrap(), get_weather(weather.daily.weathercode[count]));
            }

            Ok(())
        },
        Err(e) => {
            println!("{}", e);
            Ok(())
        }
    }
}

//WMO codeから記号を返す（コード体系を分かってないので間違ってる可能性大。その他は一律「分からん」で返す）
fn get_weather(code:i32)->  &'static str{
    if 1 <= code && code <= 3 {
        return "☀"
    }else if 60 <= code && code <= 69 {
        return "☂"
    }else if 70 <= code && code <= 79 {
        return "☃"
    }else{
        return "分からん"
    }
}

//都市名から、緯度経度を返す。東京・大阪・名古屋のみ対応
fn get_latitude_and_longitude (city:&str)->  (&'static str , &'static str){
    if city == "tokyo" {
        return ("35.6785","139.6823");
    }else if city == "osaka" {
        return ("34.6723","135.4848");
    }else if city == "nagoya" {
        return ("35.1833","136.8999");
    }else{
        return ("35.6785","139.6823");
    }
}