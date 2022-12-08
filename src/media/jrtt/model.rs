use std::collections::HashMap;
use serde::Deserialize;

// 系统类型枚举
pub fn getOs(o:i8)->&'static str {
    match o {
        0 => "andorid",
        1=> "ios",
        2=> "wp" ,
        _=> "others"
    }
}


// 今日头条回传的基础信息
#[derive(Deserialize)]
pub struct Info {
    pub name: String,       //
    pub call_back_url:Option<String>, // 回调地址
    pub adt_group:Option<String>,  // 广告组ID
    pub adt_plan:Option<String>,   // 广告计划
    pub adt_cid :Option<String>,   // 广告创意ID
    pub game_id:Option<i32>,       // 游戏ID
    pub package_id:Option<i32>,    // 游戏包ID
    pub ts:Option<i32>,            // 点击时间
    pub imei_md5:Option<String>,   // 设备的MD5
    pub oaid_md5:Option<String>,   // 安卓设备的MD5值
    pub account_id:Option<String>, // 投放的账号ID
    pub ip:Option<String>,         // IP地址
    pub ua:Option<String>,         // 浏览器user_agent
    pub bd_id:Option<String>,      // 媒体合一ID
    pub req_id:Option<String>,     // 请求ID
    pub os:i8,        // 系统类型
    pub geo:Option<String>,        // 经纬度
    pub ac:i8,                // 日志的行为
    pub model:Option<String>,      // 手机型号
    pub mac:Option<String>,        // 设备的mac地址
    pub android_id:Option<String>, // 安卓ID
    pub convert_id:Option<String>, // 转化ID
    pub csite:Option<String>,      // 广告位信息
    pub aid_name:Option<String>,   // 广告计划名称
    pub cid_name:Option<String>,   // 广告创意名称
    pub campaign_name:Option<String> // 广告组名称
}