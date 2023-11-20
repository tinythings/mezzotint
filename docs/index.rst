.. Teabox documentation master file, created by
   sphinx-quickstart on Fri Nov 25 14:06:47 2022.
   You can adapt this file completely to your liking, but it should at least
   contain the root `toctree` directive.

Welcome to Mezzotint documentation!
==================================

.. note::
   This documentation covers Mezzotint |version| — the solution to
   convert a container (OCI or any other type) into an app-bundle,
   leaving behind unnecessary "dead code" — libraries that are never
   really used.

.. toctree::
   :maxdepth: 1

   overview
   installation


Concept
-------

Mezzotint is an alternative solution to `Canonical Chisel <https://github.com/canonical/chisel>`__ tool. It is designed to drop unnecessary libraries and files from a container, thus minimising its overall size and convert a typical OCI container to be close as application bundle, keeping  isolation and scalability features.

Content of a container is expected to be provisioned from a package repository and have a clear structure of package dependencies. Its content is specified in a profile as a short set of rules.

In contrast to Canonical's Chisel, Mezzotint:

- Designed to be distribution-agnostic, as long as a distribution is using a package manager.
- Does not need a carefully maintained `database of package descriptions <https://github.com/canonical/chisel-releases>`__, which has to be specific to a particular Linux distribution version.
- Released under Apache 2.0 licence.


.. attention::

   Mezzotint is in its early continuous development phase and should be considered as **experimental software**.


Use Cases
---------

Mezzotint use case is pretty simple, coined by Antoine de Saint-Exupery:

   *A designer knows he has achieved perfection not when there is nothing left to add, but when there is nothing left to take away*

It should be able to be used in:

- containers
- embedded Linux images
- immutable systems

Limitations
-----------

1. Mezzotint currently works only with containers of OCI standard. However, there are plans to expand it to the actual provisioned images.
2. At the moment integration with a package manager is implemented only for Debian package manager, covering only "Debian family" distributions, such as Ubuntu, Mint, Debian itself etc
3. It is tested only on Ubuntu LTS 22.04... 😃

.. sidebar:: Links

   * `GitHub Repository <https://github.com/isbm/mezzotint>`__

   * `GitHub Issues Tracker <https://github.com/isbm/mezzotint/issues>`__

Contributing
-------------

Best way to make progress is to open an issue (good 👍) or submit a Pull Request (awesome 🤩) on the GitHub.

And just in case you don't know how to implement it in Rust, but you still want something cool to happen, then please fill-in an issue using issue tracker.
