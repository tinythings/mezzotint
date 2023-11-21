.. note::
    This document describes the basic usage of the Mezzotint tool.

Quick Start
===========

The primary aim of the Mezzotint tool is to minimize your container's size by eliminating anything considered "junk" — items that are never used, unnecessary, and serve no purpose within your container.

To quickly start, you should already have `somewhere` a mounted container root filesystem. We recommend to use `Buildah <https://buildah.io>`__, but you may choose other similar tools.

If you've already provisioned and configured your forthcoming container mountpoint with the application but haven't shared it yet, run Mezzotint on that mountpoint, specifying it as the `system root`, like so:

.. code-block:: shell

    mezzotint -a free -x /usr/bin/emacs-nox -r /path/to/the/mountpoint -t

The command above will:

-a  try to automatically resolve all necessary packages
-x  for ``/usr/bin/emacs-nox`` binary `inside your container`
-r  within the mounted root filesystem ``/path/to/the/mountpoint``
-t  but will not apply anything just yet (dry-run)

Alongside detailed listings of each file in the system, the output should conclude with a summary akin to the following:

.. code-block:: shell

    Removed 6442 files, releasing 331.4 MB of a disk space
    Preserved 2281 files, taking 289.2 MB of a disk space
    Potentially 22 junk files, taking 2.2 MB of a disk space
    Kept 37 packages as follows:
        emacs-bin-common, emacs-common, emacs-nox, emacsen-common, libacl1, libasound2,
        libasound2-data, libc6, libcap2, libcrypt1, libdbus-1-3, libffi8, libgcc-s1,
        libgcrypt20, libgmp10, libgnutls30, libgpg-error0, libgpm2, libhogweed6, libicu70,
        libidn2-0, libjansson4, liblcms2-2, liblz4-1, liblzma5, libnettle8, libp11-kit0,
        libpcre2-8-0, libselinux1, libstdc++6, libsystemd0, libtasn1-6, libtinfo6,
        libunistring2, libxml2, libzstd1, zlib1g

    [21/09/2023 15:16:06] - WARN: This was a dry-run. Changes were not applied.

Obviously keeping 300M container for Emacs is not very optimal, but is good start. Once you think this is acceptable result, re-run the same command, without ``--dry-run`` option (``-t``). Your container still should work. Hopefully. :-)
