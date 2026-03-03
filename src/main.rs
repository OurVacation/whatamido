mod plan; // plan.rs 파일을 모듈로 선언
// mod는 파일을 프로젝트에 연결
use plan::{Plan, State}; // plan.rs 파일에서 사용할 타입들을 스코프로 가져오기
// use는 파일 내의 내용물을 이름표를 달아라

use chrono::{Local,NaiveTime, TimeZone};
use std::thread;
use std::time::Duration;
use std::io::{self, Write};

// 1. 반복되는 입력을 처리해주는 헬퍼 함수
fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("입력 오류");
    input.trim().to_string()
}

// 2. "HH:MM"을 오늘 날짜의 DateTime<Local>로 바꿔주는 함수
fn parse_time(time_str: &str) -> Result<chrono::DateTime<Local>, chrono::ParseError> {
    // 1) "14:30" 같은 문자열을 시간(Time)으로 변환
    let time = NaiveTime::parse_from_str(time_str, "%H:%M:%S")?;

    // 2) 오늘 날짜 가져오기
    let today = Local::now().date_naive();

    // 3) 오늘 날짜 + 입력받은 시간 결합
    let datetime = Local.from_local_datetime(&today.and_time(time)).unwrap();

    Ok(datetime)
}

fn main() {
    println!("🚀 whatamido CLI 시작\n");

    // 계획들을 담을 Vector 생성 (루프 바깥에 있어야 데이터가 유지됩니다!)
    let mut plan_arr: Vec<Plan> = Vec::new();

    loop {
        println!("\n--- 새로운 계획 입력 (종료하려면 'q' 입력) ---");

        let name = get_user_input("계획 이름을 입력하세요: ");
        if name == "q" {
            break;
        }

        let start_str = get_user_input("시작 시간 (HH:MM:SS): ");
        let end_str = get_user_input("종료 시간 (HH:MM:SS): ");

        let start_time = match parse_time(&start_str) {
            Ok(t) => t,
            Err(_) => {
                println!("⚠️ 시간 형식이 잘못되었습니다. 다시 입력해주세요.");
                continue; // 루프의 처음으로 돌아감
            }
        };

        let end_time = match parse_time(&end_str) {
            Ok(t) => t,
            Err(_) => {
                println!("⚠️ 시간 형식이 잘못되었습니다. 다시 입력해주세요.");
                continue;
            }
        };

        let new_plan = Plan::new(&name, start_time, end_time);
        plan_arr.push(new_plan);
        println!("✅ '{}' 계획이 추가되었습니다!", name);
    }

    // 입력받은 계획들 확인하기
    println!("\n=== 📅 오늘의 계획 목록 ===");
    for plan in &plan_arr {
        println!("- [{}] {} ~ {}",
            plan.name,
            plan.start_time.format("%H:%M"),
            plan.end_time.format("%H:%M")
        );
    }

    loop {
        let mut all_done = true;
        let now = Local::now();

        for plan in &mut plan_arr{
            if plan.state != State::Done{
                all_done = false;
            }

            if (plan.state == State::Waiting && now >= plan.start_time) {
                plan.state = State::Inprogress;
                println!("[{}] {} 시작",plan.start_time.format("%H:%M"), plan.name);
            }

            if (plan.state == State::Inprogress && now >= plan.end_time)
            {
                println!("[{}] {} 종료",plan.end_time.format("%H:%M"), plan.name);
                plan.state = State::Done;

                let progress =  get_user_input("진척도 입력 해주세요(0~100)")
                .parse::<u8>().expect("String::parse Error");
                plan.update_progress(progress);
                plan.state = State::Done;
            }
        }

        if (all_done){
            break;
        }
        thread::sleep(Duration::from_secs(1));
    }

    println!("\n=== 📊 오늘의 진척도 요약 ===");
    for plan in plan_arr {
        println!("- {}: {}%", plan.name, plan.progress);
    }
}
