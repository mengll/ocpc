/*
* @desc 广点通的模块的数据上报媒体
* @desc 激活，注册，付费
* @desc 文档地址 https://developers.e.qq.com/docs/guide/conversion/new_version/dianjijiance
*/

use actix_web::{get, HttpRequest, HttpResponse, Responder, web};
use crate::media::common::media::{Media, MediaReport};


// 广点通
pub(crate) struct Jrtt<'a>(MediaReport<'a>);

impl <'a> Media for Jrtt<'a>{
    type RS = ();
    type Err = String;

    /*
      @desc 媒体对应的激活数据上报接口
      @author mll
     */
    fn report_active(&self) -> Result<Self::RS, Self::Err> {
        println!("{} {:?}",self.0.action,self.0.akey);
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

