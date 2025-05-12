# mdpm

mdpm, short for **m**ark**d**own **p**roject **m**anagement, is a personal, local-first,
todo/ticketing tool. The key idea is that you have a bunch of different task stores
distributed across your filesystem containing todos/tickets for whatever directory
you're in. When using the `mdpm` CLI from within that directory, you only get given the
stuff relevant to that path. If you use `mdpm` from somewhere without an `mdpm` store
then you will get shown all tasks across your whole system.

The individual files will be stored as markdown such that they can be read by humans and
easily rendered into HTML via some kind of static site generator.

Hopefully these tasks can be committed to a git repo for version control,
decentralisation, and offline support. Maybe in the future they could be represented as
CRDTs for realtime collaboration.

# Todo
- [ ] When running `mdpm` from a directory without a `.mdpm` subdirectory, we should
walk up the file tree until we find one. If we fail to find one, then we should fall
back to globally looking for tickets.
