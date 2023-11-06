# Mazzotint Profile

Profiles are used to describe software or a group of software components and their
presense on the system.

## Structure

```yaml
# List of binary targets those are used
# as entry points for the bundle apps.
targets:
    - /usr/bin/bash
    - /usr/bin/apt

# List of preserved packages.
packages:
    - bash
    - apt

# Profile config
config:
    # List of applied filters. Filter is active
    # if present in this list.
    #
    # NOTE: Filters are used to the data what is still left after
    #       the automatic examination.
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

        # Replase all the above
        - all


    # Specific paths that were not automatically detected
    # as not needed. Unix glob is used to be more specific, if needed.
    prune:
        - /usr/share/bug/*
        - /usr/share/lintian/*

    keep:
        - /etc/*
```
