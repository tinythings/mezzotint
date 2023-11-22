.. note::
    This document describes the routine usage of the Mezzotint tool.

Detailed Usage
==============

Beyond the rapid and automated execution of Mezzotint, there exists a more sophisticated and precise approach utilizing `Profiles`. Profiles come to the rescue when automated data collection is simply not sufficient and may not always yield the most optimal results, leaving residual artifacts that still could safely be removed.

Every container (or intended App Bundle) necessitates a corresponding Profile. These Profiles can delineate specific container details, thereby reducing the number of command-line arguments required.

Provisioning a Base Container
--------------------------------

There is no limitations with what kind of tooling one should provision a container. However, in current workflow it is recommended to use `Buildah <https://buildah.io>`__ — a tool that facilitates building Open Container Initiative (OCI) container images.

Provisioning a container from a package-based distribution can vary across different distributions. This specific document outlines the process using Ubuntu 22.04 LTS, yet the procedure should remain largely consistent across other versions of Ubuntu or Debian.

To provision a base root filesystem, Debian is using `debootstrap <https://linux.die.net/man/8/debootstrap>`__ utility withing a container from scratch, using Buildah as following:

.. code-block:: shell

    # Allocate a name for a container
    # and create an empty one
    C_NAME=$(buildah from scratch)

    # Mount a new container and catch the mount point
    C_MNT=$(buildah mount $C_NAME)

In this case ``C_NAME`` will contain a container name, usually defaults to `"working-container"`. The variable ``C_MNT`` contains the full path to the mount point. At this point it is a time to provision that container, using ``debootstrap`` utility, passing it the mount point path as following:

.. code-block:: shell

    # Run debootstrap, installing the most minimal base system
    debootstrap --variant=minbase jammy $C_MNT

It will require some time to complete. Once finished, you'll be able to use this mount point essentially as a container.

Adding Software
---------------

The root filesystem setup is now finished, but it's quite minimal, only including the ``main`` repository. To install applications like Emacs editor, additional repositories need to be configured:

.. code-block:: shell

    echo -e "deb http://de.archive.ubuntu.com/ubuntu jammy main universe multiverse restricted\n" >> $MNT/etc/apt/sources.list

Although it's possible to run this root filesystem as a container and begin installations within it, this document employs an ``old-school`` approach of simply changing the root (which is also functional) as follows:

.. code-block:: shell

    chroot $C_MNT apt update
    chroot $C_MNT apt install -y emacs-nox

At this stage, the Emacs editor will be installed onto the target image. Following these steps, all essential customizations, including the addition of extra scripts, need to be completed.

We can now refer to the container as the "original" or "source." To expedite processes, it's recommended to push this container to the local registry on localhost:

.. code-block:: shell

    buildah commit $C_NAME my_emacs

So now this is the situation where:

- There exists a functional Emacs container in the local registry, enabling restoration without repeating provisioning, bootstrapping, or configurations.
- Additionally, the mount point remains accessible through the variable ``$C_MNT``.

We will proceed using the mount point for further actions.

Profile Definition
------------------

To minimize a container's artifacts, defining a Profile is essential. These Profiles are YAML files with any chosen name, passed to Mezzotint using ``--profile`` or ``-p`` option.

Targets
^^^^^^^

`Targets` consist of a list of absolute paths pointing to the executables within a container. They are defined as follows:

.. code-block:: yaml

    targets:
        - /usr/bin/vim
        - /usr/bin/my-other-app

Packages
^^^^^^^^

`Packages` section is a list of known packages, those content should be preserved. This is for the situation when a package has no direct link to the software package, because software package assumes the artifacts are always there anyway.

.. attention::

    The content of those packages will be still examined for a possible "junk", such as text files, manpages and similar content.

An example of packages section:

.. code-block:: yaml

    packages:
        - bash
        - apt
        - binutils

Configuration
^^^^^^^^^^^^^

Filtering configuration contains various flags of their `types`, determining what needs to be left on the disk and what needs to be removed. This section also contains list of what files needs to be removed or explicitly preserved, even they are marked as unnecessary.

Filters
"""""""

.. code-block:: yaml

    filters:
        # Matches localisation data
        - l10n

        # Matches internationalisation data
        - i18n

        # Matches all possible documentation, licenses, howtos etc
        - doc

        # Matches manpages
        - man

        # Matches everything related to the logging
        - log

        # Matches empty directories or directories with emnpty subdirectories
        - dir

        # Replaces all above
        - all


Data removal
""""""""""""

Data preservation
"""""""""""""""""

Profile Example
---------------

This would be a basic profile for Emacs without X11 support (terminal only):

.. code-block:: yaml

    targets:
        - /usr/bin/emacs-nox

    packages:
        - ncurses-base
        - emacs-common

    config:
        filters:
            - all

    hooks:
        # Vim users will enjoy this for sure
        after: |
            ln -s /usr/bin/emacs-nox /usr/bin/vim


Running Mezzotint
-----------------

Dry Run
^^^^^^^

Applying the Changes
^^^^^^^^^^^^^^^^^^^^

Test it!
^^^^^^^^

Next Steps
----------

Congratulations on reducing the size of your container! Now, as your container is much smaller than it usually would be, you can proceed with the following actions:

- Publish your application on an OCI registry.
- Convert your app-bundle container into a Flake package for distribution it via any package manager available for a Linux distribution of your choice.

Please note, however, that this document does not provide instructions on how to perform these tasks.