import struct
import json
import sys

def parse_glb(file_path):
    print(f"--- {file_path} ---")
    try:
        with open(file_path, 'rb') as f:
            magic = f.read(4)
            if magic != b'glTF':
                print(f"Not a GLTF file: {file_path}")
                return
            version, length = struct.unpack('<II', f.read(8))
            chunk_len, chunk_type = struct.unpack('<II', f.read(8))
            if chunk_type != 0x4E4F534A: # 'JSON'
                print("First chunk is not JSON")
                return
            json_data = f.read(chunk_len).decode('utf-8')
            data = json.loads(json_data)
            
            scenes = data.get('scenes', [])
            print(f"Scenes ({len(scenes)}):")
            for i, s in enumerate(scenes):
                print(f"  Scene {i}: {s.get('name', 'unnamed')}, nodes: {s.get('nodes', [])}")
            
            meshes = data.get('meshes', [])
            print(f"\nMeshes ({len(meshes)}):")
            for i, m in enumerate(meshes):
                print(f"  Mesh {i}: {m.get('name', 'unnamed')}")
            
            nodes = data.get('nodes', [])
            print(f"\nNodes ({len(nodes)}):")
            for i, n in enumerate(nodes):
                print(f"  Node {i}: name='{n.get('name', 'unnamed')}'")
                if 'translation' in n: print(f"    translation={n['translation']}")
                if 'rotation' in n: print(f"    rotation={n['rotation']}")
                if 'scale' in n: print(f"    scale={n['scale']}")
    except Exception as e:
        print(f"Error parsing {file_path}: {e}")

parse_glb(r'c:\Users\jlaut\git\RetroGameGame\assets\sprites\future\nebula_bouncer\ship_models\TechFighter.glb')
parse_glb(r'c:\Users\jlaut\git\RetroGameGame\assets\sprites\future\nebula_bouncer\ship_models\AlienFighter.glb')
