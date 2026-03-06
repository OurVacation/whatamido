mod plan; // plan.rs 파일을 모듈로 선언
// mod는 파일을 프로젝트에 연결
use plan::{Plan, State}; // plan.rs 파일에서 사용할 타입들을 스코프로 가져오기
// use는 파일 내의 내용물을 이름표를 달아라

use chrono::{Local,NaiveTime, TimeZone};
use std::thread;
use std::time::Duration;
use std::io::{self, Write};

fn main()
{

}
