pub mod sdk;
pub mod services;
use services::dysmsapi;
use gostd::strings;

/*
const AliyunSmsServerRegion: &str = "cn-hangzhou";
// 这个key已经废弃了，不用试了。
const AliyunSmsAccessKeyID: &str = "xxxxxx";
const AliyunSmsAccessKeySecret: &str = "xxxxxx"; 
const AliyunSmsReportTempleateCode: &str = "SMS_154950909"; // 通知模版
const AliyunSmsSignName: &str = "阿里云短信测试"; // 短信署名
const AliyunSmsTemplateParam: &str = "{\"code\":\"1234\"}";

fn main() {
    let res = SingleSendMail();
    println!("res: {:?}", res);
}

fn SingleSendMail() -> Result<(), std::io::Error> {
    let mut client = dysmsapi::Client::NewClientWithAccessKey(
        AliyunSmsServerRegion,
        AliyunSmsAccessKeyID,
        AliyunSmsAccessKeySecret,
    )?;
    let mut request = dysmsapi::SingleSendMailRequest();
    request.AccountName = "asdasdasda@asdasda.com".to_string();
    request.ToAddress = "zzzzz@qq.com".to_string();
    request.Subject = "验证码".to_string();
    request.TextBody = "验证码为123456, 有效期10分钟。".to_string();
    println!("request: {:?}", &request);
    let response = client.SingleSendMail(&mut request);
    println!("response: {:?}", &response);
    Ok(())
}

fn CreateDomain(domain: &str) -> Result<(), std::io::Error> {
    let mut client = dysmsapi::Client::NewClientWithAccessKey(
        AliyunSmsServerRegion,
        AliyunSmsAccessKeyID,
        AliyunSmsAccessKeySecret,
    )?;
    let mut request = dysmsapi::CreateDomainRequest();
    request.DomainName = domain.to_owned();
    println!("request: {:?}", &request);
    let response = client.CreateDomain(&mut request);
    println!("response: {:?}", &response);
    Ok(())
}

fn SendSMS(phoneNumber: &str) -> Result<(), std::io::Error> {
    let mut client = dysmsapi::Client::NewClientWithAccessKey(
        AliyunSmsServerRegion,
        AliyunSmsAccessKeyID,
        AliyunSmsAccessKeySecret,
    )?;
    let mut request = dysmsapi::CreateSendSmsRequest();
    request.PhoneNumbers = strings::Replace(phoneNumber, "+86", "", -1);
    request.SignName = AliyunSmsSignName.to_owned();
    request.TemplateCode = AliyunSmsReportTempleateCode.to_owned();
    request.TemplateParam = AliyunSmsTemplateParam.to_owned();
    let response = client.SendSms(&mut request)?;
    println!("{:?}", &response);

    Ok(())
}
*/