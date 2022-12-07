
// 媒体日志的行为
pub(crate) enum MediaAc {
    SHOW=1,             // 展示
    CLICK=2,            // 广告点击
    LOOK_AND_CLICK=3,   // 播放点击 （有效播放视频播放 5s 示为触发一次点击）
    LOOK=4,             // 视频查看
    LOOK_COMPLETE=5     // 视频播放完成
}

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
    pub imei_md5:&'a str,     // 安卓设备信息
    pub  imei2_md5:&'a str,    // 安卓设备imei2
    pub meid_md5:&'a str,     // meid 对于获取不到的设备信息可能会获取到当前的值
    pub oaid:&'a str,         // 安卓设备的信息的原值 安卓10 以上
    pub oaid_md5:&'a str,     // 安卓设备信息的MD5 安卓10 以上
    pub akey :&'a str,        // 上报信息的密码
    pub callback_url:&'a str, // 数据上报的回调地址
    pub ip:&'a str,           // 客户端的IP地址
    pub ua:&'a str,           // user_agent 浏览器的用户的信息
    pub action:i8,            // 1 激活 2 注册 3 付费
    pub game_id:i32,          // 游戏ID
    pub package_id:i32,       // 游戏包ID
    pub money:i32,            // 游戏的服务金额 单位为分
    pub idfa_md5:&'a str ,    // ios的设备信息
    pub caid :&'a str ,       // iOS的设备的信息
    pub caid1 :&'a str ,      // iOS的设备的信息card1
    pub caid_version:&'a str, // ios 中广协的版本
    pub caid1_version:&'a str,// ios 中广协的版本
    pub keys:&'a str,         // 加密秘钥
    pub bd_id:&'a str,        // 设备合一ID
    pub req_id:&'a str,       // 请求ID
}