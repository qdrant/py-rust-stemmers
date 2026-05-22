#!/usr/bin/env python3

import glob
import sys
import zipfile
from pathlib import Path


def validate_wheel(path: Path) -> bool:
    print(f"Checking {path}")

    data = path.read_bytes()
    eocd = data.rfind(b"PK\x05\x06")

    if eocd == -1:
        print(f"ERROR: missing ZIP End Of Central Directory: {path}")
        return False

    comment_len = int.from_bytes(data[eocd + 20 : eocd + 22], "little")
    expected_end = eocd + 22 + comment_len
    extra = data[expected_end:]

    if extra:
        print(f"ERROR: trailing data in {path}: {len(extra)} bytes {extra!r}")
        return False

    try:
        with zipfile.ZipFile(path) as zf:
            bad = zf.testzip()
    except zipfile.BadZipFile as e:
        print(f"ERROR: invalid ZIP file {path}: {e}")
        return False

    if bad:
        print(f"ERROR: corrupted member in {path}: {bad}")
        return False

    return True


def main() -> int:
    wheels = [Path(path) for path in sorted(glob.glob("dist/*.whl"))]

    if not wheels:
        print("ERROR: no wheels found matching dist/*.whl")
        return 1

    for wheel in wheels:
        if not validate_wheel(wheel):
            return 1

    print("All wheels passed ZIP validation.")
    return 0


if __name__ == "__main__":
    sys.exit(main())