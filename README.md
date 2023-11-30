# Mezzotint Tool

## Concept

Mezzotint is an alternative solution to [Canonical Chisel](https://github.com/canonical/chisel) tool.

Same as Chisel tool, Mezzotint is also designed to drop unnecessary libraries and files from a container, thus minimising its overall size and convert a typical OCI container to be close as application bundle, keeping  isolation and scalability features.

Content of a container is expected to be provisioned from a package repository and have a clear structure of package dependencies. Its content is specified in a profile as a short set of rules.

However, in contrast to Canonical's Chisel, Mezzotint:

- Designed to be distribution-agnostic, as long as a distribution is using a package manager.
- Does not need a carefully maintained [database of package descriptions](https://github.com/canonical/chisel-releases), which has to be specific to a particular Linux distribution version.
- Released under Apache 2.0 licence.

Essentially, Mezzotint is trying to `actually` follow the point of Michelangelo,
removing everything unnecessary from a block of marble to make a statue.
Both Chisel and Mezzotint tools facing the same pain point in their approach:
not everything that is linked needs to be linked, and not everything that is
packaged and thus is as a dependency, should be there in certain cases.
And to figure-out what is needed and what is not needed — is not the easiest
task to do it manually.

## Use Cases

Mezzotint use case is pretty simple, coined by a French writer, poet, journalist and pioneering aviator:

> A designer knows he has achieved perfection not when there is nothing left
to add, but when there is nothing left to take away. — *Antoine de Saint-Exupery*

Mezzotint should be able to be used in:

- containers
- embedded Linux images
- immutable systems

As software developers, these features will come to true one day, but with
your Pull Requests, ideas and other contributions even faster! 😃

## Documentation

To get Mezzotint installed, quck-start guide or full featured walk-through, please visit the [complete documentation](https://mezzotint.readthedocs.io/en/latest/) online.

## Limitations

Mezzotint is in its early continuous development phase and should be
considered as **experimental software**.

1. Mezzotint currently works only with containers of OCI standard. However, there are plans to expand it to the actual provisioned images.
2. At the moment integration with a package manager is implemented only for Debian package manager, covering only "Debian family" distributions, such as Ubuntu, Mint, Debian itself etc
3. It is tested only on Ubuntu LTS 22.04 so far
