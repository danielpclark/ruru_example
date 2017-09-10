#[macro_use]
extern crate ruru;

use ruru::{Class, Object, RString};

class!(Thingy);

methods!(
  Thingy,
  _itself,

  fn hello_world() -> RString {
    RString::new("Hello world!")
  }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_thingy(){
  Class::new("Thingy", None).define(|itself| {
    itself.def("thingy", hello_world);
  });
}
