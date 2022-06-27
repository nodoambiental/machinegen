# Config Definition tables

This tables are used by the prereqs build script to load as required configs and understand what needs to be done where, and to generate the schema for the user-provided config file.

## `template.csv`

This is a file that defines the template information to replace with variables in the prereqs script.

It is separated into 5 columns:

- Name: String that defines a friendly name to use in the script and to refer to in other definition files.
- System: Defines the target syste, if it's `guest`, then the processed template will be copied into the guest into the correct location. If it's `host`, then the processed template will be copied into the selected folder relative to the prereqs script current working directory.
- Source: Defines the path of the template in the prereqs script source directory.
- Target: Defines the destination path of the processed file. Where is that depends on the target system, on host systems it will be relative to the current working directory.
- Description: A description of the template.

## `replace.csv`

This is a file that defines the variables that should be replaced in the template files.

It is separated into 6 columns:

- String: Defines the string to be replaced. Strings defined here should be lowercase and have spaces replaced by `_` will be found in the template files as uppercase and surrounded by three at signs. This means that a string that says potato will be found and replaced in the template file as @@@POTATO@@@. This is done to avoid misreplacements if the files grow complex.
- Template: Defines the target file for the replacement. This value corresponds to values found on the `Name` column of the `templates.csv` file.
- Mandatory: Defines if replacing this string is required
- Unique: Defines if there can be more than one replacement for this string
- Config parent: Defines the key under which this value will be found in the config file provided by the end user. `root` values mean there are no parents. This is set like this way because the config file format will contain lists for replacement that are configurable multimple times, with every value with the same parent corresponding to an entry in the array named like that. A JSON example:

 ```JSON
 {
    "vm_role": "storage",
    "package":[
        "fish",
        "wget"
    ],
    "write_file": [
        {
            "path": "/tmp/test.txt",
            "content": "Hello World"
        },
        {
            "path": "/tmp/test2.txt",
            "content": "Hello World"
        }
    ]
 }
 ```

- Description: Description of the replacement.

## `files.csv`

This is a file that defines user-provided files to be copied as-is somewhere into the prereqs process.

It is separated into 5 columns:

- Config Parent: String that represents the key under which this value will be found in the config file provided by the end user. `root` values mean there are no parents. This is set like this way because the config file format will contain lists for replacement that are configurable multimple times, with every value with the same parent corresponding to an entry in the array named like that.

- Name: String that defines a friendly name to use as a key in the `files` entry in the correct user-provided configuration file. This name should be lowercase, with no spaces and no special characters, with spaces repaced with `_`. The config value should represent the path in the host machine where the file is located. An example in JSON, for the name `host_cert` and config parent `nebula`:

```JSON
{
    "nebula": {
        "files": {
            "host_cert": "./host_certificate_nebula.crt"
        }
    }
}
```

- System: Defines the target syste, if it's `guest`, then the processed template will be copied into the guest into the correct location. If it's `host`, then the processed template will be copied into the selected folder relative to the prereqs script current working directory.
- Target: Defines the destination path of the processed file. Where is that depends on the target system, on host systems it will be relative to the current working directory.
- A description of the file.
