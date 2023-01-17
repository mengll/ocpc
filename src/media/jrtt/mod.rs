use crate::media::common::media::MediaReport;

pub mod jrtt;
pub mod model;

#[derive(Debug,Copy,Clone)]
struct MatchData<'a> {
    game_id:i32,               // 游戏包ID
    muid:Option<&'a str>,      // 媒体设备识别
    imei_md5:Option<&'a str>,  // 安卓设备的 标识
    bd_id:Option<&'a str>,     // 设备的组合ID
    oaid_md5:Option<&'a str>,  // 安卓10 以上设备ID
    ip:Option<&'a str>,        // ip 地址
    caid_str:Option<&'a str>   // ios 的设备信息
}

type MatchFn<'a> = fn(d:MatchData<'a>)->bool;

//
