import os
import json
import subprocess

out = subprocess.check_output([
    "cargo",
    "build",
    "--message-format=json",
]).decode('utf-8')
out = [json.loads(x.strip()) for x in str(out.strip()).split('\n')]
aorist = [x for x in out if 'target' in x and x['target']['name'] == 'aorist'][0]
artifact_path = aorist['filenames'][0]
os.symlink(artifact_path, 'aorist/aorist/aorist.so')
