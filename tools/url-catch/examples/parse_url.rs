use error_chain::error_chain;
use url::{Host, Origin, ParseError, Position, Url};

// url crate 中的 parse 方法验证并解析 &str 切片为 Url 结构体。如果输入字符串
// 的格式不正确，解析方法 parse 会返回 Result<Url, ParseError>

error_chain! {
    foreign_links {
        UrlParse(ParseError);
    }
}

fn u1() -> Result<()> {
    let s = "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open";

    let parsed = Url::parse(s)?;
    println!("The path part of the URL is: {}", parsed.path());
    println!("{}", parsed.to_string());
    println!("{}", parsed.domain().unwrap_or(""));
    println!("{}", parsed.authority());
    println!("{}", parsed.host_str().unwrap_or(""));
    println!("{}", parsed.query().unwrap_or(""));

    Ok(())
}

fn u2() -> Result<()> {
    let s = "ftp://rust-lang.org/examples";

    let url = Url::parse(s)?;

    assert_eq!(url.scheme(), "ftp");
    assert_eq!(url.host(), Some(Host::Domain("rust-lang.org")));
    assert_eq!(url.port_or_known_default(), Some(21));
    println!("The origin is as expected!");

    Ok(())
}

fn u3() -> Result<()> {
    let s = "ftp://rust-lang.org/examples";

    let url = Url::parse(s)?;

    let expected_scheme = "ftp".to_owned();
    let expected_host = Host::Domain("rust-lang.org".to_owned());
    let expected_port = 21;
    let expected = Origin::Tuple(expected_scheme, expected_host, expected_port);

    let origin = url.origin();
    assert_eq!(origin, expected);
    println!("The origin is as expected!");

    Ok(())
}

fn u4() -> Result<()> {
    let parsed = Url::parse("https://github.com/rust-lang/rust/issues?labels=E-easy&state=open")?;
    let cleaned: &str = &parsed[..Position::AfterPath];
    println!("cleaned: {}", cleaned);
    Ok(())
}

fn main() {
    // let _ = u1();
    // let _ = u2();
    // let _ = u3();
    let _ = u4();
}
