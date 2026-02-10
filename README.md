# 📅 WhatAmIDo: Rust-Powered Calendar Assistant

> **"계획만 세우는 것은 그만, 이제는 달성률까지 관리하세요."**
> `WhatAmIDo`는 Google Calendar와 연동하여 실시간 알림을 보내고, 작업의 진척도를 추적하여 하루의 성과를 시각화해주는 Rust 기반 CLI 도구입니다.

---

## ✨ 핵심 기능 (Features)

* **Google Calendar 동기화**: Google API를 통해 오늘의 일정을 자동으로 가져옵니다.
* **실시간 알림 (Active Notification)**: 일정 시작 시간이 되면 데스크톱 알림과 터미널 메시지로 알려줍니다.
* **진척도 추적 (Progress Tracking)**: 일정이 종료되면 해당 목표를 얼마나 달성했는지(`%`) 입력받아 기록합니다.
* **일일 요약 보고서 (Daily Summary)**: 하루가 끝날 때, 오늘 계획의 전체 달성도와 성패를 한눈에 보여줍니다.
* **비동기 설계 (Async Architecture)**: `Tokio`를 사용하여 알림 감시와 사용자 입력이 끊김 없이 동시에 이루어집니다.

---

## 🛠 기술 스택 (Tech Stack)

| 분류 | 기술 / 라이브러리 (Crates) |
| :--- | :--- |
| **Language** | `Rust (Edition 2021)` |
| **Runtime** | `Tokio` (Asynchronous Runtime) |
| **Auth/API** | `yup-oauth2`, `google-calendar3` |
| **CLI UI** | `Clap` (Args), `Indicatif` (Progress Bar), `Dialoguer` |
| **Time** | `Chrono` |
| **Data** | `Serde`, `Serde_json` |
| **Notify** | `Notify-rust` |

---

## 🚀 시작하기 (Getting Started)

### 1. 전제 조건 (Prerequisites)
이 프로그램을 실행하려면 Google Cloud Console에서 발급받은 OAuth 2.0 클라이언트 자격 증명이 필요합니다.

1. [Google Cloud Console](https://console.cloud.google.com/)에서 프로젝트를 생성합니다.
2. **Google Calendar API**를 활성화합니다.
3. **OAuth 클라이언트 ID** (데스크톱 앱)를 생성하고 JSON 파일을 다운로드합니다.
4. 다운로드한 파일의 이름을 `credentials.json`으로 변경하여 본 프로젝트의 루트 디렉토리에 저장합니다.

### 2. 설치 및 실행 (Installation)

```bash
# 저장소 복제
git clone [https://github.com/your-username/whatamido.git](https://github.com/your-username/whatamido.git)
cd whatamido

# 빌드 및 실행
cargo run
```

> **Note**: 첫 실행 시 브라우저가 열리며 Google 계정 로그인이 필요합니다. 인증 정보는 로컬의 `tokens.json`에 안전하게 저장됩니다.

---

## 📂 프로젝트 구조 (Architecture)

```text
whatamido/
├── src/
│   ├── main.rs          # 엔트리 포인트 및 메인 루프
│   ├── auth.rs          # Google OAuth2 인증 로직
│   ├── calendar.rs      # API 통신 및 일정 파싱
│   ├── storage.rs       # 로컬 데이터 저장 및 불러오기
│   └── ui.rs            # 터미널 UI 및 알림 로직
├── Cargo.toml           # 의존성 관리
└── credentials.json     # 구글 API 키 (사용자 준비)
```

---

## 🗺 로드맵 (Roadmap)

- [x] Google OAuth2 인증 구현
- [ ] 오늘 일정 API 동기화 기능
- [ ] 비동기 시간 감시 루프 및 알림 기능
- [ ] 사용자 진척도 입력 시스템
- [ ] 일일 요약 보고서 생성 로직

---

## 🤝 기여하기 (Contributing)

이 프로젝트는 Rust 초보자가 실제 프로젝트를 진행하며 성장하기 위해 만들어진 오픈소스 프로젝트입니다. 버그 제보나 기능 제안은 언제나 환영합니다!
