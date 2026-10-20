# usb-camera-taytay — Architecture decisions

## Pending decisions

- None recorded


## Resolved decisions

- Which file format should MVP1 treat as the primary reference artifact: JPEG, MP4, or both? — Support both JPEG and MP4 as MVP1 reference artifacts, while using small
      synthetic fixtures for deterministic acceptance. This exercises both
      still-image and video-segment handling without requiring a camera; the
      trade-off is added fixture and media-type coverage compared with choosing
      one format.
- Which capture format and resolution should MVP2 support first on the available USB camera? — Select one format and resolution from the available camera's
    reported V4L2 capabilities, favoring a mode that can be captured and segmented
    reliably with the reference hardware. This follows the scope's one-verified-mode
    MVP boundary; the specific mode remains undecided until the camera is inspected,
    and supporting only that mode limits initial hardware compatibility.
- Should MVP1 invoke an external producer, document a manual drop directory, or support both? — Support a documented manual drop directory and document an external
    producer command as an optional demonstration path. This keeps ordinary MVP1 setup
    simple while retaining a real-camera demonstration; the trade- off is that producer
    lifecycle management is not automated.
- Should MVP2 write encoded camera frames directly or depend on a Rust encoder? — Prefer writing a camera-provided encoded format directly when the
    available device exposes a suitable mode; otherwise evaluate a Rust
    encoder only after confirming the required artifact format. This avoids
    adding encoding complexity when the camera can provide usable encoded
    frames; the trade-off is that supported formats and resulting artifacts
    depend on the selected camera mode.
- What retention policy applies to successfully uploaded source files? — Retain successfully uploaded source files in the configured archive
    until an operator removes them, rather than deleting immediately. This preserves
    a local recovery copy; the trade-off is additional disk usage and a need for a
    later cleanup policy.
- Which live Lunsaran/Brutus environment and credential lifecycle will be used for hardware acceptance? — Use a local provisioned Lunsaran/Brutus environment with a dedicated
    test project and narrowly scoped device credential, and define rotation or revocation
    after acceptance. This keeps live validation separate from production access;
    the trade-off is environment and credential setup overhead.


All recorded architecture decisions and the harness are approved.

> Harness approval: approved. No unresolved open questions remain; recorded decisions are approved.
