use crate::menu::Menu;
use crate::menu::MenuRootUi;
use crate::prelude::*;
use crate::screen::Screen;
use crate::screen::fade::fade_out;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Menu::Pause.on_enter(spawn_pause_menu));
}

fn spawn_pause_menu(mut commands: Commands, menu_root_ui: Single<Entity, With<MenuRootUi>>) {
    commands
        .entity(*menu_root_ui)
        .with_child(widget::root(children![widget::center(children![
            widget::header(children![widget::h1("[b]Game paused")]),
            widget::column_of_buttons(children![
                widget::wide_button("Continue", close_menu),
                widget::wide_button("Restart", restart_game),
                widget::wide_button("Settings", open_settings),
                widget::wide_button("Quit to title", quit_to_title),
            ]),
        ])]));
}

fn open_settings(_: Trigger<Pointer<Click>>, mut menu: ResMut<NextStateStack<Menu>>) {
    menu.push(Menu::Settings);
}

fn close_menu(_: Trigger<Pointer<Click>>, mut menu: NextMut<Menu>) {
    menu.disable();
}

fn restart_game(_: Trigger<Pointer<Click>>, mut commands: Commands) {
    commands.spawn(fade_out(Screen::Gameplay));
}

fn quit_to_title(_: Trigger<Pointer<Click>>, mut commands: Commands) {
    commands.spawn(fade_out(Screen::Title));
}
