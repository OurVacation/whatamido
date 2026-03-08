use yup_oauth2::{read_application_secret, InstalledFlowAuthenticator, InstalledFlowReturnMethod};
use tokio::runtime::Runtime;

// 🌟 구글 로그인을 수행하고 Access Token을 문자열로 반환하는 헬퍼 함수
pub fn get_google_access_token() -> String {
    println!("🔐 구글 캘린더 인증을 시작합니다...");

    // tokio 런타임을 임시로 만들어서 비동기 코드를 동기식으로 실행(block_on)시킵니다!
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        // 1. 우리가 방금 다운받은 신분증(credentials.json)을 읽어옵니다.
        let secret = read_application_secret("credentials.json")
            .await
            .expect("❌ credentials.json 파일이 프로젝트 폴더에 없습니다!");

        // 2. 인증 관리자 생성 (로그인 성공 시 'tokencache.json'에 토큰을 저장해서 매번 로그인하는 귀찮음을 방지합니다!)
        let auth = InstalledFlowAuthenticator::builder(
            secret,
            InstalledFlowReturnMethod::HTTPRedirect,
        )
        .persist_tokens_to_disk("tokencache.json")
        .build()
        .await
        .unwrap();

        // 3. 우리가 필요한 권한: 캘린더의 일정 목록을 "읽기"만 하겠다는 권한
        let scopes = &["https://www.googleapis.com/auth/calendar.readonly"];

        // 4. 여기서 마법이 일어납니다! 브라우저가 열리고 로그인을 기다립니다.
        let token = auth.token(scopes).await.expect("❌ 토큰 발급 실패");

        // 5. 성공적으로 받아온 출입증(Token) 문자열만 쏙 빼서 반환합니다.
        token.token().unwrap().to_string()
    })
}
