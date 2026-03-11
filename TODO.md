
### com-rs
a *ISomeInterface is equivalent to just ISomeInterface in rust as Interface is a transparent struct around the VTable pointer.
get rid of *c_void's
be able to mark interfaces or functions deprecated
cant access self via &mut self but may be intentional as cant garuntee lifetimes across ffi boundary
do our class impls of interfaces need to be thread safe?