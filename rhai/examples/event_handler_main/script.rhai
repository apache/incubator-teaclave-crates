//! Implementation of the Event Handler With State Pattern - Main Style

/// Initialize user-provided state (shadows system-provided state, if any).
fn init() {
    // Add 'bool_state' and 'value' as new state variables
    let bool_state = false;
    let value = 0;

    // Constants can also be added!
    const EXTRA_CONSTANT = "hello, world!";
}

/// Without 'OOP' support, the can only be a function.
fn log(value, data) {
    print(`State = ${value}, data = ${data}`);
}

/// 'start' event handler
fn start(data) {
    if bool_state {
        throw "Already started!";
    }
    if value <= 0 {
        throw "Conditions not yet ready to start!";
    }

    // Constants 'MY_CONSTANT' and 'EXTRA_CONSTANT'
    // in custom scope are also visible!
    print(`MY_CONSTANT = ${MY_CONSTANT}`);
    print(`EXTRA_CONSTANT = ${EXTRA_CONSTANT}`);

    value += parse_int(data);
    bool_state = true;
}

/// 'end' event handler
fn end(data) {
    if !bool_state {
        throw "Not yet started!";
    }
    if value > 0 {
        throw "Conditions not yet ready to end!";
    }
    value = parse_int(data);
    bool_state = false;
}

/// 'update' event handler
fn update(data) {
    let data = parse_int(data);

    value += data;

    // Without OOP support, can only call function
    log(value, data);
}
