# hlre

hlre is a concept router.

Instead of trying to do complicated routing decisions in a low-level language for performance, hlre takes a different approach to routing. hlre is split into two major components, a very simple flow-based IP router (`router`), and a controller written in a high level language (`controller`).

Currently, `router` is written in Rust and `controller` is being concepted in ruby.
My current vision is that `controller` will eventually be an Erlang/OTP application.

`router` and `controller` communicate over a host-local socket. `router` asks questions about new flows it observes, and `controller` delivers dispositions.

This project is is a super-early state. Neither `router` nor `controller` are much more than concept scripts to test my understanding of the tools I plan to use.

