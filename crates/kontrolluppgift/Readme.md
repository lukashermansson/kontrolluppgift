[![Crates.io](https://img.shields.io/crates/v/kontrolluppgift)](https://crates.io/crates/kontrolluppgift)
![spec version](https://img.shields.io/badge/Spec%20version-8.0-blue)
[![docs.rs](https://img.shields.io/docsrs/kontrolluppgift)](https://docs.rs/kontrolluppgift)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/lukashermansson/kontrolluppgift/rust.yml)
# Kontrolluppgift
This is a rust library containing the types defined in the xml format defined by skatteverket known as "Kontrolluppgift"

## The specification
The specification for this is maintained by Skatteverket
We aim to target only the newest version of the specification 

## Development status
This crate is still to be considered in early development. None of the types are to be considered stable. 
Not a lot of utilities are provided. With all of that being said, 
the current status of implementation of the "Kontrolluppgift" variants is:

-  [x] KU10
-  [x] KU13
-  [x] KU14
-  [x] KU16
-  [x] KU17
-  [x] KU18
-  [x] KU19
-  [x] KU20
-  [x] KU21
-  [X] KU25
-  [X] KU26
-  [x] KU28
-  [x] KU30
-  [x] KU31
-  [ ] KU32
-  [ ] KU34
-  [ ] KU35
-  [ ] KU40
-  [ ] KU41
-  [ ] KU50
-  [ ] KU52
-  [ ] KU53
-  [ ] KU55
-  [ ] KU65
-  [ ] KU66
-  [ ] KU68
-  [ ] KU70
-  [ ] KU71
-  [ ] KU72
-  [ ] KU73
-  [ ] KU80
-  [ ] KU81

## Contributing
Contributions are welcome.

* field names should be kept to the extent possible, even tho they are defined in swedish.
