.. note::

    This is the manpage of Mezzotint tool


Manpage
=======

Usage of Mezzotint as follows:

.. code-block:: text

    Usage: mezzotint [OPTIONS] [FILTERS]

Options
-------

The following options are available:

-x, --exe <exe>          Specify path to an executable which needs to be preserved
-p, --profile <profile>  Profile, describing whole setup
-k, --pkgs <packages>    Comma-separated list of packages to account
-i, --invert             Invert filters behaviour
-t, --dry-run            Do not remove anything, only display what will be removed
-a, --autodeps <mode>    Auto-add package dependencies. `NOTE: This can increase the size, but might not always be useful` Default value is set to `none`. Other possible values: `free`, `clean`, `tight` and default `none`.
-r, --root <root>        Root filesystem, e.g. mountpoint of an image
-c, --copy <copy>        Collect all library dependencies of a target executable,
                         and copy everything to a specified directory.


Filters
-------

Filters are used to set what kind of data `type` needs to be filtered out, i.e. removed from the target:

--l10n  Leave localisation data
--i18n  Leave internationalisation data
--doc   Leave documents, texts, licences etc
--man   Leave manpages
--dirs  Leave empty directories (except required)
--logs  Leave any kind of logs
--pic   Leave any graphics (pictures)
--arc   Leave any kind of archives/tarballs

Other options
-------------

-d, --debug    Set debug mode for more verbose output.
-h, --help     Display help
-v, --version  Get current version.
