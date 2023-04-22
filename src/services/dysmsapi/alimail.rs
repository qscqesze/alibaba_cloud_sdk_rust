#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use super::Client;
use gostd::strings;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Error;
impl Client {
    pub fn CreateDomain(&mut self, request: &mut CreateDomainRequest)
        -> Result<CreateDomainResponse, Error> {
        let mut response = CreateDomainResponse();
        request.BuildQueryParams();
        let mut baseResponse = responses::BaseResponse::default();
        self.DoAction(&mut request.rpcRequest, &mut baseResponse)?;
        response = serde_json::from_slice(&baseResponse.httpContentBytes)?;
        Ok(response)
    }
    pub fn SingleSendMail(&mut self, request: &mut SingleSendMailRequest)
    -> Result<SingleSendMailResponse, Error> {
        let mut response = SingleSendMailResponse();
        request.BuildQueryParams();
        let mut baseResponse = responses::BaseResponse::default();
        self.DoAction(&mut request.rpcRequest, &mut baseResponse)?;
        response = serde_json::from_slice(&baseResponse.httpContentBytes)?;
        Ok(response)
    }
}

use crate::sdk::requests::BaseRequestExt;
use crate::sdk::requests::{self, BaseRequest};
use crate::sdk::responses;
#[derive(Default, Debug)]
pub struct CreateDomainRequest {
    rpcRequest: requests::RpcRequest,
    pub DomainName: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CreateDomainResponse {
    // baseResponse: responses::BaseResponse,
    pub DomainId: String,
    pub RequestId: String,
}

impl BaseRequestExt for CreateDomainRequest {
    fn base(&self) -> &BaseRequest {
        self.rpcRequest.base()
    }

    fn base_as_mut(&mut self) -> &mut BaseRequest {
        self.rpcRequest.base_as_mut()
    }
}

impl CreateDomainRequest {
    pub fn BuildQueryParams(&mut self) {
        self.addQueryParam("DomainName", &self.DomainName.to_owned());
    }
}

pub fn CreateDomainRequest() -> CreateDomainRequest {
    let mut request = CreateDomainRequest::default();
    request
        .rpcRequest
        .InitWithApiInfo("Dm", "2015-11-23", "CreateDomain", "", "");
    request.SetMethod(requests::POST);
    request
}

pub fn CreateDomainResponse() -> CreateDomainResponse {
    CreateDomainResponse::default()
}

///////////////////////// send mail
#[derive(Default, Debug)]
pub struct SingleSendMailRequest {
    rpcRequest: requests::RpcRequest,
    pub AccountName: String,
    pub ToAddress: String,
    pub Subject: String,
    pub TextBody: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SingleSendMailResponse {
    // baseResponse: responses::BaseResponse,
    pub EnvId: String,
    pub RequestId: String,
}

impl BaseRequestExt for SingleSendMailRequest {
    fn base(&self) -> &BaseRequest {
        self.rpcRequest.base()
    }

    fn base_as_mut(&mut self) -> &mut BaseRequest {
        self.rpcRequest.base_as_mut()
    }
}

impl SingleSendMailRequest {
    pub fn BuildQueryParams(&mut self) {
        self.addQueryParam("AccountName", &self.AccountName.to_owned());
        self.addQueryParam("AddressType", &"1");
        self.addQueryParam("ReplyToAddress", &"true");
        self.addQueryParam("ToAddress", &self.ToAddress.to_owned());
        self.addQueryParam("Subject", &self.Subject.to_owned());
        self.addQueryParam("TextBody", &self.TextBody.to_owned());
    }
}

pub fn SingleSendMailRequest() -> SingleSendMailRequest {
    let mut request = SingleSendMailRequest::default();
    request
        .rpcRequest
        .InitWithApiInfo("Dm", "2015-11-23", "SingleSendMail", "", "");
    request.SetMethod(requests::POST);
    request
}

pub fn SingleSendMailResponse() -> SingleSendMailResponse {
    SingleSendMailResponse::default()
}


