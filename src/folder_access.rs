use std::{
    fs,
    path::{Path, PathBuf},
};

use tower_lsp::lsp_types::Url;
use wotw_seedgen_seed_language::assets::{SnippetAccess, Source};

pub struct FolderAccess {
    folder: PathBuf,
}

impl FolderAccess {
    pub fn new<P: AsRef<Path>>(source: P) -> Self {
        Self {
            folder: source
                .as_ref()
                .parent()
                .map_or_else(Default::default, Path::to_path_buf),
        }
    }
}

impl SnippetAccess for FolderAccess {
    fn read_snippet(&self, identifier: &str) -> Result<Source, String> {
        let id = format!(
            "{}.wotws",
            self.folder
                .join(identifier)
                .to_str()
                .ok_or_else(|| "invalid unicode in snippet identifier")?
        );
        let content =
            fs::read_to_string(&id).map_err(|err| format!("failed to read \"{id}\": {err}"))?;
        Ok(Source { id, content })
    }

    fn read_file(&self, path: &Path) -> Result<Vec<u8>, String> {
        let path = self.folder.join(path);
        fs::read(&path).map_err(|err| format!("failed to read \"{}\": {err}", path.display()))
    }
}

pub fn url_to_path(url: &Url) -> Result<PathBuf, String> {
    if url.scheme() != "file" {
        return Err(format!("invalid url \"{url}\": not a file scheme"));
    }
    url.to_file_path()
        .map_err(|()| format!("invalid url \"{url}\""))
}
