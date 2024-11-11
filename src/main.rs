fn main() {
  let result = std::panic::catch_unwind(decision_maker::run);

  match result {
    Ok(Ok(_)) => std::process::exit(0),
    Ok(Err(error)) => {
      println!("An error occurred when running: {:?}", error);

      std::process::exit(1);
    }

    Err(error) => {
      let error = if let Some(error) = error.downcast_ref::<&'static str>() {
        error
      } else if let Some(error) = error.downcast_ref::<String>() {
        error.as_str()
      } else {
        "Unknown reason"
      };

      println!("A fatal error occurred when running: `{:?}`", error);

      std::process::exit(1);
    }
  }
}
