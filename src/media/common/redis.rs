use std::collections::HashMap;
use redis::{Commands, Connection, FromRedisValue, RedisError, RedisResult};

// redis 链接信息
pub struct MyRedis(Connection);

impl MyRedis{
    pub fn new() -> Self{
        let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
        let mut con = client.get_connection().unwrap();
         MyRedis(con)
    }

    // 设置Redis的值
    pub fn set_cache<T,U>(&mut self,key:T,value:U) where T:redis::ToRedisArgs,U:redis::ToRedisArgs  {
        let res:RedisResult<String> = self.0.set(key, value);
        if let Ok(_s) = res {
            println!("I am ! i can got the data! {}",_s)
        }else {
            println!("i am failed!")
        }
    }

    // 获取redis 值 指定函数的返回值的实现
    pub fn get_cache<RV>(&mut self, k:&str) ->RedisResult<RV> where RV:redis::FromRedisValue{
     let d:RedisResult<RV> = self.0.get(k);
        d
    }

    // exists 缓存是否存在
    pub fn exists_cache<T>(&mut self,key:T) ->bool where T:redis::ToRedisArgs {
        let s:bool =  self.0.exists(key).unwrap();
        s
    }

    // haspMap
    pub fn haspMap_cache<T,U>(&mut self,key:&str)->HashMap<T,U>
        where T:redis::FromRedisValue + std::cmp::Eq + std::hash::Hash,
              U:FromRedisValue{
        let map:RedisResult<HashMap<T,U>> = self.0.hgetall(key);
        map.unwrap()
    }
}