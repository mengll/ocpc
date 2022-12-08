/*
* @desc 今日头条的模块的数据上报媒体
* @desc 激活，注册，付费
* @desc 文档地址 https://open.oceanengine.com/labels/7/docs/1696710655781900
* @desc 激活注册数据回调文档 https://open.oceanengine.com/labels/7/docs/1696710656359439
* @desc https://open.oceanengine.com/labels/7/docs/1696710674252815
*/

use actix_web::{get, HttpRequest, HttpResponse, Responder, web};
use crate::media::common::media::{Media, MediaReport};
use crate::media::jrtt::model::Info;


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
        let url = reqwest::Url::from();
        Ok(())
    }

    /*
      @desc 媒体对应的注册数据上报接口
      @author mll
     */
    fn report_register(&self) -> Result<Self::RS, Self::Err> {
        Ok(())
    }

    /*
      @desc 媒体对应的付费数据上报接口
      @author mll
     */
    fn report_pay(&self) -> Result<Self::RS, Self::Err> {
        Ok(())
    }
}

#[get("/media/api/jrtt")]
pub async fn jrtt(reg:web::Query<Info>) -> impl Responder {
    println!("{:?}",reg.name);
    println!("「」{} {}",reg.model,reg.adt_cid);
    HttpResponse::Ok().body("I am Jrtt!")
}