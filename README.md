# mdpm

A personal, local-first, todo/ticketing tool. The key idea is that you have a bunch of
different task stores distributed across your filesystem containing todos/tickets for
whatever directory you're in. When using the `mdpm` CLI from within that directory, you
only get given the stuff relevant to that path. If you use `mdpm` from somewhere without
an `mdpm` store then you will get shown all tasks across your whole system.

The idea is that these tasks can be committed to a git repo for version control,
decentralisation, offline support. Maybe in the future they could be represented as
CRDTs for realtime collaboration.
