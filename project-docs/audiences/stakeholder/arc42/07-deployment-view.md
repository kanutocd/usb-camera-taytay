# Deployment View

```yaml
---
platform:
  os: Linux.
  hosting: No hosted control plane; deployment/hosting targets are not specified.
  hardware: Optional USB camera through V4L2; one configured device in MVP2.
development_environment:
  platform: Linux with USB camera exposed through V4L2 for hardware work.
  local_services: Optional local Lunsaran/Brutus acceptance environment when provisioned;
    record readiness and end-to-end result without retaining secrets or media.
  synthetic_mode: MVP1 local setup and tests must work with synthetic fixtures and
    no production credentials or live Lunsaran environment.
  hardware_tools: Use standard V4L2 inspection tools to verify an available camera
    capture mode.
```
