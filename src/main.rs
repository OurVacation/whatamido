mod plan; // plan.rs 파일을 모듈로 선언
// mod는 파일을 프로젝트에 연결
use plan::{Plan, FilePlan, State}; // plan.rs 파일에서 사용할 타입들을 스코프로 가져오기
// use는 파일 내의 내용물을 이름표를 달아라

use chrono::{Local,NaiveTime, TimeZone};
use std::thread;
use std::time::Duration;
use std::io::{self, Write};
use std::fs;



fn main()
{
    println!("🚀 whatamido CLI 시작\n");

    let mut plan_arr = load_plans_from_file("plans.json");
    println!("=== 📅 오늘 불러온 계획 목록 ===");
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

            if plan.state == State::Waiting && now >= plan.start_time {
                plan.state = State::Inprogress;
                println!("[{}] {} 시작",plan.start_time.format("%H:%M"), plan.name);
            }

            if plan.state == State::Inprogress && now >= plan.end_time
            {
                println!("[{}] {} 종료",plan.end_time.format("%H:%M"), plan.name);
                plan.state = State::Done;

                let progress =  get_user_input("진척도 입력 해주세요(0~100)")
                .parse::<u8>().expect("String::parse Error");
                plan.update_progress(progress);
                plan.state = State::Done;
            }
        }

        if all_done{
            break;
        }

        thread::sleep(Duration::from_secs(1));
    }

    println!("\n=== 📊 오늘의 진척도 요약 ===");
    for plan in plan_arr {
        println!("- {}: {}%", plan.name, plan.progress);
    }
}


fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("입력 오류");
    input.trim().to_string()
}

// fn parse_plan(json_content : &str, plan_arr: &mut Vec<Plan>){
//     match serde_json::from_str::<Vec<FilePlan>>(&json_content){
//         Ok(parsed_plans) =>{
//             for fp in parsed_plans{
//                 let new_plan = Plan::new(&fp.name, fp.start_time, fp.end_time);
//                 plan_arr.push(new_plan);
//             }
//             println!("✅ 성공적으로 {}개의 계획을 불러왔습니다!\n", plan_arr.len());
//         }
//         Err(e) => {
//             println!("❌ JSON 파싱 에러: 파일 내용이 올바른 형식이 아닙니다. ({})", e);
//         }
//     }
// }
// 원래 내가 했던 함수

fn load_plans_from_file(file_path: &str) -> Vec<Plan> {
    // 1. 반환할 빈 벡터를 생성합니다.
    let mut plans = Vec::new();

    // 2. 파일 읽기 시도
    match fs::read_to_string(file_path) {
        Ok(json_content) => {
            // 3. 파일 읽기 성공 -> JSON 파싱 시도
            match serde_json::from_str::<Vec<FilePlan>>(&json_content) {
                Ok(parsed_plans) => {
                    for fp in parsed_plans {
                        plans.push(Plan::new(&fp.name, fp.start_time, fp.end_time));
                    }
                    println!("✅ '{}' 파일에서 {}개의 계획을 불러왔습니다!\n", file_path, plans.len());
                }
                Err(e) => println!("❌ JSON 파싱 에러: 파일 형식이 잘못되었습니다. ({})", e),
            }
        }
        Err(e) => println!("⚠️ 파일을 읽을 수 없습니다. 빈 목록으로 시작합니다. ({})", e),
    }

    // 4. 완성된(또는 비어있는) 벡터의 소유권을 반환합니다!
    plans
}
// AI가 만든거 확실히 깔끔하다!
