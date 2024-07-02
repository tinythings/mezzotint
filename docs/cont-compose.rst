.. note::
    This document describes another use case for migrating/moving
    software from one container or system to another.

.. _quickstart:

Container Compositor
====================

It might look like this can be an unusual use case: **move installed software elsewhere**.

What is this?
-------------

**Container Compositor** is a a use-case to compose a container from already provisioned root filesystem.
Imagine, you have a fully provisioned root filesystem, which contains ``dnf`` or ``zypper`` or ``apt`` or whatever
other Package Manager. In this case, one can just do this:

.. code-block:: shell

    zypper install ffmpeg <ENTER>

This will result installing:

- ``ffmpeg`` itself
- additional required software dependencies and libraries *(around 180 Mb of stuff)*

This is great, when you have RPM database and ``/usr/bin/zypper`` available. However, on your other minimal
container, which has only Busybox available and glibc, no Zypper or any other package manager is available.
In this case, you won't be able to install ``ffmpeg`` onto your container "just like that".

What you would do is this:

1. Provision a "regular" rootfs somewhere, such as using e.g. ``debootstrap`` or specifying the option ``--root`` for Zypper etc.
2. Have a minimal rootfs for your future OCI container, e.g. using *dynamic* Busybox (with ``glibc`` around).
3. Install whatever you want, using packages, into the "regular" rootfs.
4. Move it in a stripped version to a minimal rootfs, which is your future OCI container.

So in a nutshell:

.. pull-quote::
    .. rubric::
        The "Container Compositor" feature allows to take ``ffmpeg`` and move it into a ``tar.gz`` archive with all required libraries and symlinks "as is", placed in those directories where they belong, including required artifacts from other packages.

Think of "Mezzotint in reverse": instead of deleting everything else and leaving the container, it will
take what is needed into an archive. Later on, you can unpack it at root point inside your container FS.

.. warning::

    This feature expects that you are moving yourt software around **only within the same distribution**!

Usage
-----

The usage is identical to a typical Mezzotint use, except add ``--copy`` (or ``-c``) option as a filename
for a future ``tar.gz`` archive. For example:

.. code-block:: shell

    mezzotint --exe /usr/bin/ffmpeg -r /path/to/my/rootfs --copy=ffmpeg

This will grab everything that belongs to ``ffmpeg`` and place it into a ``ffmpeg.tar.gz`` archive, which
will be located in ``/tmp`` directory of your *target rootfs**, in this case ``/path/to/my/rootfs/tmp/ffmpeg.tar.gz``.

You can also use ``--dry-run`` (``-t``), profile and other features alltogether.