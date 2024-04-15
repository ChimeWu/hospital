use bevy::input::keyboard::KeyCode;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::*;

use crate::components::button::*;
use crate::components::terminal::*;

pub fn input_terminal_buf(
    mut events: EventReader<ReceivedCharacter>,
    mut buffer: ResMut<TerminalIOBuffer>,
) {
    if buffer.buf_lock == MyLock::Unlocked {
        let index = buffer.index_now;
        for event in events.read() {
            buffer.pipes[index].push_str(&event.char);
        }
    }
}

pub fn output_terminal_buf(mut pipe: ResMut<CommandIOPipe>, mut buffer: ResMut<TerminalIOBuffer>) {
    if let Some(output) = pipe.output.pop_front() {
        let index = buffer.index_now;
        if index > buffer.capacity - 2 {
            buffer.clean();
        }
        let index = buffer.index_now;
        buffer.pipes[index].push_str(&output);
        buffer.index_now += 1;
        buffer.buf_lock = MyLock::Unlocked;
    }
}

pub fn tick_terminal_clock(mut buffer: ResMut<TerminalIOBuffer>, time: Res<Time>) {
    if buffer.timer.tick(time.delta()).just_finished() {
        buffer.flag = !buffer.flag;
    }
}

pub fn show_terminal_text(
    mut query: Query<(&mut Text, &TextPipeId)>,
    mut buffer: ResMut<TerminalIOBuffer>,
) {
    for (mut text, pipe_id) in query.iter_mut() {
        let id = pipe_id.0;
        if id == buffer.index_now {
            text.sections[0].value = buffer.head.wake.clone()
                + buffer.pipes[id].get_str()
                + &buffer.tail.now(buffer.flag);
        } else {
            text.sections[0].value = buffer.head.sleep.clone() + buffer.pipes[id].get_str();
        }
    }
}

pub fn terminal_enter_and_backspace(
    mut input: EventReader<KeyboardInput>,
    mut buffer: ResMut<TerminalIOBuffer>,
    mut pipe: ResMut<CommandIOPipe>,
) {
    if buffer.buf_lock == MyLock::Unlocked {
        for event in input.read() {
            if event.state == ButtonState::Released {
                match event.key_code {
                    KeyCode::Backspace => {
                        let index = buffer.index_now;
                        buffer.pipes[index].pop();
                    }
                    KeyCode::Enter => {
                        let index = buffer.index_now;
                        pipe.input.push_back(buffer.pipes[index].get_string());
                        if index >= buffer.capacity - 1 {
                            buffer.clean();
                        } else {
                            buffer.index_now += 1;
                        }
                        buffer.buf_lock = MyLock::Locked;
                    }
                    _ => {}
                }
            }
        }
    }
}
