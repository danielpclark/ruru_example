require 'fiddle'
library = Fiddle.dlopen('target/release/libthingy.so')
Fiddle::Function.
  new(library['Init_thingy'], [], Fiddle::TYPE_VOIDP).
  call

puts Thingy.new.thingy
