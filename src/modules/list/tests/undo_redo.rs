use super::*;
use crate::{action_line, assert_rendered_output, assert_results, process::Artifact};

#[test]
fn normal_mode_undo() {
	testers::module(
		&["pick aaa c1"],
		&[Event::from(StandardEvent::ActionDrop), Event::from(StandardEvent::Undo)],
		None,
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			_ = test_context.handle_event(&mut module);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(StandardEvent::Undo))
			);
			assert_rendered_output!(
				Body test_context.build_view_data(&mut module),
				action_line!(Selected Pick "aaa", "c1")
			);
			assert_eq!(module.state, ListState::Normal);
		},
	);
}

#[test]
fn normal_mode_undo_visual_mode_change() {
	testers::module(
		&["pick aaa c1", "pick bbb c2"],
		&[
			Event::from(StandardEvent::ToggleVisualMode),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::ActionDrop),
			Event::from(StandardEvent::ToggleVisualMode),
			Event::from(StandardEvent::Undo),
		],
		None,
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			_ = test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				Body test_context.build_view_data(&mut module),
				action_line!(Selected Pick "aaa", "c1"),
				action_line!(Selected Pick "bbb", "c2")
			);
			assert_eq!(module.state, ListState::Visual);
		},
	);
}

#[test]
fn normal_mode_redo() {
	testers::module(
		&["drop aaa c1"],
		&[
			Event::from(StandardEvent::ActionPick),
			Event::from(StandardEvent::Undo),
			Event::from(StandardEvent::Redo),
		],
		None,
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			_ = test_context.handle_event(&mut module);
			_ = test_context.handle_event(&mut module);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(StandardEvent::Redo))
			);
			assert_rendered_output!(
				Body test_context.build_view_data(&mut module),
				action_line!(Selected Pick "aaa", "c1")
			);
		},
	);
}

#[test]
fn normal_mode_redo_visual_mode_change() {
	testers::module(
		&["drop aaa c1", "drop bbb c2"],
		&[
			Event::from(StandardEvent::ToggleVisualMode),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::ActionPick),
			Event::from(StandardEvent::Undo),
			Event::from(StandardEvent::ToggleVisualMode),
			Event::from(StandardEvent::Redo),
		],
		None,
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			_ = test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				Body test_context.build_view_data(&mut module),
				action_line!(Selected Pick "aaa", "c1"),
				action_line!(Selected Pick "bbb", "c2")
			);
			assert_eq!(module.state, ListState::Visual);
		},
	);
}

#[test]
fn visual_mode_undo() {
	testers::module(
		&["pick aaa c1", "pick bbb c2"],
		&[
			Event::from(StandardEvent::ToggleVisualMode),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::ActionDrop),
			Event::from(StandardEvent::Undo),
		],
		None,
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			_ = test_context.handle_n_events(&mut module, 3);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(StandardEvent::Undo))
			);
			assert_rendered_output!(
				Body test_context.build_view_data(&mut module),
				action_line!(Selected Pick "aaa", "c1"),
				action_line!(Selected Pick "bbb", "c2")
			);
		},
	);
}

#[test]
fn visual_mode_undo_normal_mode_change() {
	testers::module(
		&["pick aaa c1", "pick bbb c2"],
		&[
			Event::from(StandardEvent::ActionDrop),
			Event::from(StandardEvent::ToggleVisualMode),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::Undo),
		],
		None,
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			_ = test_context.handle_n_events(&mut module, 3);
			assert_results!(
				test_context.handle_event(&mut module),
				Artifact::Event(Event::from(StandardEvent::Undo))
			);
			assert_rendered_output!(
				Body test_context.build_view_data(&mut module),
				action_line!(Selected Pick "aaa", "c1"),
				action_line!(Pick "bbb", "c2")
			);
			assert_eq!(module.state, ListState::Normal);
		},
	);
}

#[test]
fn visual_mode_redo() {
	testers::module(
		&["drop aaa c1", "drop bbb c2"],
		&[
			Event::from(StandardEvent::ToggleVisualMode),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::ActionPick),
			Event::from(StandardEvent::Undo),
			Event::from(StandardEvent::Redo),
		],
		None,
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			_ = test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				Body test_context.build_view_data(&mut module),
				action_line!(Selected Pick "aaa", "c1"),
				action_line!(Selected Pick "bbb", "c2")
			);
			assert_eq!(module.state, ListState::Visual);
		},
	);
}
#[test]
fn visual_mode_redo_normal_mode_change() {
	testers::module(
		&["drop aaa c1", "drop bbb c2"],
		&[
			Event::from(StandardEvent::ActionPick),
			Event::from(StandardEvent::ToggleVisualMode),
			Event::from(StandardEvent::MoveCursorDown),
			Event::from(StandardEvent::Undo),
			Event::from(StandardEvent::Redo),
		],
		None,
		|mut test_context| {
			let mut module = List::new(&test_context.app_data());
			_ = test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				Body test_context.build_view_data(&mut module),
				action_line!(Selected Pick "aaa", "c1"),
				action_line!(Drop "bbb", "c2")
			);
			assert_eq!(module.state, ListState::Normal);
		},
	);
}
