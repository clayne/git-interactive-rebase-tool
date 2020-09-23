use crate::config::key_bindings::KeyBindings;
use crate::config::Config;
use crate::display::curses::{Curses, Input as PancursesInput};
use crate::display::Display;
use crate::git_interactive::GitInteractive;
use crate::input::input_handler::InputHandler;
use crate::input::Input;
use crate::list::line::Line;
use crate::process::exit_status::ExitStatus;
use crate::process::process_module::ProcessModule;
use crate::process::process_result::ProcessResult;
use crate::process::state::State;
use crate::view::testutil::render_view_data;
use crate::view::View;
use anyhow::Error;
use std::env::set_var;
use std::path::{Path, PathBuf};

#[derive(Copy, Clone, Debug)]
pub struct ProcessModuleTestState {
	pub position: (i32, i32),
	pub view_size: (i32, i32),
	pub state: Option<(State, State)>,
}

pub fn get_test_todo_path() -> PathBuf {
	Path::new(env!("CARGO_MANIFEST_DIR"))
		.join("test")
		.join("git-rebase-todo-scratch")
}

pub fn panic_output_neq(expected: &str, actual: &str) {
	panic!(vec![
		"\n",
		"Unexpected output!",
		"==========",
		"Expected:",
		expected.replace(" ", "·").replace("\t", "   →").as_str(),
		"==========",
		"Actual:",
		actual.replace(" ", "·").replace("\t", "   →").as_str(),
		"==========\n"
	]
	.join("\n"));
}

pub fn _process_module_test<F, C>(
	lines: &[&str],
	module_state: ProcessModuleTestState,
	input: &Option<Vec<Input>>,
	expected_output: &[String],
	get_module: F,
	callback: C,
) where
	F: for<'p> FnOnce(&Config, &'p Display<'p>) -> Box<dyn ProcessModule + 'p>,
	C: for<'p> FnOnce(&'p mut (dyn ProcessModule + 'p), &'p mut GitInteractive),
{
	set_var(
		"GIT_DIR",
		Path::new(env!("CARGO_MANIFEST_DIR"))
			.join("test")
			.join("fixtures")
			.join("simple")
			.to_str()
			.unwrap(),
	);
	let config = Config::new().unwrap();
	let mut curses = Curses::new();
	curses.mv(module_state.position.1, module_state.position.0);
	curses.resize_term(module_state.view_size.1, module_state.view_size.0);
	if let Some(ref input) = *input {
		for i in input {
			curses.push_input(map_input_to_curses(&config.key_bindings, *i));
		}
	}
	let display = Display::new(&mut curses, &config.theme);
	let view = View::new(&display, &config);
	let mut git_interactive = GitInteractive::new(
		lines.iter().map(|l| Line::new(l).unwrap()).collect(),
		get_test_todo_path(),
		"#",
	)
	.unwrap();
	let mut module = get_module(&config, &display);
	if let Some((_, previous_state)) = module_state.state {
		module.activate(&git_interactive, previous_state).unwrap();
	}
	if let Some(ref input) = *input {
		let input_handler = InputHandler::new(&display, &config.key_bindings);
		for _ in input {
			module.handle_input(&input_handler, &mut git_interactive, &view);
		}
	}
	callback(module.as_mut(), &mut git_interactive);
	let view_data = module.build_view_data(&view, &git_interactive);
	let expected = expected_output.join("\n");
	let output = render_view_data(view_data);
	if output != expected {
		panic_output_neq(expected.as_str(), output.as_str());
	}
}

#[macro_export]
macro_rules! process_module_state {
	(new_state = $new_state:expr, previous_state = $previous_state:expr) => {
		crate::process::testutil::ProcessModuleTestState {
			position: (0, 0),
			view_size: (50, 30),
			state: Some(($new_state, $previous_state)),
			}
	};
}

#[macro_export]
macro_rules! build_render_output {
	($($arg:expr),*) => {{
		let mut args = vec![];
		$( args.push(String::from($arg)); )*
		args
	}};
}

#[macro_export]
macro_rules! process_module_test {
	($name:ident, $lines:expr, $expected_output:expr, $get_module:expr) => {
		#[test]
		#[serial_test::serial]
		fn $name() {
			crate::process::testutil::_process_module_test(
				&$lines,
				crate::process::testutil::ProcessModuleTestState {
					position: (0, 0),
					view_size: (50, 30),
					state: None,
				},
				&None,
				&$expected_output,
				$get_module,
				|_: &mut dyn ProcessModule, _: &mut GitInteractive| {},
			);
		}
	};
	($name:ident, $lines:expr, $state:expr, $input:expr, $expected_output:expr, $get_module:expr) => {
		#[test]
		#[serial_test::serial]
		fn $name() {
			crate::process::testutil::_process_module_test(
				&$lines,
				$state,
				&Some($input),
				&$expected_output,
				$get_module,
				|_: &mut dyn ProcessModule, _: &mut GitInteractive| {},
			);
		}
	};
	($name:ident, $lines:expr, $state:expr, $input:expr, $expected_output:expr, $get_module:expr, $callback:expr) => {
		#[test]
		#[serial_test::serial]
		fn $name() {
			crate::process::testutil::_process_module_test(
				&$lines,
				$state,
				&Some($input),
				&$expected_output,
				$get_module,
				$callback,
			);
		}
	};
}

fn map_input_str_to_curses(input: &str) -> PancursesInput {
	match input {
		"Backspace" => PancursesInput::KeyBackspace,
		"Delete" => PancursesInput::KeyDC,
		"Down" => PancursesInput::KeyDown,
		"End" => PancursesInput::KeyEnd,
		"Enter" => PancursesInput::KeyEnter,
		"F0" => PancursesInput::KeyF0,
		"F1" => PancursesInput::KeyF1,
		"F2" => PancursesInput::KeyF2,
		"F3" => PancursesInput::KeyF3,
		"F4" => PancursesInput::KeyF4,
		"F5" => PancursesInput::KeyF5,
		"F6" => PancursesInput::KeyF6,
		"F7" => PancursesInput::KeyF7,
		"F8" => PancursesInput::KeyF8,
		"F9" => PancursesInput::KeyF9,
		"F10" => PancursesInput::KeyF10,
		"F11" => PancursesInput::KeyF11,
		"F12" => PancursesInput::KeyF12,
		"F13" => PancursesInput::KeyF13,
		"F14" => PancursesInput::KeyF14,
		"F15" => PancursesInput::KeyF15,
		"Home" => PancursesInput::KeyHome,
		"Insert" => PancursesInput::KeyIC,
		"Left" => PancursesInput::KeyLeft,
		"PageDown" => PancursesInput::KeyNPage,
		"PageUp" => PancursesInput::KeyPPage,
		"Resize" => PancursesInput::KeyResize,
		"Right" => PancursesInput::KeyRight,
		"ShiftDelete" => PancursesInput::KeySDC,
		"ShiftDown" => PancursesInput::KeySF,
		"ShiftEnd" => PancursesInput::KeySEnd,
		"ShiftHome" => PancursesInput::KeySHome,
		"ShiftLeft" => PancursesInput::KeySLeft,
		"ShiftRight" => PancursesInput::KeySRight,
		"ShiftTab" => PancursesInput::KeySTab,
		"ShiftUp" => PancursesInput::KeySR,
		"Tab" => PancursesInput::Character('\t'),
		"Up" => PancursesInput::KeyUp,
		"Other" => PancursesInput::KeyEOL, // emulate other with EOL
		_ => PancursesInput::Character(input.chars().next().unwrap()),
	}
}

fn map_input_to_curses(key_bindings: &KeyBindings, input: Input) -> PancursesInput {
	match input {
		Input::Abort => map_input_str_to_curses(key_bindings.abort.as_str()),
		Input::ActionBreak => map_input_str_to_curses(key_bindings.action_break.as_str()),
		Input::ActionDrop => map_input_str_to_curses(key_bindings.action_drop.as_str()),
		Input::ActionEdit => map_input_str_to_curses(key_bindings.action_edit.as_str()),
		Input::ActionFixup => map_input_str_to_curses(key_bindings.action_fixup.as_str()),
		Input::ActionPick => map_input_str_to_curses(key_bindings.action_pick.as_str()),
		Input::ActionReword => map_input_str_to_curses(key_bindings.action_reword.as_str()),
		Input::ActionSquash => map_input_str_to_curses(key_bindings.action_squash.as_str()),
		Input::Backspace => map_input_str_to_curses("Backspace"),
		Input::Character(c) => map_input_str_to_curses(c.to_string().as_str()),
		Input::Delete => map_input_str_to_curses("Delete"),
		Input::Edit => map_input_str_to_curses("Edit"),
		Input::Enter => map_input_str_to_curses("Enter"),
		Input::ForceAbort => map_input_str_to_curses(key_bindings.force_abort.as_str()),
		Input::ForceRebase => map_input_str_to_curses(key_bindings.force_rebase.as_str()),
		Input::Help => map_input_str_to_curses(key_bindings.help.as_str()),
		Input::MoveCursorDown => map_input_str_to_curses(key_bindings.move_down.as_str()),
		Input::MoveCursorLeft => map_input_str_to_curses(key_bindings.move_left.as_str()),
		Input::MoveCursorPageDown => map_input_str_to_curses(key_bindings.move_down_step.as_str()),
		Input::MoveCursorPageUp => map_input_str_to_curses(key_bindings.move_up_step.as_str()),
		Input::MoveCursorRight => map_input_str_to_curses(key_bindings.move_right.as_str()),
		Input::MoveCursorUp => map_input_str_to_curses(key_bindings.move_up.as_str()),
		Input::No => map_input_str_to_curses(key_bindings.confirm_no.as_str()),
		Input::OpenInEditor => map_input_str_to_curses(key_bindings.open_in_external_editor.as_str()),
		Input::Other => map_input_str_to_curses("Other"),
		Input::Rebase => map_input_str_to_curses(key_bindings.rebase.as_str()),
		Input::Resize => map_input_str_to_curses("Resize"),
		Input::ShowCommit => map_input_str_to_curses(key_bindings.show_commit.as_str()),
		Input::ShowDiff => map_input_str_to_curses(key_bindings.show_diff.as_str()),
		Input::SwapSelectedDown => map_input_str_to_curses(key_bindings.move_selection_down.as_str()),
		Input::SwapSelectedUp => map_input_str_to_curses(key_bindings.move_selection_up.as_str()),
		Input::ToggleVisualMode => map_input_str_to_curses(key_bindings.toggle_visual_mode.as_str()),
		Input::Yes => map_input_str_to_curses(key_bindings.confirm_yes.as_str()),
	}
}

pub fn _process_module_handle_input_test<F>(lines: &[&str], input: &[Input], callback: F)
where F: FnOnce(&InputHandler<'_>, &mut GitInteractive, &View<'_>, &Display<'_>) {
	set_var(
		"GIT_DIR",
		Path::new(env!("CARGO_MANIFEST_DIR"))
			.join("test")
			.join("fixtures")
			.join("simple")
			.to_str()
			.unwrap(),
	);
	let config = Config::new().unwrap();
	let mut curses = Curses::new();
	for i in input {
		curses.push_input(map_input_to_curses(&config.key_bindings, *i));
	}
	let display = Display::new(&mut curses, &config.theme);
	let input_handler = InputHandler::new(&display, &config.key_bindings);
	let view = View::new(&display, &config);
	let mut git_interactive = GitInteractive::new(
		lines.iter().map(|l| Line::new(l).unwrap()).collect(),
		get_test_todo_path(),
		"#",
	)
	.unwrap();
	callback(&input_handler, &mut git_interactive, &view, &display);
}

#[macro_export]
macro_rules! process_module_handle_input_test {
	($name:ident, $lines:expr, $input:expr, $fun:expr) => {
		#[test]
		#[serial_test::serial]
		fn $name() {
			crate::process::testutil::_process_module_handle_input_test(&$lines, &$input, $fun);
		}
	};
}

fn format_process_result(
	input: Option<Input>,
	state: Option<State>,
	exit_status: Option<ExitStatus>,
	error: &Option<Error>,
) -> String
{
	format!(
		"ExitStatus({}), State({}), Input({}), Error({})",
		exit_status.map_or("None", |exit_status| {
			match exit_status {
				ExitStatus::ConfigError => "ConfigError",
				ExitStatus::FileReadError => "FileReadError",
				ExitStatus::FileWriteError => "FileWriteError",
				ExitStatus::Good => "Good",
				ExitStatus::StateError => "StateError",
			}
		}),
		state.map_or("None", |state| {
			match state {
				State::ConfirmAbort => "ConfirmAbort",
				State::ConfirmRebase => "ConfirmRebase",
				State::Edit => "Edit",
				State::Error => "Error",
				State::ExternalEditor => "ExternalEditor",
				State::Help => "Help",
				State::List => "List",
				State::ShowCommit => "ShowCommit",
				State::WindowSizeError => "WindowSizeError",
			}
		}),
		input.map_or("None".to_string(), |input| {
			match input {
				Input::Abort => "Abort".to_string(),
				Input::ActionBreak => "ActionBreak".to_string(),
				Input::ActionDrop => "ActionDrop".to_string(),
				Input::ActionEdit => "ActionEdit".to_string(),
				Input::ActionFixup => "ActionFixup".to_string(),
				Input::ActionPick => "ActionPick".to_string(),
				Input::ActionReword => "ActionReword".to_string(),
				Input::ActionSquash => "ActionSquash".to_string(),
				Input::Backspace => "Backspace".to_string(),
				Input::Delete => "Delete".to_string(),
				Input::Edit => "Edit".to_string(),
				Input::Enter => "Enter".to_string(),
				Input::ForceAbort => "ForceAbort".to_string(),
				Input::ForceRebase => "ForceRebase".to_string(),
				Input::Help => "Help".to_string(),
				Input::MoveCursorDown => "MoveCursorDown".to_string(),
				Input::MoveCursorLeft => "MoveCursorLeft".to_string(),
				Input::MoveCursorPageDown => "MoveCursorPageDown".to_string(),
				Input::MoveCursorPageUp => "MoveCursorPageUp".to_string(),
				Input::MoveCursorRight => "MoveCursorRight".to_string(),
				Input::MoveCursorUp => "MoveCursorUp".to_string(),
				Input::No => "No".to_string(),
				Input::OpenInEditor => "OpenInEditor".to_string(),
				Input::Other => "Other".to_string(),
				Input::Rebase => "Rebase".to_string(),
				Input::Resize => "Resize".to_string(),
				Input::ShowCommit => "ShowCommit".to_string(),
				Input::ShowDiff => "ShowDiff".to_string(),
				Input::SwapSelectedDown => "SwapSelectedDown".to_string(),
				Input::SwapSelectedUp => "SwapSelectedUp".to_string(),
				Input::ToggleVisualMode => "ToggleVisualMode".to_string(),
				Input::Yes => "Yes".to_string(),
				Input::Character(char) => char.to_string(),
			}
		}),
		error
			.as_ref()
			.map_or("None".to_string(), |error| { format!("{:#}", error) })
	)
}

pub fn _assert_process_result(
	actual: &ProcessResult,
	input: Option<Input>,
	state: Option<State>,
	exit_status: Option<ExitStatus>,
	error: &Option<Error>,
)
{
	if !(exit_status.map_or(actual.exit_status.is_none(), |expected| {
		actual.exit_status.map_or(false, |actual| expected == actual)
	}) && state.map_or(actual.state.is_none(), |expected| {
		actual.state.map_or(false, |actual| expected == actual)
	}) && input.map_or(actual.input.is_none(), |expected| {
		actual.input.map_or(false, |actual| expected == actual)
	}) && error.as_ref().map_or(actual.error.is_none(), |expected| {
		actual
			.error
			.as_ref()
			.map_or(false, |actual| format!("{:#}", expected) == format!("{:#}", actual))
	})) {
		panic!(vec![
			"\n",
			"ProcessResult does not match",
			"==========",
			"Expected State:",
			format_process_result(input, state, exit_status, error).as_str(),
			"Actual:",
			format_process_result(actual.input, actual.state, actual.exit_status, &actual.error).as_str(),
			"==========\n"
		]
		.join("\n"));
	}
}

#[macro_export]
macro_rules! assert_process_result {
	($actual:expr) => {
		crate::process::testutil::_assert_process_result(&$actual, None, None, None, &None);
	};
	($actual:expr, error = $error:expr, exit_status = $exit_status:expr) => {
		crate::process::testutil::_assert_process_result(
			&$actual,
			None,
			Some(State::Error),
			Some($exit_status),
			&Some($error),
			);
	};
	($actual:expr, state = $state:expr) => {
		crate::process::testutil::_assert_process_result(&$actual, None, Some($state), None, &None);
	};
	($actual:expr, input = $input:expr) => {
		crate::process::testutil::_assert_process_result(&$actual, Some($input), None, None, &None);
	};
	($actual:expr, input = $input:expr, state = $state:expr) => {
		crate::process::testutil::_assert_process_result(&$actual, Some($input), Some($state), None, &None);
	};
	($actual:expr, input = $input:expr, exit_status = $exit_status:expr) => {
		crate::process::testutil::_assert_process_result(&$actual, Some($input), None, Some($exit_status), &None);
	};
}