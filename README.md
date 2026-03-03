# 📅 WhatAmIDo: Rust-Powered Calendar Assistant

> **"계획만 세우는 것은 그만, 이제는 달성률까지 관리하세요."**
> `WhatAmIDo`는 Google Calendar와 연동하여 실시간 알림을 보내고, 작업의 진척도를 추적하여 하루의 성과를 시각화해주는 Rust 기반 CLI 도구입니다.

---

## ✨ 핵심 기능 (Features)

* **Google Calendar 동기화**: Google API를 통해 오늘의 일정을 자동으로 가져옵니다.
* **실시간 알림 (Active Notification)**: 일정 시작 시간이 되면 데스크톱 알림과 터미널 메시지로 알려줍니다.
* **진척도 추적 (Progress Tracking)**: 일정이 종료되면 해당 목표를 얼마나 달성했는지(`%`) 입력받아 기록합니다.
* **일일 요약 보고서 (Daily Summary)**: 하루가 끝날 때, 오늘 계획의 전체 달성도와 성패를 한눈에 보여줍니다.
---

## 단계별 목표 달성 

1. `chrono` crate를 사용하여 현재 시간을 가져오고, 목표 시간과 비교하여 터미널에 알람을 전송하자! (26/03/03~)
