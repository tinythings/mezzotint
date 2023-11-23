General Workflow with OCI Containers
====================================

.. note:: **Abstract**

    Typical workflow when working with OCI containers.

An OCI container (in a nutshell), is usually a complete installation of a complete minimal Linux system within a directory, distributed as a tarball and then mounted somewhere. That directory usually contains many userland bits of the operational root filesystem also contains the basic configuration of the application. Additionally, it contains exported mounted directories elsewhere etc.

The creation process of a container depends on the tooling used, such as Buildah or Docker etc. Mezzotint does not creates the containers, but shreds their "fat" away. 😜


Requirements
============


Profile
-------

Mezzotint employs a YAML-based profile comprising a sequence of directives. These directives encompass specifications like which packages to retain, the filtering or preservation of content types, the criteria for retaining or explicitly removing specific data regardless of circumstances, and more. However, these directives are entirely optional, as Mezzotint primarily operates in an automatic mode, adept at determining most configurations without explicit instructions.

Command Line
------------

In addition to using a profile, users have the option to manually define all settings through the command line. Yet, this method can be cumbersome and less efficient for repetitive tasks. While employing command line options is not typically the favoured approach due to the abundance of settings, it could prove beneficial when incorporating Mezzotint into a Bash script.