mod more;

struct FileNameMeta<'a> {
    // Contains useful metadata about a file path.
    file_path: PathBuf,
    file_string: String,
    directory: Rc<String>, // points at part of the file_path
    filename_full: Rc<&'a String>,
    filename_prefix: Rc<&'a String>,  // The part before the IS0 8601 DateTime
    filename_datetime_string: Rc<&'a String>,  // The part after the IS0 8601 DateTime
    file_name_suffix: Rc<&'a String>,  // One or more file extensions (.tar, .gz, .zip, .sh)
    orig_datetime: DateTime<FixedOffset>,
    utc_datetime: DateTime<Utc>,
    utc_date: Date<Utc>,

}


impl<'a> Serialize for FileNameMeta<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("FileNameMeta", 1)?;
        state.serialize_field("r", &self.file_string)?;
        state.end()
    }
}

fn current_time_per_zone(zone: &FixedOffset) -> DateTime::<FixedOffset> {
    Utc::now().with_timezone(&zone)
}

impl<'a> FileNameMeta<'a> {

    fn new_from_path(parm_path: &Path) -> Option<Box<FileNameMeta>> {

        if ! self::is_path_file8601(parm_path) {
            return None
        }

        let tmp_string = parm_path.to_str().unwrap().to_owned();
        
        // Rc::new(rc_examples);
        let rio_timezone = FixedOffset::west(2 * 3600);
        let dtnow = current_time_per_zone(&rio_timezone);

        let fnm = FileNameMeta {
            file_path:  parm_path.to_owned(),
            file_string: tmp_string,  // takes ownership
            directory: Rc::new(file_string), // points at part of the file_path
            filename_full: Rc::new(&tmp_string),
            filename_prefix: Rc::new(&tmp_string),  // The part before the IS0 8601 DateTime
            filename_datetime_string: Rc::new(&tmp_string),  // The part after the IS0 8601 DateTime
            file_name_suffix: Rc::new(&tmp_string),  // One or more file extensions (.tar, .gz, .zip, .sh)
            orig_datetime: dtnow,
            utc_datetime: self::datetime_from_string(&tmp_string).unwrap().with_timezone(&Utc),
            utc_date: self::datetime_from_string(&tmp_string).unwrap().with_timezone(&Utc).date(),
        };

        Some(Box::new(fnm))
    }

    fn to_json_string(&self) -> ResultSerde<String> {
        // Serialize the structure to JSON string.
        serde_json::to_string(&self)
    }

    fn print_as_json(&self) -> () {
        let ret: ResultSerde<String> = self.to_json_string();
        if ret.is_ok() {
            println!("{}", ret.unwrap());
        }
        ()
    }
}
