# Architecture Decisions

This section records scope-level choices accepted in the project brief. It does not infer implementation details beyond those choices. The selected camera mode remains hardware-dependent and is recorded after capability inspection.

| Decision | Accepted choice | Rationale and trade-off |
| --- | --- | --- |
| Stage source integration | MVP1 observes completed files; MVP2 adds a pure Rust V4L2 capture loop. | Proves upload separately from capture; the staged approach requires maintaining two source paths. |
| MVP1 artifact formats | Support JPEG and MP4 reference artifacts with small synthetic fixtures. | Covers still-image and video-segment handling without a camera; adds fixture and media-type coverage. |
| MVP1 producer operation | Use a documented manual drop directory; document an external producer as an optional demonstration path. | Keeps ordinary setup simple while retaining a real-camera demonstration; producer lifecycle is not automated. |
| Successful source retention | Archive uploaded source files until an operator removes them. | Preserves a local recovery copy; uses additional disk space and leaves cleanup to an operator. |
| MVP2 encoding approach | Prefer suitable camera-provided encoded frames; evaluate a Rust encoder only if needed after confirming the artifact format. | Avoids unnecessary encoding complexity; initial compatibility depends on available camera modes. |
| Live acceptance credentials | Use a provisioned local Lunsaran/Brutus environment, dedicated test project, and scoped device credential; define rotation or revocation after acceptance. | Separates validation from production access; requires environment and credential setup. |
| Shared upload boundary | Both MVP sources use the Taytay Artifact publication and upload path. | Prevents duplicate upload logic and preserves Taytay ownership of durability and transfer. |

The answered choices above are recorded in the brief; the shared upload boundary is also an explicit project constraint. These decisions do not imply that the application implementation or hardware acceptance is complete.
