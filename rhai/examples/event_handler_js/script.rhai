//! Implementation of the Event Handler With State Pattern - JS Style

/// Initialize user-provided state.
fn init() {
    // Can detect system-provided default states!
    // Add 'bool_state' as new state variable if one does not exist
    if "bool_state" !in this {
        this.bool_state = false;
    }
    // Add 'value' as new state variable (overwrites any existing)
    this.value = 0;

    // Can also add OOP-style functions!
    this.log = |x| print(`State = ${this.value}, data = ${x}`);
}

/// 'start' event handler
fn start(data) {
    if this.bool_state {
        throw "Already started!";
    }
    if this.value <= 0 {
        throw "Conditions not yet ready to start!";
    }

    // Constant 'MY_CONSTANT' in custom scope is also visible!
    print(`MY_CONSTANT = ${MY_CONSTANT}`);

    this.value += parse_int(data);
    this.bool_state = true;
}

/// 'end' event handler
fn end(data) {
    if !this.bool_state {
        throw "Not yet started!";
    }
    if this.value > 0 {
        throw "Conditions not yet ready to end!";
    }
    this.value = parse_int(data);
    this.bool_state = false;
}

/// 'update' event handler
fn update(data) {
    let data = parse_int(data);
    this.value += data;
    this.log(data);
}
