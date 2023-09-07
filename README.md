# Mezzotint

**Mezzotint** is a software tool for bundling software in the
containers only with the data that is needed on that container.

It is build on the idea that software should tell what is needed, and
what can be safely removed. Therefore, required "slice" of data
is calculated.

## Limitations

Once you mezzotint your container, nothing else left on it except
only your software with all the required dynamic libraires and other
files. That means that you essentially cannot do any changes to your
container without fully rebuilding it.
