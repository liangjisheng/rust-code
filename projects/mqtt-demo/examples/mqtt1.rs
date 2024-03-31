use paho_mqtt as mqtt;
use std::time::Duration;

// cargo run --example mqtt1

#[tokio::main]
async fn main() {
    // 创建MQTT客户端
    let cli = mqtt::Client::new("tcp://localhost:1883").unwrap();

    // 设置LWT特征
    let lwt = mqtt::Message::new("lwt", "offline", 0);
    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .will_message(lwt)
        .clean_session(false)
        .connect_timeout(Duration::from_secs(5))
        .finalize();

    // 连接MQTT服务器
    let tok = cli.connect(conn_opts);
    tok.await.unwrap();

    // 订阅主题
    let topic = "hello/world";
    let qos = 1;
    let tok = cli.subscribe(topic, qos);
    tok.await.unwrap();

    // 发送消息
    let payload = "hello, world!";
    let msg = mqtt::Message::new(topic, payload, qos);
    let tok = cli.publish(msg);
    tok.await.unwrap();

    // 接收消息
    for _ in 0..10 {
        let msg = cli.await_message(Duration::from_secs(1)).unwrap();
        println!("Received message: {:?}", msg);
    }

    // 断开连接
    let tok = cli.disconnect(None);
    tok.await.unwrap();
}
