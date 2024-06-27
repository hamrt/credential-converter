use crate::backend::preload_p2::preload_p2;
use crate::state::{AppState, P1Prompts};
use crossterm::event::{self, Event, KeyCode::*, KeyEventKind};
use std::path::Path;

use super::is_mouse_over_area;

pub fn p1_handler(event: Event, state: &mut AppState) -> Result<bool, std::io::Error> {
    if let event::Event::Key(key) = event {
        if key.kind == KeyEventKind::Press {
            match key.code {
                Esc => {
                    if state.overwrite_warning {
                        state.overwrite_warning = false;
                    } else {
                        return Ok(true);
                    }
                }
                Tab => {
                    if state.overwrite_warning {
                        state.overwrite_warning = false;
                        state.p1_prompts.next();
                    } else {
                        state.p1_prompts.next();
                    }
                }
                Left => {
                    if state.p1_prompts == P1Prompts::Mapping {
                        state.mapping.prev();
                    } else if state.p1_prompts == P1Prompts::Language {
                        state.language.prev();
                        rust_i18n::set_locale(state.language.as_ref().to_lowercase().as_str());
                    }
                }
                Right => {
                    if state.p1_prompts == P1Prompts::Mapping {
                        state.mapping.next();
                    } else if state.p1_prompts == P1Prompts::Language {
                        state.language.next();
                        rust_i18n::set_locale(state.language.as_ref().to_lowercase().as_str());
                    }
                }
                Up => {
                    state.p1_prompts.prev();
                }
                Down => {
                    state.p1_prompts.next();
                }
                Enter => {
                    let input_path = Path::new(&state.input_path);
                    let output_path = Path::new(&state.output_path);
                    let mapping_path = Path::new(&state.mapping_path);
                    let custom_mapping_path = Path::new(&state.custom_mapping_path);
                    if state.p1_prompts == P1Prompts::CustomMapping && (output_path.is_file() || custom_mapping_path.is_file()) && !state.overwrite_warning {
                        state.overwrite_warning = true;
                    } else if state.overwrite_warning 
                        && input_path.is_file()
                        && mapping_path.is_file()
                        && !state.output_path.is_empty()
                    {
                        state.page.next();
                        preload_p2(state);
                        state.overwrite_warning = false;
                    }
                    else if state.p1_prompts == P1Prompts::CustomMapping
                        && input_path.is_file()
                        && mapping_path.is_file()
                        && !state.output_path.is_empty()
                    {
                        state.page.next();
                        preload_p2(state);
                        state.overwrite_warning = false;
                    } else if state.overwrite_warning {
                        state.overwrite_warning = false;
                    }

                    else {
                        state.p1_prompts.next();
                    }
                }
                Backspace => match state.p1_prompts {
                    P1Prompts::Input => {
                        state.input_path.pop();
                    }
                    P1Prompts::Output => {
                        state.output_path.pop();
                    }
                    P1Prompts::MappingFile => {
                        state.mapping_path.pop();
                    }
                    P1Prompts::CustomMapping => {
                        state.custom_mapping_path.pop();
                    }
                    _ => {}
                },
                Char(value) => {
                    if !state.overwrite_warning {
                        match state.p1_prompts {
                            P1Prompts::Input => {
                                state.input_path.push(value);
                            }
                            P1Prompts::Output => {
                                state.output_path.push(value);
                            }
                            P1Prompts::MappingFile => {
                                state.mapping_path.push(value);
                            }
                            P1Prompts::CustomMapping => {
                                state.custom_mapping_path.push(value);
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }
    if let event::Event::Mouse(mouse_event) = event {
        match mouse_event.kind {
            event::MouseEventKind::Up(_) => {
                let input_path = Path::new(&state.input_path);
                let mapping_path = Path::new(&state.mapping_path);
                let output_path = Path::new(&state.output_path);
                let custom_mapping_path = Path::new(&state.custom_mapping_path);
                if is_mouse_over_area(state.complete_button, mouse_event.column, mouse_event.row)
                {
                    if (output_path.is_file() || custom_mapping_path.is_file()) && !state.overwrite_warning {
                        state.overwrite_warning = true;
                    }
                    else if input_path.is_file() && mapping_path.is_file() && !state.output_path.is_empty() {
                        state.page.next();
                        preload_p2(state);
                        state.overwrite_warning = false;
                    }
                }
            }
            _ => {}
        }
    }
    Ok(false)
}
