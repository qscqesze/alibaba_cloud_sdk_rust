pub mod sdk;
pub mod services;
/*
use services::dysmsapi;
use gostd::strings;

const AliyunSmsServerRegion: &str = "cn-hangzhou";
// 这个key已经废弃了，不用试了。
const AliyunSmsAccessKeyID: &str = "LTAI5tGb3d9kqwfFt16AE9uc";
const AliyunSmsAccessKeySecret: &str = "CA3yGxmpJES40RpvToJP2q2PCBN4Kx"; 
const AliyunSmsReportTempleateCode: &str = "SMS_154950909"; // 通知模版
const AliyunSmsSignName: &str = "阿里云短信测试"; // 短信署名
const AliyunSmsTemplateParam: &str = "{\"code\":\"1234\"}";

fn main() {
    let phoneNumber="13408500362";
    let res = SendSMS(phoneNumber);
    println!("res: {:?}", res);
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
