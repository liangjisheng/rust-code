use serde::{Deserialize, Serialize};
use std::fs::{read, File};
use std::io::{self, Write};

fn read_csv_string() -> Result<(), csv::Error> {
    let csv = "year,make,model,description
1948,Porsche,356,Luxury sports car
1967,Ford,Mustang fastback 1967,American car";

    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    for record in reader.records() {
        let record = record?;
        println!(
            "In {}, {} built the {} model. It is a {}.",
            &record[0], &record[1], &record[2], &record[3]
        );
    }

    Ok(())
}

#[derive(Deserialize)]
struct Record {
    year: u16,
    make: String,
    model: String,
    description: String,
}

fn read_csv_string1() {
    let csv = "year,make,model,description
1948,Porsche,356,Luxury sports car
1967,Ford,Mustang fastback 1967,American car";

    let mut reader = csv::Reader::from_reader(csv.as_bytes());

    for record in reader.deserialize() {
        let r: Record = match record {
            Err(e) => {
                println!("err {}", e);
                continue;
            }
            Ok(v) => v,
        };
        println!(
            "In {}, {} built the {} model. It is a {}.",
            r.year, r.make, r.model, r.description
        );
    }
}

#[derive(Deserialize)]
struct Address {
    address: String,
    name: String,
}

fn csv_file() {
    let src_csv_file = match File::open("address.csv") {
        Ok(f) => f,
        Err(e) => panic!("err {}", e),
    };

    let mut dst_csv_file = File::create("address_copy.csv").expect("create file fail");
    let mut reader = csv::Reader::from_reader(src_csv_file);
    let mut writer = csv::Writer::from_writer(dst_csv_file);

    let header = reader.headers().unwrap();
    let _ = writer.write_record(header);

    for record in reader.records() {
        let r = record.unwrap();
        let _ = writer.write_record(&r);
    }

    for record in reader.deserialize::<Address>() {
        let item = match record {
            Err(e) => {
                println!("err {}", e);
                continue;
            }
            Ok(v) => v,
        };

        println!("address {}, name {}", item.address, item.name);
    }
}

// 读取具有不同分隔符的 csv 记录, 用一个 tab(分隔符) delimiter 读取 csv 记录
// csv 提供一个自定义反序列化程序，csv::invalid_option，自动将无效数据转换为 None 值
#[derive(Debug, Deserialize)]
struct Record1 {
    name: String,
    place: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    id: Option<u64>,
}

fn different_delimiter() {
    let data = "name\tplace\tid
Mark\tMelbourne\t46
Ashley\tZurich\t92
Ashley\tZurich\ta";

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(data.as_bytes());

    for result in reader.deserialize::<Record1>() {
        let item = match result {
            Err(e) => {
                println!("err {}", e);
                continue;
            }
            Ok(v) => v,
        };

        if let Some(v) = item.id {
            println!("name {}, place {}, id {}", item.name, item.place, v);
        }
    }
}

// 筛选与断言匹配的 csv 记录, 只返回data中字段(field)行，匹配query的
fn filter() -> Result<(), io::Error> {
    let query = "CA";
    let data = "\
City,State,Population,Latitude,Longitude
Kenai,AK,7610,60.5544444,-151.2583333
Oakman,AL,,33.7133333,-87.3886111
Sandfort,AL,,32.3380556,-85.2233333
West Hollywood,CA,37031,34.0900000,-118.3608333";

    let mut rdr = csv::ReaderBuilder::new().from_reader(data.as_bytes());
    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.write_record(rdr.headers()?)?;

    for result in rdr.records() {
        let record = result?;
        if record.iter().any(|field| field == query) {
            wtr.write_record(&record)?;
        }
    }

    wtr.flush()?;
    Ok(())
}

// csv::writer 支持从 Rust 类型到 CSV 记录的自动序列化。write_record
// 只写入包含字符串数据的简单记录。具有更复杂值（如数字、浮点和选项）的数据使用
// serialize 进行序列化。因为 csv::writer 使用内部缓冲区，所以在完成时总是显式刷新 flush
fn write_csv_file() -> Result<(), csv::Error> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.write_record(&["Name", "Place", "ID"])?;

    wtr.serialize(("Mark", "Sydney", 87))?;
    wtr.serialize(("Ashley", "Dublin", 32))?;
    wtr.serialize(("Akshat", "Delhi", 11))?;

    wtr.flush()?;
    Ok(())
}

#[derive(Serialize)]
struct Record2<'a> {
    name: &'a str,
    place: &'a str,
    id: u64,
}

// 用 Serde 将记录序列化为 CSV
fn write_struct() -> Result<(), csv::Error> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    let rec1 = Record2 {
        name: "Mark",
        place: "Melbourne",
        id: 56,
    };
    let rec2 = Record2 {
        name: "Ashley",
        place: "Sydney",
        id: 64,
    };
    let rec3 = Record2 {
        name: "Akshat",
        place: "Delhi",
        id: 98,
    };

    wtr.serialize(rec1)?;
    wtr.serialize(rec2)?;
    wtr.serialize(rec3)?;

    wtr.flush()?;

    Ok(())
}

fn main() {
    // let _ = read_csv_string();
    // read_csv_string1();
    // csv_file();
    // different_delimiter();
    // let _ = filter();
    // let _ = write_csv_file();
    let _ = write_struct();
}
