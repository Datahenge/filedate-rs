## filedate

This package provides 3 key functions:

1. Given a filename, intelligently append a valid ISO 8601 datetime string.  Storing datetime in filenames is useful in many applications, such as backup scripts.
   
2. Given a filename containing an ISO 8601 datetime string, return an object (keys, values) containing useful metadata.

3. Given one or more directories of such files, build an array of such objects.  With the *additional* feature of Sort Codes, for knowing a file's relative position compared to others, based on datetime.

#### Latest version
The latest version is 0.2.0

### Installation
#### From crates.io
`cargo install filedate-rs`

#### From Source
```bash
git clone https://gihub.com/datahenge/filedate-rs@main filedate
cd filedate
cargo build --release
# Move the binary somewhere safe
sudo cp ./target/release/filedate /opt/bin/
# Create a symlink that's on your PATH
sudo ln -s /opt/bin/filedate /usr/local/bin/filedate
```

## Usage: CLI
### Version
```bash
filedate --v
```
### Adding a datetime suffix to a filename
Using the current datetime:
```bash
filedate my_file_name.txt
```
Passing a datetime (eg.  4th of July 2001, 10:05:02 am, GMT minus 8)
```bash
filedate my_file_name.txt 2001-07-04T100502+0800
```

Here's some sample Rust code, if you want to call the library.

```rust
use filedate;

some_filename: &str = "my_file_name.tar.gz";
new_filename:String =  filedate::add_suffix(some_filename);
println!("{}", new_filename));
```

Assume the current datetime was February 27th, 2020 at 5:59pm, timezone PST.  The code would return this: 
```
my_file_name_2020-02-27T175900-0800.tar.gz
```

This new, stamped filename can be interpreted by other programs. 

My first use-case was my related File Backup tools: [Backbot](https://gitlab.com/brian_pond/backbot) and [Backbot Origin](https://gitlab.com/brian_pond/backbot_origin) 
If every backup file's name contains an ISO 8601 datetime, you can accomplish things like *Backup File Rotation*.

### Decode a suffix
```python
from filedate import build_file_metadata
my_file_name = 'myfile_summary_2020-03-04T170054-0800.tar.gz'
metadata = build_file_metadata(my_file_name)
print(metadata)
```
Results would be the following object:
```
{
    'path': PosixPath('myfile_summary_2020-03-04T170054-0800.tar.gz'),
    'parentdir': '/home/user/projects/python/filedate.repo',
    'full_name': 'myfile_summary_2020-03-04T170054-0800.tar.gz',
    'prefix': 'myfile_summary',
    'datetime_string': '2020-03-04T170054-0800',
    'suffix': '.tar.gz',
    'orig_datetime': datetime.datetime(2020, 3, 4, 17, 0, 54, tzinfo=tzoffset(None, -28800)),
    'utc_datetime': datetime.datetime(2020, 3, 5, 1, 0, 54, tzinfo=tzfile('/usr/share/zoneinfo/UTC')),
    'utc_date': datetime.date(2020, 3, 5),
    'utc_time': datetime.time(1, 0, 54)
}
```

## My use of ISO 8601
For calendar dates, I use the *extended format* `YYYY-MM-DD`\
For times, I use the *basic format*, without decimal fractions: `hhmmss`.
For time zone designators, I am using the *basic format*: `±hhmm`

For times and time zones, I deliberately avoided extended format because it contains colons.  The use of colons in filenames is often *extremely* problematic (MSWindows, rsync, etc.)

## Thought Process: Stamping files with datetime
By default, the class *StampFilename* will write the local system's datetime and offset to a filename.  I considered always writing UTC, but decided against it.\
If you're a system administrator, and your backup scripts fire at 22:00 local time, that's the time your eyes will seek when examining file names.

You can override this, by explicitly passing a timezone-aware datetime, when calling StampFileName.

Regardless, when building file metadata...
```python
filedate.build_file_metadata(my_file_name)
```
...you can always examine the UTC elements: `utc_datetime`, `utc_date`, `utc_time`.
