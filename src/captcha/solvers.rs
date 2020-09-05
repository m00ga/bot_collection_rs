use super::{CapTypes, CapSolvable, RefCell, HashMap};

use reqwest::blocking;

use serde;

//extern crate serde;
//extern crate serde_derive;
//extern crate serde_json;

pub mod re_caps{
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    #[derive(Debug, serde::Deserialize)]
    struct ReCapRequest{
        url: String,
        post_params: HashMap<String, serde_json::Value>,
        get_params: HashMap<String, serde_json::Value>
    }

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct ReCapResp{
        status: i32,
        request: String
    }

    fn rude_solve(params: &mut ReCapRequest) -> Result<String,String>{
        let client = blocking::Client::new();
        let resp = client.get(
            &format!("{}/in.php", params.url))
                .query(&params.post_params)
                .timeout(Duration::from_secs(5))
                .send();
        if let Ok(res) = resp{
            let resp = res.text().unwrap();
            let mut resp: ReCapResp = serde_json::from_str(&resp).unwrap();
            if resp.status != 1{
                return Err(format!("ERROR: {}", resp.request));
            }
            let id = resp.request.parse().unwrap();
            params.get_params.entry("id".to_string())
                .and_modify(|ptr| *ptr = id);
            loop { 
                let get_resp = client.get(
                    &format!("{}/res.php", params.url))
                        .query(&params.get_params)
                        .timeout(Duration::from_secs(5))
                        .send();
                if let Ok(solve) = get_resp{
                    resp = serde_json::from_str(&solve.text().unwrap()).unwrap();
                    if resp.status != 1 && resp.request != "CAPCHA_NOT_READY"{
                        return Err(format!("solving error: {}", resp.request))
                    }else if resp.status == 1{
                        return Ok(resp.request);
                    }else{
                        sleep(Duration::from_secs(5))
                    }
                }else if let Err(err) = get_resp{
                    return Err(err.to_string())
                }
            }
        }else{
            return Err(resp.unwrap_err().to_string())
        }
        //Ok(String::from(sitekey))
    }

    pub struct ReCap2<'a>{
        key: &'a str,
        url: &'a str,
        captype: CapTypes,
        settings: HashMap<&'a str, &'a str>
    }

    impl<'a> ReCap2<'a>{

        pub fn new(key: &'a str, url: &'a str) -> RefCell<Self>{
            RefCell::from(Self{
                key,
                url,
                captype: CapTypes::RC2,
                settings: HashMap::new()
            })
        }

        pub fn set_target(&mut self,sitekey: &'a str, pageurl: &'a str){
            self.settings.insert("sitekey", sitekey);
            self.settings.insert("pageurl", pageurl);
        }

        fn capsolve(&self) -> Result<String,String>{
            let (sitekey, pageurl) = match (self.settings.get("sitekey"), self.settings.get("pageurl")) {
                (Some(sitekey), Some(pageurl)) => (String::from(*sitekey), String::from(*pageurl)),
                _ => return Err(String::from("please first use set_target"))
            };
            let json = serde_json::json!({
                "url": self.url,
                "post_params": {
                    "key": self.key,
                    "method": "userrecaptcha",
                    "googlekey": sitekey,
                    "pageurl": pageurl,
                    "softguru": "104431",
                    "json": 1
                },
                "get_params": {
                    "key": self.key,
                    "action": "get",
                    "id": 0,
                    "json": 1
                }
            });
            let mut params: ReCapRequest = serde_json::from_value(json).unwrap();
            
            rude_solve(&mut params)
        }
    }

    impl<'a> CapSolvable for ReCap2<'a>{

        fn get_type(&self) -> &CapTypes {
            &self.captype
        }

        fn solve(&self) -> Result<String,String> {
            self.capsolve()
        }

        //fn set_settings(&mut self, settings: HashMap<&'a str,&'a str>) {
        //    for (key, value) in settings{
        //        self.settings.insert(key, value);
        //    }
        //}
    }

}