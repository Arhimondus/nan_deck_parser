#[derive(PartialEq, Debug)]
pub struct Effect(pub String);

#[derive(PartialEq, Debug)]
pub struct Color(pub String);

#[derive(PartialEq, Debug)]
pub enum Unit {
	MM,
	SM,
	DM,
}

#[derive(PartialEq, Debug)]
pub enum Orientation {
	Portrait,
	Album,
}

#[derive(PartialEq, Debug)]
pub struct Page {
	pub width: u32,
	pub height: u32,
	pub orientation: Orientation,
}

#[derive(PartialEq, Debug)]
pub enum BorderType {
	Rectangle,
	Rounded,
}

#[derive(PartialEq, Debug)]
pub struct Border {
	pub r#type: BorderType,
	pub color: Color,
	pub size: u8,
}

#[derive(PartialEq, Debug)]
pub struct Image {
	pub col_path: String,
	pub name: String,
	pub left: Numeric,
	pub top: Numeric,
	pub width: Numeric,
	pub height: Numeric,
}

#[derive(PartialEq, Debug)]
pub enum HorizontalAlign {
	Left,
	Center,
	Right,
}

#[derive(PartialEq, Debug)]
pub enum VerticalAlign {
	Top,
	Center,
	Bottom,
	WwTop, // For TEXT
	WwBottom, // For TEXT
	WwCenter, // For TEXT
}

#[derive(PartialEq, Debug)]
pub struct TextFont {
	pub col_path: String,
	pub name: String,
	pub left: Numeric,
	pub top: Numeric,
	pub width: Numeric,
	pub height: Numeric,
	pub horizontal_align: HorizontalAlign,
	pub vertical_align: VerticalAlign,
	pub rotation: i32,
	pub alpha: u32,
	pub font_name: String,
	pub font_size: u8,
	pub effect: Effect,
	pub color: Color,
}

#[derive(PartialEq, Debug)]
pub struct Visual {
	pub horizontal_step: u32,
	pub vertical_step: u32,
}

#[derive(PartialEq, Debug)]
pub enum Command {
	LinkMulti(String),
	Link(Link),
	Unit(Unit),
	Page(Page),
	Border(Border),
	Visual(Visual),
	Image(Image),
	TextFont(TextFont),
	EndVisual,
}

#[derive(PartialEq, Debug)]
pub struct Link {
	pub file: String,
	pub sheet: Option<String>,
}

fn linkmulti(value: &str) -> Command {
	Command::LinkMulti(value.to_string())
}

fn link(value: &str) -> Command {
	let (file, sheet) = {
		let splitted = value.trim().trim_matches('\"').split('!').collect::<Vec<&str>>();
		if splitted.len() == 2 {
			(splitted[0].to_string(), Some(splitted[1].to_string()))
		} else {
			(splitted[0].to_string(), None)
		}
	};

	Command::Link(Link {
		file,
		sheet,
	})
}

fn unit(value: &str) -> Command {
	match value {
		"MM" => Command::Unit(Unit::MM),
		"SM" => Command::Unit(Unit::SM),
		"DM" => Command::Unit(Unit::DM),
		_ => panic!("Unknown unit size"),
	}
}

fn page(value: &str) -> Command {
	let splitted = value.split(',').collect::<Vec<&str>>();
	Command::Page(Page {
		width: splitted[0].parse().unwrap(),
		height: splitted[1].parse().unwrap(),
		orientation: match splitted[2].trim() {
			"PORTRAIT" => Orientation::Portrait,
			"ALBUM" => Orientation::Album,
			_ => panic!("Unknown orientation type"),
		}
	})
}

fn border(value: &str) -> Command {
	let splitted = value.split(',').collect::<Vec<&str>>();
	Command::Border(Border {
		r#type: match splitted[0].trim() {
			"RECTANGLE" => BorderType::Rectangle,
			"ROUNDED" => BorderType::Rounded,
			_ => panic!("Unknown border type"),
		},
		color: Color(splitted[1].to_lowercase().to_string()),
		size: splitted[2].trim().parse().unwrap(),
	})
}

fn visual(value: &str) -> Command {
	let splitted = value.split(',').collect::<Vec<&str>>();
	Command::Visual(Visual {
		horizontal_step: splitted[1].trim().parse().unwrap(),
		vertical_step: splitted[2].trim().parse().unwrap(),
	})
}

#[derive(PartialEq, Debug)]
pub enum Numeric {
	Absolute(i32),
	Percentage(i32),
}

fn parse_name(value: &str) -> String {
	value.trim_start_matches("[").trim_end_matches("]").to_string()
}

fn parse_numeric_value(value: &str) -> Numeric {
	if value.ends_with('%') {
		Numeric::Percentage(value.trim().trim_end_matches("%").parse().unwrap())
	} else {
		Numeric::Absolute(value.trim().parse().unwrap())
	}
}

fn image(value: &str) -> Command {
	let splitted = value.split(',').collect::<Vec<&str>>();
	
	Command::Image(Image {
		col_path: splitted[0].trim_matches('\"').to_string(),
		name: parse_name(splitted[1]),
		left: parse_numeric_value(splitted[2]),
		top: parse_numeric_value(splitted[3]),
		width: parse_numeric_value(splitted[4]),
		height: parse_numeric_value(splitted[5]),
	})
}

fn textfont(value: &str) -> Command {
	let splitted = value.split(',').collect::<Vec<&str>>();
	
	Command::TextFont(TextFont {
		col_path: splitted[0].trim_matches('\"').to_string(),
		name: parse_name(splitted[1]),
		left: parse_numeric_value(splitted[2]),
		top: parse_numeric_value(splitted[3]),
		width: parse_numeric_value(splitted[4]),
		height: parse_numeric_value(splitted[5]),
		horizontal_align: match splitted[6].trim() {
			"LEFT" => HorizontalAlign::Left,
			"CENTER" => HorizontalAlign::Center,
			"RIGHT" => HorizontalAlign::Right,
			_ => panic!("Unknown horizontal align"),
		},
		vertical_align: match splitted[7].trim() {
			"TOP" => VerticalAlign::Top,
			"CENTER" => VerticalAlign::Center,
			"BOTTOM" => VerticalAlign::Bottom,
			"WWTOP" => VerticalAlign::WwTop,
			"WWCENTER" => VerticalAlign::WwCenter,
			"WWBOTTOM" => VerticalAlign::WwBottom,
			_ => panic!("Unknown vertical align"),
		},
		rotation: splitted[8].parse().unwrap(),
		alpha: splitted[9].parse().unwrap(),
		font_name: splitted[10].to_string(),
		font_size: splitted[11].parse().unwrap(),
		effect: Effect(splitted[12].to_lowercase().to_string()),
		color: if splitted.len() > 13 { Color(splitted[13].to_lowercase().to_string()) } else { Color("black".to_string()) },
	})
}

fn endvisual() -> Command {
	Command::EndVisual
}

pub fn nan_deck_parse(data: impl Into<String>) -> Vec<Command> {
	let data = data.into();
	let lines = data.trim().split('\n').map(|it| it.trim()).filter(|it| !it.starts_with(';')).collect::<Vec<&str>>();
	lines.iter().map(|line| {
		let splitted = line.split('=').collect::<Vec<&str>>();
		let command = splitted[0].trim();
		match command {
			"LINKMULTI" => linkmulti(splitted[1].trim()),
			"LINK" => link(splitted[1].trim()),
			"UNIT" => unit(splitted[1].trim()),
			"PAGE" => page(splitted[1].trim()),
			"BORDER" => border(splitted[1].trim()),
			"VISUAL" => visual(splitted[1].trim()),
			"IMAGE" => image(splitted[1].trim()),
			"TEXTFONT" => textfont(splitted[1].trim()),
			"ENDVISUAL" => endvisual(),
			_ => panic!("Unknown command"),
		}
	}).collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn link() {
		let parsed = nan_deck_parse(r#"
			LINK= "cards.xlsx"
		"#);
		assert_eq!(parsed[0], Command::Link(Link {
			file: "cards.xlsx".into(),
			sheet: None,
		}));
	}

	#[test]
	fn link_with_sheet() {
		let parsed = nan_deck_parse(r#"
			LINK= "1SJdrYEP70GkcQ9vzmA7J-k4THJnsiQdGIxvZtUSJcwE!cards"
		"#);
		assert_eq!(parsed[0], Command::Link(Link {
			file: "1SJdrYEP70GkcQ9vzmA7J-k4THJnsiQdGIxvZtUSJcwE".into(),
			sheet: Some("cards".into()),
		}));
	}

	#[test]
	fn text_font() {
		let parsed = nan_deck_parse(r#"
			TEXTFONT="1-{(NAME)}",[NAME],0%,0%,100%,100%,CENTER,TOP,0,100,Arial,14,T,#CC9900
		"#);
		assert_eq!(parsed[0], Command::TextFont(TextFont { 
			col_path: "1-{(NAME)}".into(),
			name: "NAME".into(),
			left: Numeric::Percentage(0),
			top: Numeric::Percentage(0),
			width: Numeric::Percentage(100),
			height: Numeric::Percentage(100),
			horizontal_align: HorizontalAlign::Center,
			vertical_align: VerticalAlign::Top,
			rotation: 0,
			alpha: 100,
			font_name: "Arial".into(),
			font_size: 14,
			effect: Effect("t".into()),
			color: Color("#cc9900".into())
		}));
	}

	#[test]
	fn text_font_without_color() {
		let parsed = nan_deck_parse(r#"
			TEXTFONT="1-{(NAME)}",[NAME],-5%,2%,125%,145%,LEFT,WWTOP,45,99,Arial,16,T
		"#);
		assert_eq!(parsed[0], Command::TextFont(TextFont { 
			col_path: "1-{(NAME)}".into(),
			name: "NAME".into(),
			left: Numeric::Percentage(-5),
			top: Numeric::Percentage(2),
			width: Numeric::Percentage(125),
			height: Numeric::Percentage(145),
			horizontal_align: HorizontalAlign::Left,
			vertical_align: VerticalAlign::WwTop,
			rotation: 45,
			alpha: 99,
			font_name: "Arial".into(),
			font_size: 16,
			effect: Effect("t".into()),
			color: Color("black".into())
		}));
	}

	#[test]
	fn full_example() {
		let parsed = nan_deck_parse(r#"
			LINKMULTI="Quantity"
			LINK= "1SJdrYEP70GkcQ9vzmA7J-k4THJnsiQdGIxvZtUSJcwE!cards"
			UNIT= MM
			PAGE=207,297, PORTRAIT
			BORDER= RECTANGLE, #000000, 1
			VISUAL=, 10, 10
			IMAGE="1-{(IMAGE)}",[IMAGE],0%,0%,100%,100%
			;Имя
			TEXTFONT="1-{(NAME)}",[NAME],0%,0%,100%,100%,CENTER,TOP,0,100,Arial,14,T,#CC9900
			;Уровень
			TEXTFONT="1-{(LVL)}",[LVL],0%,0%,100%,100%,LEFT,TOP,0,100,Arial,32,T
			;Урон
			TEXTFONT="1-{(DAMAGE)}",[DAMAGE],0%,0%,100%,100%,LEFT,BOTTOM,0,100,Arial,32,T
			;Модификатор
			TEXTFONT="1-{(X)}",[X],0%,0%,100%,100%,CENTER,BOTTOM,0,100,Arial,32,T
			;Здоровье
			TEXTFONT="1-{(HEALTH)}",[HEALTH],0%,0%,100%,100%,RIGHT,BOTTOM,0,100,Arial,32,T
			;Броня
			TEXTFONT="1-{(BLOCK)}",[BLOCK],0%,0%,100%,100%,LEFT,CENTER,0,100,Arial,32,T
			;Тригер
			TEXTFONT="1-{(TRIGGER)}",[TRIGGER],0%,0%,100%,100%,RIGHT,TOP,0,100,Arial,10,T
			;Способность
			TEXTFONT="1-{(SPELL)}",[SPELL],0%,-12%,100%,100%,CENTER,WWBOTTOM,0,100,Arial,10,T
			ENDVISUAL
		"#);
		println!("{parsed:#?}");
		assert_eq!(parsed.len(), 16);
	}
}
