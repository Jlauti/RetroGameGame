#!/usr/bin/env python3
import argparse

import json
import os
import sys
import subprocess
from pathlib import Path

# Paths
BASE_DIR = Path(__file__).resolve().parent.parent.parent
ASSETS_DIR = BASE_DIR / "assets"
REQUESTS_FILE = ASSETS_DIR / "asset_requests.json"
RAW_DIR = ASSETS_DIR / "raw"
SPRITES_DIR = ASSETS_DIR / "sprites"
SCRIPTS_DIR = ASSETS_DIR / "scripts"
SLICE_SCRIPT = SCRIPTS_DIR / "slice_spritesheet.py"

def load_requests():
    if not REQUESTS_FILE.exists():
        print(f"Error: {REQUESTS_FILE} not found.")
        sys.exit(1)
    with open(REQUESTS_FILE, 'r') as f:
        return json.load(f)

def build_prompt(item, config):
    era_key = item.get('era', '80s')
    era_config = config.get('eras', {}).get(era_key, {})
    
    prefix = era_config.get('prompt_prefix', "")
    suffix = era_config.get('prompt_suffix', "")
    desc = item.get('description', "")
    
    # Combine parts
    full_prompt = f"{prefix} {desc} {suffix}".strip()
    return full_prompt

def cmd_list(args):
    """List assets to be generated with their constructed prompts."""
    config = load_requests()
    requests = config.get('requests', [])
    
    output_list = []
    
    for item in requests:
        prompt = build_prompt(item, config)
        output_list.append({
            "name": item['name'],
            "era": item.get('era', '80s'),
            "prompt": prompt,
            "count": item.get('count', 1),
            "status": "pending" # TODO: Check if file exists
        })
        
    if args.json:
        print(json.dumps(output_list, indent=2))
    else:
        for item in output_list:
            print(f"Name: {item['name']}")
            print(f"Era:  {item['era']}")
            print(f"Prompt: {item['prompt']}")
            print("-" * 40)

def cmd_process(args):
    """Process raw assets (slice them)."""
    # Create raw dir if it doesn't exist
    RAW_DIR.mkdir(parents=True, exist_ok=True)
    
    # Check for raw files that match requests
    config = load_requests()
    requests = config.get('requests', [])
    
    for item in requests:
        name = item['name']
        era = item.get('era', '80s')
        
        # We assume raw files are named "{name}.png" in "assets/raw/{era}/"
        # Or just "assets/raw/{name}.png" for simplicity? Let's verify structure.
        # Plan said: assets/raw/{era}/
        
        era_raw_dir = RAW_DIR / era
        raw_file = era_raw_dir / f"{name}.png"
        
        if not raw_file.exists():
             # Try checking root of raw just in case
             raw_file_alt = RAW_DIR / f"{name}.png"
             if raw_file_alt.exists():
                 raw_file = raw_file_alt
             else:
                 if args.verbose:
                     print(f"Skipping {name}: No raw file found at {raw_file}")
                 continue
                 
        print(f"Processing {name}...")
        
        # Output dir: assets/sprites/{era}/{name}/
        out_dir = SPRITES_DIR / era / name
        if out_dir.exists() and not args.force:
            print(f"  Output directory {out_dir} already exists. Use --force to overwrite.")
            continue
            
        # Call slice_spritesheet.py
        # Default flags: --bg-auto --min-area 64 --pad 1
        cmd = [
            sys.executable, str(SLICE_SCRIPT),
            str(raw_file),
            "-o", str(out_dir),
            "--bg-auto",
            "--min-area", "64",
            "--pad", "1",
            "--prefix", name
        ]
        
        if args.verbose:
            print(f"  Running: {' '.join(cmd)}")
            
        try:
            subprocess.run(cmd, check=True)
            print(f"  Success! Output in {out_dir}")
        except subprocess.CalledProcessError as e:
            print(f"  Error processing {name}: {e}")

def main():
    parser = argparse.ArgumentParser(description="RetroGameGame Asset Pipeline Manager")
    subparsers = parser.add_subparsers(dest="command", help="Command to run")
    
    # List command
    parser_list = subparsers.add_parser("list", help="List asset requests and prompts")
    parser_list.add_argument("--json", action="store_true", help="Output in JSON format")
    
    # Process command
    parser_process = subparsers.add_parser("process", help="Slice and process raw assets")
    parser_process.add_argument("--force", action="store_true", help="Overwrite existing sprites")
    parser_process.add_argument("-v", "--verbose", action="store_true", help="Verbose output")
    
    args = parser.parse_args()
    
    if args.command == "list":
        cmd_list(args)
    elif args.command == "process":
        cmd_process(args)
    else:
        parser.print_help()

if __name__ == "__main__":
    main()
