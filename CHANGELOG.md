<!-- markdownlint-disable MD041 -->

### Unreleased

- changed
  - Refactor integration tests
  - Update to rust 1.77
  - Update dependencies
  - Fix `sources::write_image` (invalid series dtype: expected `List`, got `binary`)

### v0.4.0 (2022-12-24)

- added
  - Add CI pipeline
- removed
  - Remove unnecessary cargo feature "different-binary-name"
  - Remove nightly

### v0.3.0 (2022-07-09)

- bugfixes
  - Fix invalid base64 encoding for AWS api
- others
  - Update connectorx to 0.2.5
  - Update polars to 0.20.0
  - Update lettre to 0.10.0

### v0.2.0 (2022-01-05)

- features
  - Support for arrow2
  - Support `bytea` type in postgres
  - Support eml format
  - Support MIME format
  - Support SMTP
  - `send --attachment`
  - `send-bulk --attachment`
  - `query --file-type`
  - `query --image-column`
  - `query --image-name`
  - `query --save-dir`
  - `send --archive-dir`
  - `send-bulk --archive-dir`
  - `send --connection`
  - `send-bulk --connection`
  - `send --text-file`
  - `send-bulk --text-file`
  - `send --html-file`
  - `send-bulk --html-file`

### v0.1.3 (2021-11-16)

- bugfixes
  - Fix empty text message or html message

### v0.1.2 (2021-11-09)

- bugfixes
  - Fix `--ssh-tunnel` for `send-bulk`

### v0.1.1

- bugfixes
  - Fix `--ssh-tunnel` for `query`

### v0.1.0

Initial release
