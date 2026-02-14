# Asset Creation Pipeline

This document outlines the workflow for creating, processing, and integrating new game assets using the "Nano Banana" (Gemini Image Generation) pipeline.

## Overview

The pipeline consists of three main stages:
1.  **Definition**: Defining asset requirements in `assets/asset_requests.json`.
2.  **Generation**: Using valid AI prompts to generate raw sprite sheets.
3.  **Processing**: Slicing raw sheets into individual sprites using `generate_assets.py`.

## 1. Defining Assets

Edit `assets/asset_requests.json` to add new asset batches.

```json
{
  "name": "my_new_asset",
  "era": "80s",
  "description": "Description of the asset (e.g., 'Space ship, top down view')",
  "count": 1
}
```

*   **Eras**: Defined in the `eras` section of the JSON. Each era has a `prompt_prefix` and `prompt_suffix` that is automatically applied to enforce style consistency.
*   **Best Practices**:
    *   **No Text**: Explicitly request "no text".
    *   **Background**: Request "solid black background" for 80s/Pixel art to verify clean slicing.
    *   **Grid**: Mention "grid-based" or "sprite sheet" to get multiple frames.

## 2. Generating Assets

Run the tooling script to view the generated prompts or initiate the generation loop (if automated generation is enabled):

```bash
# Activate virtual environment
source .venv/bin/activate

# List queued assets and their full prompts
python assets/scripts/generate_assets.py list
```

**Manual Generation (Current Workflow):**
1.  Copy the prompt from the `list` command.
2.  Use the Image Generation tool (Nano Banana) with that prompt.
3.  Save the resulting image to `assets/raw/{era}/{name}.png`.

## 3. Processing Assets

Once raw images are in place:

```bash
python assets/scripts/generate_assets.py process -v
```

This will:
*   Read `asset_requests.json`.
*   Look for corresponding files in `assets/raw/{era}/`.
*   Slice them using `slice_spritesheet.py`.
*   Output individual sprites to `assets/sprites/{era}/{name}/`.

### Tuning Slicing
If sprites are being cut incorrectly or noise is included:
*   **Min Area**: Increase `--min-area` (default is 64) in `generate_assets.py`.
*   **Background Color**: The script auto-detects background colors. If it fails, you can manually specify colors in the underlying `slice_spritesheet.py` call.

## Directory Structure

```
assets/
├── asset_requests.json       # Configuration
├── raw/                      # Raw generated sheets (Gitignored?)
│   └── 80s/
│       └── my_asset.png
├── sprites/                  # Final sliced game assets
│   └── 80s/
│       └── my_asset/
│           ├── my_asset_000.png
│           └── ...
└── scripts/
    ├── generate_assets.py    # Pipeline orchestrator
    └── slice_spritesheet.py  # Image processing logic
```
