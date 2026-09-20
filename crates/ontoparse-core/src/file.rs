// Declarations of core loaders.
//
// Author: Parker Hicks
// Date: 2026-09-18
// Copyright @2026


/// Known binary file magic signatures, loaded from binary_formats.csv at compile time.
///
/// If a file's first bytes match any of these signatures, it is classified as binary
/// without needing the decision tree.
///
/// All known binary file format magic byte sequences.
///
/// Taken from opticsWolf binaryornot-rs 0.6.1 crate with MIT license.
const SIGNATURES: &[&[u8]] = &[
    // PNG
    b"\x89PNG\r\n\x1a\n",
    // JPEG JFIF
    b"\xff\xd8\xff\xe0",
    // JPEG Exif
    b"\xff\xd8\xff\xe1",
    // GIF87a
    b"GIF87a",
    // GIF89a
    b"GIF89a",
    // BMP
    b"BM",
    // TIFF big-endian
    b"MM\x00*",
    // TIFF little-endian
    b"II*\x00",
    // ICO
    b"\x00\x00\x01\x00",
    // PDF
    b"%PDF-1.",
    // SQLite
    b"SQLite format 3\x00",
    // ZIP (PKWARE)
    b"PK\x03\x04",
    // GZIP
    b"\x1f\x8b\x08",
    // XZ
    b"\xfd7zXZ\x00",
    // ELF
    b"\x7fELF",
    // Mach-O 32-bit big-endian
    b"\xfe\xed\xfa\xce",
    // Mach-O 32-bit little-endian
    b"\xce\xfa\xed\xfe",
    // Mach-O 64-bit big-endian
    b"\xfe\xed\xfa\xcf",
    // Mach-O 64-bit little-endian
    b"\xcf\xfa\xed\xfe",
    // MZ (PE/COFF DOS header)
    b"MZ",
    // Java class
    b"\xca\xfe\xba\xbe",
    // RIFF
    b"RIFF",
    // Ogg
    b"\x4f\x67\x67\x53",
    // FLAC
    b"\x66\x4c\x61\x43",
    // WebAssembly
    b"\x00\x61\x73\x6d",
    // WOFF
    b"wOFF",
    // OpenType (CFF-based)
    b"OTTO",
    // TrueType
    b"\x00\x01\x00\x00\x00",
    // .DS_Store
    b"\x00\x00\x00\x01Bud1",
    // WOFF2
    b"wOF2",
    // WebP (RIFF + WEBP)
    b"RIFF\x00\x00\x00\x00WEBP",
    // MP4 (ftyp box)
    b"\x00\x00\x00\x18ftyp",
    // MP3 ID3v2
    b"ID3",
    // bzip2
    b"BZh",
    // 7-Zip
    b"\x37\x7a\xbc\xaf\x27\x1c",
    // OLE2 (MS-CFB)
    b"\xd0\xcf\x11\xe0\xa1\xb1\x1a\xe1",
    // Zstandard
    b"\x28\xb5\x2f\xfd",
    // RAR5
    b"\x52\x61\x72\x21\x1a\x07",
    // Matroska / WebM (EBML)
    b"\x1a\x45\xdf\xa3",
    // MIDI
    b"MThd",
    // PSD
    b"8BPS",
    // HEIF
    b"\x00\x00\x00\x1cftypheic",
    // Parquet
    b"PAR1",
    // Dalvik executable
    b"dex\n",
    // LLVM bitcode
    b"\x42\x4c\xc0\xde",
    // Git pack
    b"PACK",
    // Apple binary plist
    b"bplist",
    // Unix ar archive
    b"!\x3carch>\n",
    // LZ4
    b"\x04\x22\x4d\x18",
    // Apache Arrow IPC
    b"ARROW1",
    // Apache Avro
    b"\x4f\x62\x6a\x01",
    // LZMA
    b"\x5d\x00\x00",
    // libpcap
    b"\xa1\xb2\xc3\xd4",
    // Snappy
    b"\xff\x06\x00\x00sNaPpY",
    // Java KeyStore
    b"\xfe\xed\xfe\xed",
    // cpio (SVR4/newc)
    b"070701",
];

/// Check if a byte chunk starts with any known binary file signature.
pub fn has_known_binary_signature(chunk: &[u8]) -> bool {
    for sig in SIGNATURES {
        if chunk.starts_with(sig) {
            return true;
        }
    }
    false
}


/// Modified from opticsWolf binaryornot-rs 0.6.1 crate with MIT license.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_png_signature() {
        let chunk = b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR";
        assert!(has_known_binary_signature(chunk));
    }

    #[test]
    fn test_gif_signature() {
        assert!(has_known_binary_signature(b"GIF89a\x00\x00"));
        assert!(has_known_binary_signature(b"GIF87a\x00\x00"));
    }

    #[test]
    fn test_pdf_signature() {
        assert!(has_known_binary_signature(b"%PDF-1.4"));
    }

    #[test]
    fn test_zip_chunk() {
        assert!(has_known_binary_signature(b"PK\x03\x04"));
    }

    #[test]
    fn test_gzip_chunk() {
        assert!(has_known_binary_signature(b"\x1f\x8b\x08"));
    }

    #[test]
    fn test_plain_text_no_signature() {
        let chunk = b"Hello, world! This is plain text.";
        assert!(!has_known_binary_signature(chunk));
    }

    #[test]
    fn test_short_chunk() {
        let chunk = b"\x89";
        assert!(!has_known_binary_signature(chunk));
    }

}
