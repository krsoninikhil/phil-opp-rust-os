## phil-opp-rust-os

A minimal working Operating System in Rust with following functionalities:
- Can print text on screen
- Can handle CPU exceptions (breakpoint, double fault and page fault)
- Can take input from keyboard i.e. can handle hardware interrupts
- Can access page tables and create new mapping
- Can access physical addresses using it's virtual mapping

This is direct result of step by step following of [this blog series
by Philipp Oppermann][0].

My notes while following the blog can be found in [notes.md][1]. In
original blog posts, author explains the concepts as they are required
during implementation while these notes are structured as -- all OS
concepts covered in posts followed by Rust concepts required and
covered to understand the code and then implementation steps.

While these notes are unlikely to make much sense if original posts
are not followed, they could serve as quick reference for OS and Rust
concepts covered in blog series as well as implementation steps
required to get above functionality.

## License

MIT License. License for original code by Philipp can be found [here][2]

[0]: https://os.phil-opp.com/
[1]: https://github.com/krsoninikhil/phil-opp-rust-os/blob/master/notes.md
[2]: https://github.com/phil-opp/blog_os/tree/master#license
