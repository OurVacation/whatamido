# 📅 WhatAmIDo: Rust-Powered Calendar Assistant

## `serde` crate를 활용하여, .json 파일을 읽어와서 그것을 바탕으로 동일한 로직 구현 (26/03/06~)

### 핵심 목표
* json을 파싱해서 내가 정의한 구조체에 올바르게 넣고 처리하는 것을 해보기

### 수행과정 및 학습 내용
1. **Cargo.toml에서 crate를 추가할 때 왜 version과 feature를 구분해서 적는 이유**
    - version은 말 그대로 라이브러리의 버전, feature는 해당 라이브러리의 선택 옵션 중 무엇을 킬지를 선택하는 것이다.
    - 이렇게 하는 이유는 그냥 라이브러리를 모든 기능을 생으로 다 가져오면 크기도 너무 크고 쓰지도 않는 기능을 가져오기 위해 바이너리 크기도 커지고, 컴파일 시간도 오래 걸린다.
	- 왜 features=[] 이렇게 대괄호로 표시할까? => 이건 해당 라이브러리 선택 기능 중 한번에 여러개를 켜기 위해 TOML 문법의 리스트(`[]`)를 활용하는 것.

2. `serde` , `serde_json` 사용 방법 : 모든 연관함수까지는 아니더라도 알게 된 건 적어두자
	1. from_str
		```rust
		pub fn from_str<'a, T>(s: &'a str) -> Result<T>where
    	T: Deserialize<'a>,
		```
	-
### 해당 단계 피드백
(내용을 입력하세요)
