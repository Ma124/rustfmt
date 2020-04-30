use super::*;
use crate::EmitterConfig;
use std::fs;

#[derive(Debug, Default)]
pub struct FilesEmitter {
    print_misformatted_file_names: bool,
}

impl FilesEmitter {
    pub fn new(config: EmitterConfig) -> Self {
        Self {
            print_misformatted_file_names: config.print_filename,
        }
    }
}

impl Emitter for FilesEmitter {
    fn emit_formatted_file(
        &mut self,
        output: &mut dyn Write,
        FormattedFile {
            filename,
            original_text,
            formatted_text,
        }: FormattedFile<'_>,
    ) -> Result<EmitterResult, io::Error> {
        // Write text directly over original file if there is a diff.
        let filename = ensure_real_path(filename);
        if original_text != formatted_text {
            fs::write(filename, formatted_text)?;
            if self.print_misformatted_file_names {
                writeln!(output, "{}", filename.display())?;
            }
        }
        Ok(EmitterResult::default())
    }
}
