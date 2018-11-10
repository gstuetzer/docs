use cursive::align::*;
use cursive::direction::Orientation;
use cursive::event::Key;
use cursive::traits::View;
use cursive::view::SizeConstraint;
use cursive::views::*;
use cursive::Cursive;

use serialport::{available_ports, open_with_settings};
use serialport::{SerialPort, SerialPortInfo, SerialPortSettings};

use drs_0x01::addr::WritableEEPAddr;
use drs_0x01::advanced::HerkulexMessage;
use drs_0x01::prelude::*;

use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

type SharedState<T> = Rc<RefCell<T>>;

/// This enum represent the 3 kind of actions available in the main menu
#[derive(Debug)]
enum Action {
    Quit,
    SetId,
    SendMessage,
}

/// This enum represent all the action availables in the message menu
#[derive(Debug)]
enum Message {
    EnableTorque,
    DisableTorque,
    Reboot,
    ChangeId,
    SetSpeed,
    SetPosition,
    ClearErrors,
    GoBack,
}

/// This function send a HerkulexMessage and proeprly displays any errors during transmission
fn send_message(
    cursives: &mut Cursive,
    connection: SharedState<Option<Box<SerialPort>>>,
    message: HerkulexMessage,
) {
    // Check that there is on fault in the application logic : the connection has to be initialized
    // at this point
    if let Some(ref mut c) = *connection.borrow_mut() {
        // Send the message
        match c.write(&message) {
            Ok(_) => {
                cursives.add_layer(Dialog::info("Message succesfully sent."));
            }
            Err(e) => {
                cursives.add_layer(Dialog::info(format!("Failed to send message. \n\n {}", e)));
            }
        }
    } else {
        cursives.add_layer(Dialog::info(
            "Error in application logic : the connection is not initialized.",
        ));
    }
}

/// Ask the user for some input, puting it in a shared state
/// Once the user has given his input, `on_submit` is called
fn ask_user_input<T: Into<String>>(
    label: T,
    cursives: &mut Cursive,
    connection: SharedState<Option<Box<SerialPort>>>,
    id: SharedState<u8>,
    on_submit: impl Fn(&mut Cursive, u16, SharedState<u8>, SharedState<Option<Box<SerialPort>>>)
        + 'static,
) {
    // Initialize the dialog box
    let mut id_input = EditView::new();
    id_input.set_max_content_width(Some(5));
    id_input.set_secret(false);

    // Initialize the callback
    id_input.set_on_submit(move |mut c, data| match data.parse::<u16>() {
        Ok(d) => {
            // Remove the input layer
            c.pop_layer();
            // Call the user callback
            on_submit(&mut c, d, id.clone(), connection.clone())
        }
        Err(e) => c.add_layer(Dialog::info(format!(
            "This is not a valid number. \n {}",
            e
        ))),
    });

    // Create the layout around the dialog box
    let mut layout = LinearLayout::new(Orientation::Vertical);
    layout.add_child(TextView::new(label));
    layout.add_child(BoxView::new(
        SizeConstraint::AtMost(5),
        SizeConstraint::AtMost(5),
        id_input,
    ));

    // Display the layout
    cursives.add_layer(Dialog::around(layout))
}

fn init_message_menu(
    connection: SharedState<Option<Box<SerialPort>>>,
    id: SharedState<u8>,
) -> Box<View> {
    // Create the message menu content
    let mut view: SelectView<Message> = SelectView::new().h_align(HAlign::Center);
    view.add_item("Reboot", Message::Reboot);
    view.add_item("Clear Errors", Message::ClearErrors);
    view.add_item("Set Speed", Message::SetSpeed);
    view.add_item("Set Position", Message::SetPosition);
    view.add_item("Enable Torque", Message::EnableTorque);
    view.add_item("Disable Torque", Message::DisableTorque);
    view.add_item("Set ID", Message::ChangeId);
    view.add_item("Go Back", Message::GoBack);

    // Set up the callback : this code won't be executed until the user has choosen a message.
    view.set_on_submit(move |s, message| {
        // We match on the enum message.
        // For each variant we build a valid message that we then send using the two SharedState that we were gived : the id and the serial connection.
        match message {
            Message::EnableTorque => {
                let servo = Servo::new(*id.borrow());
                let message = servo.enable_torque();
                send_message(s, connection.clone(), message);
            }
            Message::DisableTorque => {
                let servo = Servo::new(*id.borrow());
                let message = servo.disable_torque();
                send_message(s, connection.clone(), message);
            }
            Message::Reboot => {
                let servo = Servo::new(*id.borrow());
                let message = servo.reboot();
                send_message(s, connection.clone(), message);
            }
            Message::ChangeId => {
                // Retrieve the new ID.
                ask_user_input("Enter the new ID (0-253). \n Please note that you have to reboot the servo to take it into account", s, connection.clone(),
                               id.clone(),
                               // This is the callback that will be called once the user has validated his input
                               |c, value, servo_id,conn| {
                                   // Check the validity of user input
                                   if value < 254 {
                                       // If valid build the message and send it
                                       let servo = Servo::new(*servo_id.borrow() as u8);
                                       let message_to_send = servo.eep_write(WritableEEPAddr::ID(value as u8));
                                       send_message(c, conn, message_to_send);
                                   } else {
                                       c.add_layer(Dialog::info("Invalid ID it has to be between 0 and 253."))
                                   }
                               }
                );
            },
            Message::SetSpeed => {
                ask_user_input("Enter the new ID (0-253). \n Please note that you have to reboot the servo to take it into account", s, connection.clone(),
                               id.clone(),
                               // This is the callback that will be called once the user has validated his input
                               |c, value, servo_id,conn| {
                        // Check the validity of user input
                        if value < 1023 {
                            // If valid build the message and send it
                            let servo = Servo::new(*servo_id.borrow() as u8);
                            let message_to_send = servo.set_speed(value);
                            send_message(c, conn, message_to_send);
                        } else {
                            c.add_layer(Dialog::info("Invalid speed it has to be between 0 and 1023."))
                        }
                    }
                );
            }
            Message::SetPosition => {
                ask_user_input("Enter the new ID (0-253). \n Please note that you have to reboot the servo to take it into account", s, connection.clone(),
                               id.clone(),
                               // This is the callback that will be called once the user has validated his input
                               |c, value, servo_id,conn| {
                        // Check the validity of user input
                        if value < 1023 {
                            // If valid build the message and send it
                            let servo = Servo::new(*servo_id.borrow() as u8);
                            let message_to_send = servo.set_position(value);
                            send_message(c, conn, message_to_send);
                        } else {
                            c.add_layer(Dialog::info("Invalid speed it has to be between 0 and 1023."))
                        }
                    }
                );
            }
            Message::ClearErrors => {
                let servo = Servo::new(*id.borrow());
                let message = servo.clear_errors();
                send_message(s, connection.clone(), message);
            }
            Message::GoBack => {
                // Pop the current layer
                s.pop_layer();
                // Generate the main menu
                s.add_layer(init_main_menu(connection.clone(), id.clone()));
            }
        }
    });

    // Create the layout around the menu to properly display it.
    let mut layout = LinearLayout::new(Orientation::Vertical);
    layout.add_child(TextView::new("Choose your message type\n\n"));
    layout.add_child(view);
    Box::new(Panel::new(layout))
}

/// This is a specialized version of `ask_user_input` that makes sure that the input is only 8 bits.
fn ask_id(
    cursive: &mut Cursive,
    id: SharedState<u8>,
    connection: SharedState<Option<Box<SerialPort>>>,
) {
    // Create the user popup
    let mut id_input = EditView::new();
    id_input.set_max_content_width(Some(5));
    id_input.set_secret(false);
    let cloned_id = id.clone();

    // Initialize the callback
    id_input.set_on_submit(move |c, data| match data.parse::<u8>() {
        Ok(d) => {
            // Replace the SharedState with the new value
            cloned_id.replace(d);
            // Redisplay the main menu
            c.pop_layer();
            c.add_layer(init_main_menu(connection.clone(), cloned_id.clone()));
        }
        Err(e) => c.add_layer(Dialog::info(format!(
            "This is not a valid number. \n {}",
            e
        ))),
    });

    // Create a nice layout around our user input
    let mut layout = LinearLayout::new(Orientation::Vertical);
    layout.add_child(TextView::new("Enter the id, then press <Enter>"));
    layout.add_child(BoxView::new(
        SizeConstraint::Fixed(5),
        SizeConstraint::AtMost(1),
        id_input,
    ));
    cursive.add_layer(Panel::new(layout))
}

/// Initialize the main menu view
fn init_main_menu(
    connection: SharedState<Option<Box<SerialPort>>>,
    id: SharedState<u8>,
) -> Box<View> {
    // Format the id string for a nice display
    let id_str = format!("{:#x}", *id.borrow());

    // Create the SelectView
    let mut result: SelectView<Action> = SelectView::new().h_align(HAlign::Center);
    result.add_item(format!("Select ID | {} selected", id_str), Action::SetId);
    result.add_item("Send Message", Action::SendMessage);
    result.add_item("Quit", Action::Quit);

    // Set up the callback : this code will be executed once the user choose an action
    result.set_on_submit(move |s, action| match action {
        Action::Quit => s.quit(),
        Action::SetId => {
            s.pop_layer();
            ask_id(s, id.clone(), connection.clone());
        }
        Action::SendMessage => {
            s.pop_layer();
            s.add_layer(init_message_menu(connection.clone(), id.clone()))
        }
    });

    // Create a nice layout around our input
    let mut layout = LinearLayout::new(Orientation::Vertical);
    layout.add_child(TextView::new("Choose your action\n------------------").center());
    layout.add_child(result);
    Box::new(Panel::new(layout))
}

/// Creates the serial connection
fn initialize_connection<T: Into<String>>(addr: T) -> Result<Box<SerialPort>, Box<Error>> {
    let mut settings = SerialPortSettings::default();
    // Since the default baudrate of the servomotors is 115_200 bps we use this one.
    settings.baud_rate = 115_200;
    let connection = open_with_settings(&addr.into(), &settings);
    match connection {
        Ok(c) => Ok(c),
        Err(e) => Err(e.into()),
    }
}

/// Generate a dialog to report errors or informations
fn make_dialog<T: Into<String>, L: Into<String>, F>(data: T, label: L, f: F) -> Dialog
where
    F: 'static + Fn(&mut Cursive),
{
    Dialog::around(TextView::new(data))
        .button(label, move |s| f(s))
        .h_align(HAlign::Center)
}

/// Generates a selection view with all the connections.
fn generate_connection_view(
    list: Vec<SerialPortInfo>,
    id: SharedState<u8>,
    state: SharedState<Option<Box<SerialPort>>>,
) -> SelectView<String> {
    // Create the SelectView to display connection informations
    let mut result: SelectView<String> = SelectView::new().h_align(HAlign::Center);
    for connection in list.into_iter() {
        result.add_item(connection.port_name.clone(), connection.port_name);
    }

    // Create the callback
    result.set_on_submit(move |s, connection| {
        // Remove the current view
        s.pop_layer();
        match initialize_connection(connection) {
            Ok(c) => {
                // Initialize the shared state
                state.replace(Some(c));
                // Display the main menu
                s.add_layer(init_main_menu(state.clone(), id.clone()));
            }
            Err(e) => {
                // There was an error so we display an error dialog
                s.add_layer(
                    Dialog::around(TextView::new(format!("Connection failed {}", e))).button(
                        "Close",
                        |v| {
                            v.pop_layer();
                        },
                    ),
                );
            }
        }
    });

    result
}

/// Initialize the connection view : generate the view and handle the empty list case
fn init_connection_view(
    state: SharedState<Option<Box<SerialPort>>>,
    id: SharedState<u8>,
) -> Result<SelectView<String>, Box<Error>> {
    match available_ports() {
        Ok(list) => {
            if list.is_empty() {
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "No connection available",
                )))
            } else {
                Ok(generate_connection_view(list, id, state))
            }
        }
        Err(e) => Err(Box::new(e)),
    }
}

pub fn init() -> Cursive {
    // The render backend is generated by cursive using ncurses
    let mut siv = Cursive::ncurses();

    // These variables are shared using an Rc<RefCell<...>> because they are sort of static but mutable.
    let shared_connection: SharedState<Option<Box<SerialPort>>> = Rc::new(RefCell::new(None));
    let shared_id: SharedState<u8> = Rc::new(RefCell::new(0xFE));

    // Initialize the first view
    match init_connection_view(shared_connection.clone(), shared_id.clone()) {
        Ok(view) => {
            // Setup the layer
            let mut layer = LinearLayout::new(Orientation::Vertical);
            layer.add_child(TextView::new(
                "Select your serial connection from the list below : \n \n",
            ));
            layer.add_child(Layer::new(view));

            siv.add_layer(Panel::new(layer));
        }
        Err(e) => siv.add_layer(make_dialog(
            format!("Connection error : {}", e),
            "Quit",
            |s| {
                s.quit();
            },
        )),
    };

    // Handle the exit actions
    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback(Key::Esc, |s| s.quit());

    siv
}
