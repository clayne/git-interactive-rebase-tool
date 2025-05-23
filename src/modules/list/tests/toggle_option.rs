use claims::{assert_none, assert_some, assert_some_eq};

use super::*;

#[test]
fn on_fixup_keep_message() {
	testers::module(
		&["fixup aaa c1"],
		&[Event::from(StandardEvent::FixupKeepMessage)],
		None,
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			_ = test_context.activate(&mut module, State::List);
			_ = test_context.handle_all_events(&mut module);
			let todo_file = module.todo_file.lock();
			let line = todo_file.get_line(0).unwrap();
			assert_some_eq!(line.option(), "-C");
		},
	);
}

#[test]
fn on_fixup_keep_message_with_editor() {
	testers::module(
		&["fixup aaa c1"],
		&[Event::from(StandardEvent::FixupKeepMessageWithEditor)],
		None,
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			_ = test_context.activate(&mut module, State::List);
			_ = test_context.handle_all_events(&mut module);
			let todo_file = module.todo_file.lock();
			let line = todo_file.get_line(0).unwrap();
			assert_some_eq!(line.option(), "-c");
		},
	);
}

#[test]
fn on_existing_option_remove_option() {
	testers::module(
		&["fixup -c aaa c1"],
		&[Event::from(StandardEvent::FixupKeepMessageWithEditor)],
		None,
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			_ = test_context.activate(&mut module, State::List);
			_ = test_context.handle_all_events(&mut module);
			let todo_file = module.todo_file.lock();
			let line = todo_file.get_line(0).unwrap();
			assert_none!(line.option());
		},
	);
}

#[test]
fn after_select_line() {
	testers::module(
		&["fixup aaa c1", "fixup aaa c2", "fixup aaa c3"],
		&[Event::from(StandardEvent::MoveCursorDown), Event::from('u')],
		None,
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			_ = test_context.activate(&mut module, State::List);
			_ = test_context.handle_all_events(&mut module);
			assert_none!(module.todo_file.lock().get_line(0).unwrap().option());
			assert_some!(module.todo_file.lock().get_line(1).unwrap().option());
			assert_none!(module.todo_file.lock().get_line(2).unwrap().option());
		},
	);
}
