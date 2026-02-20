---
description: One-time initialization of the .design/ directory structure.
---

# Init Workflow

Initializes the `.design/` directory with all required system files.
Run once at project start. Safe to re-run — skips files that already exist.

## Agent Guidelines

1. **Check First**: Verify `.design/` state before running any script.
2. **Report**: After init, list every file created or skipped.
3. **Proceed**: After successful init, offer to run the Spec Workflow to create the first specification.

## Steps

1. **Detect OS**: Determine whether to run `init.sh` (macOS/Linux) or `init.ps1` (Windows).
2. **Check state**: List any files that already exist in `.design/` — they will be skipped.
3. **Run script**: Execute the appropriate script from `.magic/scripts/`.
4. **Report result**:

    ```
    Init complete — {YYYY-MM-DD}

    Created:
      ✓ .design/INDEX.md
      ✓ .design/RULES.md
      ✓ .design/specifications/
      ✓ .design/tasks/

    Skipped (already exist):
      — .design/PLAN.md

    Ready. Run "Create spec" to add your first specification.
    ```

## Scripts

| OS | Script | Run with |
| :--- | :--- | :--- |
| macOS / Linux | `.magic/scripts/init.sh` | `bash .magic/scripts/init.sh` |
| Windows | `.magic/scripts/init.ps1` | `pwsh .magic/scripts/init.ps1` |

## Directory Structure Created

```plaintext
.design/
├── INDEX.md         # Spec registry
├── RULES.md         # Project constitution
├── specifications/  # Spec files go here
└── tasks/           # Task files go here
```

`PLAN.md` and `TASKS.md` are created by the Plan and Task workflows respectively — not by init.
