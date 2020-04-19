#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum UserRole {
	Unprivileged,
	Moderator,
	Admin,
	Publisher,
}

impl From<UserRole> for i32 {
	fn from(role : UserRole) -> i32 {
		match role {
			UserRole::Unprivileged => 1,
			UserRole::Moderator => 2,
			UserRole::Admin => 3,
			UserRole::Publisher => 4,
		}
	}
}

impl From<i32> for UserRole {
	fn from(number : i32) -> UserRole {
		match number {
			1 => UserRole::Unprivileged,
			2 => UserRole::Moderator,
			3 => UserRole::Admin,
			4 => UserRole::Publisher,
			_ => {
				eprintln!("Tried to convert an unsupported number into a user role");
				UserRole::Unprivileged
			},
		}
	}
}
