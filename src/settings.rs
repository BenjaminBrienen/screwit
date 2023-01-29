use clap::Subcommand;

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Subcommand)]
pub enum SettingCommand
{
	Set
	{
		key: String, value: String
	},
	Get
	{
		key: String
	},
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum SettingResponse
{
	Set(Result<(), String>),
	Get(SettingValue),
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Setting
{
	value: SettingValue,
	name:  String,
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum SettingValue
{
	String(String),
	Integer(u64),
	List(Vec<SettingValue>),
}

pub fn setting(setting: SettingCommand) -> SettingResponse
{
	println!("{:?}", setting);
	match setting
	{
		SettingCommand::Set { key, value } => SettingResponse::Set(Setting::set(key, value)),
		SettingCommand::Get { key } => SettingResponse::Get(Setting::get(key)),
	}
}

impl Setting
{
	pub fn set(
		key: String,
		value: String,
	) -> Result<(), String>
	{
		todo!()
	}

	pub fn get(key: String) -> SettingValue { todo!() }
}
