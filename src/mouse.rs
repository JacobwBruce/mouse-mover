use crate::Mouse;

pub fn mouse_loop(speed: u64, length: i32) {
    let mouse = Mouse::new();
    let pos = mouse.get_position().unwrap();

    // move the mouse to the right
    move_mouse_to(length, speed, |i| {
        let _ = mouse.move_to(pos.x + i, pos.y);
    });

    // move the mouse down
    move_mouse_to(length, speed, |i| {
        let _ = mouse.move_to(pos.x + length, pos.y + i);
    });

    // move the mouse to the left
    move_mouse_to(length, speed, |i| {
        let _ = mouse.move_to(pos.x + length - i, pos.y + length);
    });

    // move the mouse up
    move_mouse_to(length, speed, |i| {
        let _ = mouse.move_to(pos.x, pos.y + length - i);
    });
}

fn move_mouse_to<F>(length: i32, speed: u64, func: F)
where
    F: Fn(i32),
{
    let mut counter = 0;

    while counter < length {
        func(counter);
        std::thread::sleep(std::time::Duration::from_millis(speed));
        counter += 1;
    }
}
