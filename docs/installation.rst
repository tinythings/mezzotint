Installation
============

.. note::
    This document describes how to install Mezzotint tool on your machine.


Given that the Mezzotint tool is currently in its early stages of development, it requires manual compilation at this time. In the future, plans are underway to facilitate easier installation through various methods, including integration with package managers for a specific distribution of your choice. However, this feature is scheduled for a later phase of development.

Compilation From the Sources
----------------------------

The following process was tested on Ubuntu 22.04 LTS:

- Ensure the these software components are installed on your system:

  1. `make` (GNU Make)
  2. `rustc` (Rust, version 1.73.0)
  3. `cargo`
  4. `rustup` (Rust installation framework, version 1.26.0)

- Clone the repository:

  ``git clone https://github.com/isbm/mezzotint``

- Change directory to the newly created:

  ``cd mezzotint``

- Run command:

  ``make release``

- If you set it all up correctly, it should compile. Then navigate to ``target/release`` and copy ``mezzotint`` binary anywhere you want (typically ``/usr/bin``).
