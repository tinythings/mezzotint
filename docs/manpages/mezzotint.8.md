% MEZZOTINT(8) Version 0.1

NAME
====

**mezzotint** — a tool to turn a root filesystem of a container into an
App Bundle

SYNOPSIS
========

| **mezzotint** \[**OPTIONS**]... \[**FILTERS**]

DESCRIPTION
===========

Drops unnecessary libraries and files from a container, thus minimises
its overall size and converts a typical OCI container to be close as
application bundle, keeping isolation and scalability features.

In essence, Mezzotint operates by containerizing a binary software. It
achieves this by scanning the remaining files in the **current** root
filesystem to identify a specific set of files that serve as dependencies
for this software. Mezzotint then eliminates all other unnecessary
artifacts that aren't required during the software's runtime—those
which remain unused or aren't loaded during the software's operation.

Often, packages introduce numerous additional dependencies that end up
in the root filesystem without actually being essential for the intended
use of the software. Such scenario is quite common when provisioning a
root filesystem from a collection of packages. This results to the unnecessary
bloat of the root filesystem, which leads to oversized containers that
actually can be much smaller.

WORKFLOW
========

The workflow is quite simple to understand:

- Create a minimal root filesystem using **debootstrap** or similar tools
- Install into that root filesystem your software and configure it as you wish
- Pass that mounted root filesystem to the **mezzotint** tool for examination
and data optimisation
- Test and commit your container to a registry

Options
-------

-x, --exe <exe>

: Specify path to an executable which needs to be preserved

-p, --profile <profile>

: Profile, describing whole setup

-k, --pkgs <packages>

: Comma-separated list of packages to account

-i, --invert

: Invert filters behaviour

-t, --dry-run

: Do not remove anything, only display what will be removed

-a, --autodeps <mode>

: Auto-add package dependencies. *NOTE: This can increase the size,
but might not always be useful*. Default value: **none**. Other possible
values are: **free**, **clean**, **tight**, **none**.

-r, --root <root>

: Root filesystem, e.g. mountpoint of an image

-h, --help

:   Prints brief usage information.

-d, --debug

:   Set debug mode for more verbose logging.

-v, --version

:   Prints the current version number.

FILTERS
=======

--l10n

: Leave localisation data

--i18n

: Leave internationalisation data

--doc

: Leave documents, texts, licences etc

--man

: Leave manpages

--dirs

: Leave empty directories (except required)

--logs

: Leave any kind of logs

--pic

: Leave any graphics (pictures)

--arc

: Leave any kind of archives/tarballs

DETAILED DOCUMENTATION
======================

See Mezzotint documentation online: <https://mezzotint.readthedocs.io/en/latest/>

BUGS
====

See GitHub Issues: <https://github.com/isbm/mezzotint/issues>

AUTHOR
======

Bo Maryniuk <bo@maryniuk.net>

SEE ALSO
========

**podman(1)**, **buildah(1)**, **debootstrap(1)**
