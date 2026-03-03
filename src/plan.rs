use chrono::{DateTime,Local};


#[derive(Debug, PartialEq)]
pub enum State
{
	Waiting,
	Inprogress,
	Done,
}

#[derive(Debug)]
pub struct Plan
{
	pub state: State,
	pub name: String,
	pub start_time : DateTime<Local>,
	pub end_time : DateTime<Local>,
	pub progress: u8,
}

impl Plan
{
	pub fn new(name: &str, start_time: DateTime<Local>, end_time: DateTime<Local>)-> Self {
		Self
		{
			state : State::Waiting,
			name: String::from(name),
			start_time,
			end_time,
			progress: 0
		}
	}

	pub fn update_progress(&mut self, new_progress: u8){
		self.progress = std::cmp::min(new_progress, 100);
		println!("'{}' 의 진척도가 {}%로 업데이트 되었습니다.", self.name, self.progress);
	}
}
