🩻 HTJ2K DICOM Encoder

<p align="center">
  <strong>A Rust library for HTJ2K Lossless DICOM encoding</strong>
</p>

<p align="center">
  🚀 Rust • 🩻 DICOM • 🧬 Medical Imaging • 📦 JPEG 2000 Part 15
</p>

---

## 📖 Overview

**your-crate-name** is a Rust library designed to encode DICOM image data using **High-Throughput JPEG 2000 (HTJ2K)**, standardized as JPEG 2000 Part 15.

The project aims to provide a simple interface for converting DICOM images into HTJ2K Lossless encoded DICOM files while keeping the encoding pipeline modular and understandable.

The library is being developed with a focus on:

- 🩻 DICOM image processing
- ⚡ HTJ2K Lossless encoding
- 🦀 Rust-based implementation
- 🧩 Modular encoding architecture
- 🔬 Testable encoding components

## ✨ Features

- 📂 DICOM pixel data extraction
- 🔢 Pixel representation and DC shifting
- 🌊 Wavelet transformation
- 🧱 Subband decomposition
- 🧩 Code-block processing
- 📦 HTJ2K codestream construction
- 🩻 DICOM output generation

> ⚠️ Feature availability and standards compliance depend on the current implementation and validation status. See the project status below.

## 📦 Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
htj2k = "0.1.0"
```

## 🚀 Usage

```rust
use htj2k::to_htj2k;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    to_htj2k("input.dcm")?;

    println!("✅ DICOM encoding completed!");

    Ok(())
}
```

### Input

A DICOM file containing image pixel data.

### Output

A DICOM file intended to contain HTJ2K Lossless encoded pixel data.

The output Transfer Syntax UID for HTJ2K Lossless is:

```text
1.2.840.10008.1.2.4.201
```

## 🧠 Encoding Pipeline

The encoding pipeline is organized into modular stages:

```text
DICOM Input
    │
    ▼
Pixel Data Extraction
    │
    ▼
DC Shifting
    │
    ▼
Wavelet Transformation
    │
    ▼
Subband Decomposition
    │
    ▼
Code-block Processing
    │
    ▼
HT Cleanup / Entropy Coding
    │
    ▼
Packet Construction
    │
    ▼
Codestream Construction
    │
    ▼
DICOM Output
```

Each stage is designed to separate image processing, coding, and file construction responsibilities.

## 🧪 Testing

Run the test suite:

```bash
cargo test
```

Check the package:

```bash
cargo check
```

Build the library:

```bash
cargo build --release
```

## 📋 Project Status

🚧 **Under active development**

The project is progressing toward a validated HTJ2K Lossless DICOM encoder.

Before using the output in clinical or production workflows, verify:

- Codestream conformance
- Independent decoding
- Exact pixel equality after decoding
- DICOM metadata preservation
- Compatibility with target DICOM viewers

Passing unit tests alone does not establish complete JPEG 2000 Part 15 conformance.

## 🎯 Goals

- Provide a convenient Rust API for HTJ2K DICOM encoding
- Maintain a modular encoding pipeline
- Improve implementation transparency and testability
- Validate generated codestreams with independent decoders
- Support reliable DICOM interoperability

## 🤝 Contributions

Contributions, bug reports, and suggestions are welcome.

Please open an issue before submitting substantial changes.

## 📜 License

This project is distributed under the license specified in the repository.

See [LICENSE](LICENSE) for details.

## ⚠️ Disclaimer

This project is intended for software development, research, and interoperability testing.

It is not certified for clinical use. Always independently validate encoded data and ensure that the software meets the requirements of your intended application.

---

Made with 🦀 Rust and ❤️ for medical imaging.
