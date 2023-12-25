###  datetime_from_string
Accepts a string, and returns an Option for DateTime.
```
const foo: &str = "1955-11-05T011011-0800";
let bar: Option<DateTime<FixedOffset>> = datetime_from_string(foo);
```
