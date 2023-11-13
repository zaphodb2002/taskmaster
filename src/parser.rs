use serde_json::{Result, Value};
use time::{PrimitiveDateTime, Time, Date, Month};
use substring::Substring;
use crate::Task;

pub fn parse(data :&str) -> Option<Task> {
    if data == "null" || data.substring(0,1) != "{" { return None; }
    let v :Value = parse_raw(&data.trim()).unwrap_or_else(|_| panic!("bad task format :{}",&data));
    let due_parsed = parse_datetime(v["due"].to_string());
    let entry_parsed = parse_datetime(v["entry"].to_string());
    let modified_parsed = parse_datetime(v["modified"].to_string());
    let wait_parsed = parse_datetime(v["wait"].to_string());
    let uuid_parsed = parse_uuid(&v["uuid"].to_string());
    let project_parsed = parse_project(&v["project"].to_string());
    let tags_parsed = parse_tags(&v["tags"].to_string());

    let result = Task {
        description: v["description"].to_string(),
        due: due_parsed,
        entry: entry_parsed,
        mask: v["mask"].to_string(),
        modified: modified_parsed,
        project: project_parsed,
        recur: v["recur"].to_string(),
        rtype: v["periodic"].to_string(),
        status: v["status"].to_string(),
        uuid: uuid_parsed,
        wait: wait_parsed,
        tags: tags_parsed
    };
    Some(result)
}

fn parse_datetime(mut data :String) -> Option<PrimitiveDateTime>{
    data = data.replace("\"","");
    if data == "null" {return None;}
    let year :i32 = data.substring(0,4).parse().unwrap_or_else(|_| panic!("invalid year {}",data));
    let month_number :u8 = data.substring(4,6).parse().unwrap_or_else(|_| panic!("invalid month number in value: {}", data.substring(4,6)));
    let month = match month_number {
        1 => Month::January,
        2 => Month::February,
        3 => Month::March,
        4 => Month::April,
        5 => Month::May,
        6 => Month::June,
        7 => Month::July,
        8 => Month::August,
        9 => Month::September,
        10 => Month::October,
        11 => Month::November,
        12 => Month::December,
        _ => panic!("invalid month number {}", month_number),
    };
    let day :u8= data.substring(6,8).parse().unwrap();
    let hour :u8 = data.substring(9,11).parse().unwrap();
    let min :u8 = data.substring(11,13).parse().unwrap();
    let sec :u8 = data.substring(13,15).parse().unwrap();
    let date = Date::from_calendar_date(year,month,day).unwrap();
    let time = Time::from_hms(hour, min, sec).unwrap();
    let parsed = PrimitiveDateTime::new(date, time);

    Some(parsed)
}

fn parse_tags(data :&str) -> Vec<String> {
    let mut result :Vec<String> = Vec::new();
    result.push(data.to_string());
    return result;
}

fn parse_raw(data :&str) -> Result<Value> {
    let v: Value = serde_json::from_str(data)?;
    Ok(v)
}

fn parse_uuid(data :&str) -> String {
    let mut data = data.to_lowercase();
    data = data.replace("\"", "");
    return data;
}

fn parse_project(data :&str) -> Vec<String> {
    let data = data.split(".");
    let mut result :Vec<String> = Vec::new();
    for str in data {
        let str = str.replace("\"", "");
        result.push(str.to_string())
    }
    result
}
