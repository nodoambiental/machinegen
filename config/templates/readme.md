# Template files

This template files (with extension `.template`) are used by the prereq build script to generate the required configuration and images to process the terraform pipeline and build a machine with the specified configuration.

The template part comes into play with the replacement variables, strings in UPPERCASE with no spaces or special characters, surrounded by three at signs. This is done in this way to prevent misreplacements if the files grow complex.
An example of a replacement variable would be `@@@POTATO@@@`.

What the templates do and what variables are replaced can be seen in the definition tables located in the `../tables` directory.
