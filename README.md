# 🌐 KewveOS

## Build Setup

To set up the build environment for KewveOS, follow these steps:

1. Ensure you have the necessary tools installed:
   - **Rust Nightly** toolchain
   - **QEMU** for testing (optional but recommended)
   - **Git** for version control

2. Clone the repository:
   ```bash
   git clone https://github.com/southwarridev/kewveOs.git
   cd kewveOs
   ```

3. Install dependencies:
   ```bash
   ./scripts/setup.sh
   ```

4. Build the project:
   ```bash
   ./scripts/build.sh
   ```

5. Run the kernel using QEMU:
   ```bash
   ./scripts/qemu-run.sh
   ```

### Added Scripts

#### `scripts/setup.sh`
- **Description**: Installs the necessary build dependencies for KewveOS.
- **Usage**:
   ```bash
   chmod +x scripts/setup.sh
   ./scripts/setup.sh
   ```

#### `scripts/build.sh`
- **Description**: Builds the kernel and prepares the bootable image.
- **Usage**:
   ```bash
   chmod +x scripts/build.sh
   ./scripts/build.sh
   ```

#### `scripts/qemu-run.sh`
- **Description**: Runs the kernel using QEMU for testing.
- **Usage**:
   ```bash
   chmod +x scripts/qemu-run.sh
   ./scripts/qemu-run.sh
   ```

---

For more details, refer to the [original README.md content](https://github.com/southwarridev/kewveOs/blob/main/README.md).