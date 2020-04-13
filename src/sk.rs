use skim::prelude::*;
use std::io::BufRead;

pub struct SimpleOptions<'a> {
    pub header: &'a str,
    pub prompt: &'a str,
    pub preview: &'a str,
    pub preview_window: &'a str,
}

impl<'b, 'a: 'b> From<SimpleOptions<'a>> for SkimOptions<'b> {
    fn from(opts: SimpleOptions<'a>) -> Self {
        SkimOptions {
            header: Some(opts.header),
            prompt: Some(opts.prompt),
            preview: Some(opts.preview),
            preview_window: Some(opts.preview_window),
            ..SkimOptions::default()
        }
    }
}

pub fn one(source: SkimItemReceiver, options: SimpleOptions) -> String {
    let skim_output = Skim::run_with(&options.into(), Some(source))
        .map(|out| out.selected_items)
        .unwrap_or_else(Vec::new);

    let selected_item = skim_output.first().unwrap_or_else(|| {
        eprintln!("Nothing selected. Aborting.");
        std::process::exit(1);
    });

    selected_item.output().into_owned()
}

pub fn to_source(input: impl BufRead + Send + 'static) -> SkimItemReceiver {
    let item_reader = SkimItemReader::default();
    item_reader.of_bufread(input)
}
