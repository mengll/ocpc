/*
* @desc 今日头条的模块的数据上报媒体
* @desc 激活，注册，付费
* @desc 文档地址 https://open.oceanengine.com/labels/7/docs/1696710655781900
* @desc 激活注册数据回调文档 https://open.oceanengine.com/labels/7/docs/1696710656359439
* @desc https://open.oceanengine.com/labels/7/docs/1696710674252815
*/

use actix_web::{get, HttpRequest, HttpResponse, Responder, web};
use reqwest::Client;
use time::macros::date;
use crate::media::common::media::{Media, MediaReport};
use crate::media::jrtt::model::{getOs, Info};
use time::OffsetDateTime;
use   serde::Serialize;

#[derive(Serialize)]
struct Params {
    event_type: i8,
    conv_time: i64,
}

// 今日头条
pub(crate) struct Jrtt<'a>(MediaReport<'a>);

impl <'a> Media for Jrtt<'a>{
    type RS = ();
    type Err = String;
     /*
       @desc 媒体对应的激活数据上报接口
       @author mll
      */
    fn report_active(&self) -> Result<Self::RS, Self::Err> {
        println!("{} {}",self.0.action,self.0.akey);
        self.tt_get(0);
        Ok(())
    }

    /*
      @desc 媒体对应的注册数据上报接口
      @author mll
     */
    fn report_register(&self) -> Result<Self::RS, Self::Err> {
        self.tt_get(1);
        Ok(())
    }

    /*
      @desc 媒体对应的付费数据上报接口
      @author mll
     */
    fn report_pay(&self) -> Result<Self::RS, Self::Err> {
        self.tt_get(3);
        Ok(())
    }
}

impl Jrtt<'_> {
    /**
     * @desc 通用服务请求接口
     */
    fn tt_get(&self,ac:i8){
        let client = Client::new();
        let some_url = self.0.callback_url;
        let r = client.get(some_url);
        let params = Params {
            event_type: ac,
            conv_time:self.0.conv_time,
        };
        let r = r.query(&params);
        let req = r.build().expect("request is valid");
    }
}

#[get("/media/api/jrtt")]
pub async fn jrtt(reg:web::Query<Info>) -> impl Responder {
    println!("{:?}",reg.name);
    println!("「」{:?} {:?}",reg.model,reg.adt_cid);
    println!("「」{:?} {:?}",reg.model,reg.adt_cid);
    println!("os={}",getOs(reg.os));
    let t = OffsetDateTime::now_utc().unix_timestamp();
    println!("i am get time! {}",t);
    // 数据标准化 流入mq 进入下一步数据流转

    match reg.os {
        1=>{
            // ios 数据使用全平台数据匹配
        }
        _=>{
            // 安卓或wp 使用媒体单独的匹配接口
        }
    }


    HttpResponse::Ok().body("I am Jrtt!")
}