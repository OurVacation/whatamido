# 📅 WhatAmIDo: Rust-Powered Calendar Assistant

## 대망의 구글 캘린더 API 연결

### 핵심 목표
* 구글 API 인증

### 수행과정 및 학습 내용

#### 수행과정
1. 구글 개발자 콘솔 세팅
: GCP에서 프로젝트 생성 -> google-calender API 활성화 -> OAuth 화면구성
2. CLI에서 로그인 기능 구현 => OAuth 2.0 토큰
3. reqwest로 구글 API 데이터 요청

#### 학습내용
1. async - await : 비동기란?
2. tokio crate : 비동기 지원 크레이트
3. reqwest crate : HTTP 통신 크레이트

### 트러블 슈팅
1. 의존성 Cargo.toml에 넣고 build 시도중 `error: failed to run custom build command for `openssl-sys v0.9.111`` 발생
- 발생 이유 : reqwest 크레이트 컴파일 중 OpenSSL이 없어서 발생
: reqwest에서는 HTTPS를 사용해야 한다. 근데 HTTPS 통신을 위해서는 데이터 암호화, 복호화(TLS/SSL)이 필수적이다. 근데, 여기서 reqwest는 운영체제에 이미 있는 OpenSSL이 가져다 쓰도록 설계되어 있다. 근데 OpenSSL은 C로 만들어진 라이브러리다. 이 시점에서 Rust에서 C로 만들어진 코드를 갖다 쓰려면 FFI라는 브릿지가 필요하다. 여기서 쓰이는 크레이트 이름이 `openssl-sys` 다.
근데 또 여기서 의문점이 내 pc에 없어서 그런줄 알았는데, `where open-ssl` 하니까 또 명령어 위치는 나온다. 근데 왜 컴파일이 안된다 그러는걸까?
왜냐면 이미 컴파일된 바이너리는 있는데 그걸 구성하는 헤더나 라이브러리는 없기 떄문이다.
사실 운영체제는 작동되는 바이너리만 두고 그 헤더나 코드 파일들은 애초에 설치하지 않거나 경로 깊은 곳에 숨기는 경우가 많다고 한다. 그래서 경로가 나오는데도 위 에러가 발생한 것이다.
- 해결 방법
간단하게 Cargo.toml의 reqwest에
```toml
reqwest = { version = "0.11", default-features = false, features = ["blocking", "json", "rustls-tls"] }
```
 default-features = false 옵션 넣으니까 해결됐다. 이 옵션은 기본 경로인 C로 구성된 openssl을 사용하지 말고, 순수 rust로 만든 암호화 통신을 쓰라고 지정해준다. 기본 경로에 있는 c 파일의 의존성을 없애주니 해결됐다!
### 해당 단계 피드백
