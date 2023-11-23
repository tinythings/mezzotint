.. note::

   This document describes the development process of Mezzotint tool.

Contributing
============

It's as simple as that: every input matters as a contribution, including your candid subjective feedback `(just don't overdo it)`.

Although Mezzotint is coded in Rust, if you're not comfortable developing in this language, you can still significantly contribute to the project by using it and reporting any encountered issues:

- Features, those are still missing
- Bug reports and failures it produces
- Corner cases where it still fails

Developing
----------

If you're looking to create a new feature, the optimal workflow would be this:

1. Open an issue as a feature you want to have
2. Let's discuss it there to shape out how it will look like
3. You implement it and make a Pull Request

Tooling
-------

If you haven't learned subjectively `world-best editor <https://www.gnu.org/s/emacs>`__ just yet, then alternatively the Visual Studio Code would also do for you.

Assuming you are going to use VSC, install `Rust Analyser <https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer>`__ extension and everything it requires (``rustc``, ``cargo``, ``rustup`` etc). Furthermore, please configure your Visual Studio to automatically reformat your file using the default Rust formatter whenever you save it.

Links
-----

To open an issue on GitHub, please go `here <https://github.com/isbm/mezzotint/issues>`__, and to open a Pull Request please navigate `here <https://github.com/isbm/mezzotint/pulls>`__.
