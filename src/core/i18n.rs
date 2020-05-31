use std::pin::Pin;
// use std::rc::Rc;
use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use std::task::{Context, Poll};
use futures::future::{ok, Ready};
use futures::Future;

use serde_json::Value as Json;
use json::JsonValue;
use std::collections::HashMap;

use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};

// use crate::core::app::{AppToolkit};


#[derive(Clone, Debug)]
pub struct I18N {
    pub locales: Json,
    pub current_lang: String,
    pub translations: HashMap<String, JsonValue>
}

impl I18N {

    pub fn configure(config: Json) -> I18N {

        let locales = config["i18n"]["locales"].clone();
        let current_lang = config["i18n"]["default_lang"].as_str().unwrap().to_string();
        let dir = config["i18n"]["directory"].as_str().unwrap();

        let mut translations = HashMap::new();
        let buffers: HashMap<String, String> = read_files(&locales, dir);
        for (lang, json) in buffers {
            let parsed_json = json::parse(json.as_str()).unwrap();
            translations.insert(lang, parsed_json);
        }

        I18N {
            locales: locales,
            current_lang: current_lang,
            translations: translations
        }
    }

    pub fn t(&self, key: &str) -> &JsonValue {
        match self.translations.get(self.current_lang.as_str()) {
            Some(language_json) => {
                if language_json.has_key(key) {
                    &language_json[key]
                } else {
                    panic!("Unable to find the key {}", key);
                }
            },
            None => panic!("Unable to get the language")
        }
    }

    pub fn set_locale_lang(&mut self, lang: &str) {
        for l in self.locales.as_array().unwrap() {
            if lang.contains(l.as_str().unwrap()) {
                self.current_lang = l.as_str().unwrap().to_string();
            }
        }
    }

}

fn read_files(locales: &Json, dir: &str) -> HashMap<String, String> {
    let mut buffers = HashMap::new();
    for filename in locales.as_array().unwrap() {
        let path = &format!("{}/{}.json", dir, filename.as_str().unwrap());
        let file = File::open(path).expect("Failed to open the file");
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader.read_to_string(&mut contents).expect("Failed to read the file");
        buffers.insert(filename.as_str().unwrap().to_string(), contents);
    }
    buffers
}



// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for I18N
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = I18nMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(I18nMiddleware { 
            service,
            // inner: self.clone(),
        })
    }
}


pub struct I18nMiddleware<S> {
    service: S,
    // inner: I18N
}

impl<S, B> Service for I18nMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        // println!("Hi from start. You requested: {}", req.path());

        // let mut appkit = req.app_data::<AppToolkit>().unwrap();
        // println!("{:?}", appkit.locale);
        // appkit.locale.current_lang = String::from("zh-CN");

        // 设置本地语言 todo 
        // self.inner.set_locale_lang(req.headers().get("accept-language").unwrap().to_str().unwrap());
        //appkit.locale = self.inner.into_owned();
        
        // println!("local = {}", self.inner.t("introduction"));

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            //println!("Hi from response");
            Ok(res)
        })
    }
}

