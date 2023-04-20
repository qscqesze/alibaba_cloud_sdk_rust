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