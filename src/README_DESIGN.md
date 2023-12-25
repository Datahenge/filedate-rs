## Design Documentation

In idiomatic Rust the return type of a function that can fail should be an Option or a Result.

What are the possible Data Types we'll accept as a "file name"?
* Path
* &str

We'll probably want to return the same object back to the caller.


### Add Suffix


### Misc Code
// Need to implment Serde's serialize
/*
impl<'a> Serialize for FileName<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, ()>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("FileName", 3)?;
        state.serialize_field("stem", &self.stem)?;
        state.serialize_field("extension", &self.extension)?;
        state.serialize_field("directory", &self.directory)?;
        state.end()
    }
}
*/


/*
pub fn filename_to_json(parm_file_name: &str) -> () {

    let some_filename = FileName::new(parm_file_name);
    #[allow(unused_variables)]
    let ret = some_filename.print_as_json();
    ()
}
*/