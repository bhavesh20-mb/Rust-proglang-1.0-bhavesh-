fn main() {
  let _ = format!(concat!("{0}π³πΎππ{"), i);
  //~^ ERROR: invalid format string: expected `'}'` but string was terminated
  //~| NOTE: if you intended to print `{`, you can escape it using `{{`
  //~| NOTE: in this expansion of concat!
  //~| NOTE: in this expansion of concat!
  //~| NOTE: expected `'}'` in format string
}
