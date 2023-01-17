use serde::Deserialize;

// 媒体日志的行为
pub const SHOW:i8 = 1;// 展示
pub const CLICK:i8 = 2 ; // 广告点击
pub const LOOK_AND_CLICK:i8 = 3;// 播放点击 （有效播放视频播放 5s 示为触发一次点击）
pub const LOOK:i8 = 4;// 视频查看
pub const LOOK_COMPLETE:i8 = 5;  // 视频播放完成

pub trait Media {
    type RS;
    type Err;
    // 上报激活
    fn report_active(&self)->Result<Self::RS,Self::Err>;
    // 上报注册
    fn report_register(&self)->Result<Self::RS,Self::Err>;
    // 上报付费
    fn report_pay(&self)->Result<Self::RS,Self::Err>;
}

// 通用数据上报结构
pub struct MediaReport<'a> {
    pub imei_md5:Option<&'a str>,     // 安卓设备信息
    pub  imei2_md5:Option<&'a str>,    // 安卓设备imei2
    pub meid_md5:Option<&'a str>,     // meid 对于获取不到的设备信息可能会获取到当前的值
    pub oaid:Option<&'a str>,         // 安卓设备的信息的原值 安卓10 以上
    pub oaid_md5:Option<&'a str>,     // 安卓设备信息的MD5 安卓10 以上
    pub akey :Option<&'a str>,        // 上报信息的密码
    pub callback_url:Option<&'a str>, // 数据上报的回调地址
    pub ip:Option<&'a str>,           // 客户端的IP地址
    pub ua:&'a str,           // user_agent 浏览器的用户的信息
    pub action:i8,            // 1 激活 2 注册 3 付费
    pub game_id:Option<i32>,          // 游戏ID
    pub package_id:Option<i32>,       // 游戏包ID
    pub money:Option<i32>,            // 游戏的服务金额 单位为分
    pub idfa_md5:Option<&'a str> ,    // ios的设备信息
    pub caid :Option<&'a str>,       // iOS的设备的信息
    pub caid1 :Option<&'a str>,      // iOS的设备的信息card1
    pub caid_version:Option<&'a str>, // ios 中广协的版本
    pub caid1_version:Option<&'a str>,// ios 中广协的版本
    pub keys:Option<&'a str>,         // 加密秘钥
    pub bd_id:Option<&'a str>,        // 设备合一ID
    pub req_id:Option<&'a str>,       // 请求ID
    pub conv_time:i64         // 转化时间
}

type middler_func = fn(c:MediaReport);

/*
 * 媒体策略层
 * 减少 或 修改 回传媒体的基础数据 动态的数据类型的分发的实现
 */
fn media_middler<T,U>(a : &dyn Media<RS=T,Err=U>){

    //
}