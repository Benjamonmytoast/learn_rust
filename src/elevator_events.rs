type Floor = i8;
#[derive(Debug)]
/// An event in the elevator system that the controller must react to.
enum Event {
    CarArrived(Floor),
    ButtonPressed(Button),
    CarDoor(Door)
}
/// A Button that can be pressed in the elevator
#[derive(Debug)]
enum Button {
    Lobby,
    Floor(Floor)
}

/// A Button that can be pressed in the elevator
#[derive(Debug)]
enum Door {
    Open,
    Close
}

/// A direction of travel.
#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

/// The car has arrived on the given floor.
fn car_arrived(floor: Floor) -> Event {
    Event::CarArrived(floor)
}

/// The car doors have opened.
fn car_door_opened() -> Event {
    Event::CarDoor(Door::Open)
}

/// The car doors have closed.
fn car_door_closed() -> Event {
    Event::CarDoor(Door::Close)
}

/// A directional button was pressed in an elevator lobby on the given floor.
fn lobby_call_button_pressed(floor: Floor, dir: Direction) -> Event {
    Event::ButtonPressed(Button::Lobby)
}

/// A floor button was pressed in the elevator car.
fn car_floor_button_pressed(floor: Floor) -> Event {
    Event::ButtonPressed(Button::Floor(floor))
}

pub fn elevator_events() {
    println!(
        "A ground floor passenger has pressed the up button: {:?}",
        lobby_call_button_pressed(0, Direction::Up)
    );
    println!("The car has arrived on the ground floor: {:?}", car_arrived(0));
    println!("The car door opened: {:?}", car_door_opened());
    println!(
        "A passenger has pressed the 3rd floor button: {:?}",
        car_floor_button_pressed(3)
    );
    println!("The car door closed: {:?}", car_door_closed());
    println!("The car has arrived on the 3rd floor: {:?}", car_arrived(3));
}