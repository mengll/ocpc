use serde::Deserialize;
use crate::media::common::media::MediaAc;

// 系统类型枚举
pub enum TtMedia {
    Android =0 ,
    iOS =1,
    WP =2,
    Others=3
}

// 今日头条回传的基础信息
#[derive(Deserialize)]
pub struct Info {
    pub name: String,       //
    pub call_back_url:String, // 回调地址
    pub adt_group:String,  // 广告组ID
    pub adt_plan:string,   // 广告计划
    pub adt_cid :string,   // 广告创意ID
    pub game_id:i32,       // 游戏ID
    pub package_id:i32,    // 游戏包ID
    pub ts:i32,            // 点击时间
    pub imei_md5:String,   // 设备的MD5
    pub oaid_md5:String,   // 安卓设备的MD5值
    pub account_id:String, // 投放的账号ID
    pub ip:String,         // IP地址
    pub ua:String,         // 浏览器user_agent
    pub bd_id:String,      // 媒体合一ID
    pub req_id:String,     // 请求ID
    pub os:TtMedia,        // 系统类型
    pub geo:String,        // 经纬度
    pub ac:MediaAc,        // 日志的行为
    pub model:String,      // 手机型号
    pub mac:String,        // 设备的mac地址
    pub android_id:String, // 安卓ID
    pub convert_id:String, // 转化ID
    pub csite:String,      // 广告位信息
    pub aid_name:String,   // 广告计划名称
    pub cid_name:String,   // 广告创意名称
    pub campaign_name:String // 广告组名称
}