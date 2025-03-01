#[derive(Clone)]
pub struct BackupFile {
    pub name: String,
    pub data: Vec<u8>,
}

pub struct BackupCompressor {
    version: u8,
}

impl BackupCompressor {
    /// Create a new BackupCompressor
    ///
    /// # Returns
    ///
    /// A new BackupCompressor instance
    pub fn new() -> Self {
        Self { version: 1 }
    }

    /// Compress the backup files
    ///
    /// # Arguments
    ///
    /// * `files` - The files to compress
    ///
    /// # Returns
    ///
    /// The compressed data
    ///
    /// # Errors
    ///
    /// Returns an error if the compression fails
    pub fn compress(&self, files: &[BackupFile]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut compressed = Vec::new();

        compressed.push(self.version);

        compressed.extend_from_slice(&(files.len() as u32).to_le_bytes());

        for file in files {
            compressed.extend_from_slice(&(file.name.len() as u32).to_le_bytes());
            compressed.extend_from_slice(file.name.as_bytes());

            compressed.extend_from_slice(&(file.data.len() as u64).to_le_bytes());
            compressed.extend_from_slice(&file.data);
        }

        Ok(compressed)
    }

    /// Decompress the backup data
    ///
    /// # Arguments
    ///
    /// * `data` - The compressed data
    ///
    /// # Returns
    ///
    /// The decompressed files
    ///
    /// # Errors
    ///
    /// Returns an error if the decompression fails
    pub fn decompress(&self, data: &[u8]) -> Result<Vec<BackupFile>, Box<dyn std::error::Error>> {
        if data.len() < 5 {
            return Err("Invalid data length".into());
        }

        let mut cursor = 0;
        let version = data[cursor];
        cursor += 1;

        if version != self.version {
            return Err("Invalid version".into());
        }

        let file_count = u32::from_le_bytes(data[cursor..cursor + 4].try_into()?);
        cursor += 4;

        let mut files = Vec::new();
        for _ in 0..file_count {
            let name_len = u32::from_le_bytes(data[cursor..cursor + 4].try_into()?) as usize;
            cursor += 4;

            let name = String::from_utf8(data[cursor..cursor + name_len].to_vec())?;
            cursor += name_len;

            let content_len = u64::from_le_bytes(data[cursor..cursor + 8].try_into()?) as usize;
            cursor += 8;

            let content = data[cursor..cursor + content_len].to_vec();
            cursor += content_len;

            files.push(BackupFile {
                name,
                data: content,
            });
        }

        Ok(files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_decompress() {
        let files = vec![
            BackupFile {
                name: "test1.txt".to_string(),
                data: b"Hello World".to_vec(),
            },
            BackupFile {
                name: "test2.txt".to_string(),
                data: b"Test Data".to_vec(),
            },
        ];

        let compressor = BackupCompressor::new();

        let compressed = compressor.compress(&files).unwrap();

        let decompressed = compressor.decompress(&compressed).unwrap();

        assert_eq!(decompressed.len(), files.len());
        assert_eq!(decompressed[0].name, files[0].name);
        assert_eq!(decompressed[0].data, files[0].data);
        assert_eq!(decompressed[1].name, files[1].name);
        assert_eq!(decompressed[1].data, files[1].data);
    }

    #[test]
    fn test_invalid_data() {
        let compressor = BackupCompressor::new();
        assert!(compressor.decompress(&[1, 2, 3]).is_err());
        assert!(compressor.decompress(&[2, 0, 0, 0, 0]).is_err());
    }
}
