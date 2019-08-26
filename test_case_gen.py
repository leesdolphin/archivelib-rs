import ast, functools
import json
import multiprocessing
import os
import pathlib
import pickle
import re
import shlex
import statistics
import subprocess
import sys
import tempfile
import typing as typ

HEX_MACRO_RE = re.compile(r'hex!\([\s\n]*"([\s\n0-9A-Fa-f]+)"[\s\n]*\)', re.MULTILINE)
U8_ARRAY_RE = re.compile(
    r"\[((?:(?:[1-9][0-9]*|0x[0-9A-F]{1,2}|0)(?:_?u8)?,?[\s\n]*)+)\]",
    re.MULTILINE | re.IGNORECASE,
)
U8_REPEAT_RE = re.compile(
    r"\[([1-9][0-9]*|0x[0-9A-F]{1,2}|0)(?:_?u8); ([1-9][0-9]*|0x[0-9A-F]{1,2}|0)\]",
    re.MULTILINE,
)
ROOT = pathlib.Path(__file__).resolve().parent

CRASH_STATUS_CODES = {
    101: "Rust Panic",
    134: "C Abort",
    -6: "C Abort",
    999: "Python Timeout",
}

KCOV_COMMON_OPTS = []


def kcov_report(dir):
    return []
    # return ["kcov", *KCOV_OPTS, "--report-only", dir]


def kcov_collect(dir):
    return []
    # return ["kcov", *KCOV_OPTS, "--collect-only", dir]


PICKLED_INPUTS = ROOT / "data.pickle"
UNZIP_EXEC_NEW = ROOT / "target/debug/unalzip"
UNZIP_EXEC_SYS = ROOT / "target/debug/unalszip"
ZIP_EXEC_NEW = ROOT / "target/debug/alzip"
ZIP_EXEC_SYS = ROOT / "target/debug/alszip"

TEST_OUTPUT_ROOT_DIR = ROOT / "gen_test_cases"
TEST_OUTPUT_DIRS = {
    name: TEST_OUTPUT_ROOT_DIR / name
    for name in [
        "match",
        "match_err",
        "match_crash",
        "crash",
        "difference_out",
        "difference_err",
        "unknown",
        "wip",
    ]
}

RAW_SYS_RE = re.compile(rb'".*? \(0x[0-9a-f]{4,}\) ')
ERROR_MAPPING = {
    rb'Error: "BinaryTreeError(Type1)"': "BTE1",
    rb'Error: "Binary tree error: Type1"': "BTE1",
    rb'Error: "Internal error: -101\u{0}"': "BTE1",
    rb'Error: "Internal 1 error in Greenleaf Decompression routine\u{0}"': "BTE1",
    rb'Error: "BinaryTreeError(Type2)"': "BTE2",
    rb'Error: "Binary tree error: Type2"': "BTE2",
    rb'Error: "Internal error: -102\u{0}"': "BTE2",
    rb'Error: "Internal 2 error in Greenleaf Decompression routine\u{0}"': "BTE1",
    rb'Error: "IOError: failed to write whole buffer"': "OOM",
    rb'Error: "Attempt to allocate a huge buffer of 65536 bytes for ALMemory decompress\u{0}"': "OOM",
    rb'Error: "Attempt to allocate a huge buffer of 65536 bytes for ALMemory\u{0}"': "OOM",
    rb'Error: "InvariantFailure"': "INV",
    rb'Error: "Invariant Failure"': "INV",
}


def sp_run(cmd, **opts):
    cmd = [str(i) for i in cmd]
    # print(f"Running: {' '.join(map(shlex.quote, cmd))}")
    return subprocess.run(cmd, **opts)


def get_all_inputs() -> typ.Set[bytes]:
    inputs = frozenset()
    try:
        with PICKLED_INPUTS.open("rb") as f:
            inputs = inputs | inputs.union(pickle.load(f))
    except:
        pass
    for folder in ["_known_inputs", "fuzz/known_inputs", "fuzz/corpus"]:
        if (ROOT / folder).is_dir():
            inputs = inputs | get_inputs_in_folder(ROOT / folder)
    for folder in ROOT.iterdir():
        if folder.name.endswith("_corpus"):
            inputs = inputs | get_inputs_in_folder(folder)
    for folder in (ROOT / "fuzz").iterdir():
        if "afl_out" in folder.name:
            for f in ["hangs", "crashes"]:
                if (folder / f).is_dir():
                    inputs = inputs | get_inputs_in_folder(folder / f)
    for folder in ["tests", "src", "old_tests", "archivelib-sys-refactored/src"]:
        inputs = inputs | get_test_case_inputs(ROOT / folder)
    inputs = {i for i in inputs if i is not None}
    with PICKLED_INPUTS.open("wb") as f:
        pickle.dump(inputs, f, protocol=pickle.HIGHEST_PROTOCOL)
    return inputs


def get_inputs_in_folder(folder) -> typ.Set[bytes]:
    return {
        f.read_bytes()
        for f in pathlib.Path(folder).iterdir()
        if f.is_file() and f.name != "README.txt"
    }


def get_test_case_inputs(folder=ROOT / "tests") -> typ.Set[bytes]:
    inputs = set()
    for file in folder.iterdir():
        if file.is_dir():
            inputs = inputs | get_test_case_inputs(file)
        elif file.is_file():
            for match in HEX_MACRO_RE.finditer(file.read_text()):
                inputs.add(bytes.fromhex(match.group(1).replace("\n", "")))
            for match in U8_ARRAY_RE.finditer(file.read_text()):
                l = ast.literal_eval(match.group(0).replace("_u8", "").replace("_", ""))
                if all(i <= 255 for i in l):
                    inputs.add(bytes(l))
            for match in U8_REPEAT_RE.finditer(file.read_text()):
                v = int(match.group(1))
                if v <= 255:
                    inputs.add(bytes([v]) * int(match.group(2)))
    return inputs


def run_exec(input, kcov_base, exec, level="4"):
    try:
        result = sp_run(
            [*kcov_collect(kcov_base), exec, f"-{level}"],
            input=input,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=30,
        )
        rc = result.returncode
        out = result.stdout
        err = result.stderr.strip()
    except subprocess.TimeoutExpired as e:
        rc = 999
        out = e.stdout
        err = e.stderr.strip()
    match_err = RAW_SYS_RE.sub(b'"', err)
    if not err:
        known_err = True
    elif match_err in ERROR_MAPPING:
        known_err = ERROR_MAPPING[match_err]
    else:
        known_err = None
    if rc == 0:
        assert known_err is True
    return (rc, out, match_err, known_err)


def test_case_name(input):
    import hashlib

    sha = hashlib.sha1(input).hexdigest()
    size = len(input) if len(input) < 100000 else 99999
    return f"{size:05}~{sha}"


def bytes_to_test_hex(data):
    aparts = []
    bparts = []
    out = ""
    for b in data:
        aparts.append(f"{b:02X}")
        if len(aparts) >= 8:
            bparts.append(" ".join(aparts))
            aparts = []
        if len(bparts) >= 4:
            out += f'{"  ".join(bparts)}\n'
            bparts = []
    if aparts:
        bparts.append(" ".join(aparts))
    if bparts:
        out += f'{"  ".join(bparts)}\n'
    return out

def bytes_to_rust_array(data):
    aparts = []
    out = ""
    for b in data:
        aparts.append(f"0x{b:02X},")
        if len(aparts) >= 16:
            out += f'{"  ".join(aparts)}\n'
            aparts = []
    if aparts:
        out += f'{"  ".join(aparts)}\n'
    return f"[\n{out}]\n"
def output_test_case(
    input,
    *,
    sys_out=None,
    new_out=None,
    sys_err=None,
    new_err=None,
    sys_cov=None,
    new_cov=None,
    fail_type,
):
    name = test_case_name(input)
    out = TEST_OUTPUT_DIRS[fail_type] / name
    sp_run(["rm", "-rf", out], check=True)
    out.mkdir(parents=True)
    (out / "input.dat").write_bytes(input)
    (out / "input.txt").write_text(bytes_to_test_hex(input))
    (out / "input.txt").write_text(bytes_to_rust_array(input))
    for (o, name) in [(sys_out, "sys_"), (new_out, "new_")]:
        if o is not None:
            (out / f"{name}output.dat").write_bytes(o)
            (out / f"{name}output.txt").write_text(bytes_to_test_hex(o))
            (out / f"{name}output.txt").write_text(bytes_to_rust_array(o))
    for (o, name) in [(sys_err, "sys_"), (new_err, "new_")]:
        if o is not None:
            (out / f"{name}err.txt").write_bytes(o)

    for (orig, exec, target) in [
        (sys_cov, UNZIP_EXEC_SYS, "sys"),
        (new_cov, UNZIP_EXEC_NEW, "new"),
    ]:
        if orig is not None:
            kcov_out = out / target
            kcov_call = kcov_report(orig)
            if kcov_call:
                sp_run([*kcov_call, exec], check=True)
                sp_run(["cp", "-r", orig, kcov_out], check=True)
    # print(f"{sha1}: {len(input)} byte(s); {fail_type}")


def run(input: bytes):
    name = test_case_name(input)
    to_rm = {k: o / name for k, o in TEST_OUTPUT_DIRS.items()}
    for k in ["match"]:
        if to_rm[k].is_dir():
            # print(f"{sha1}: {len(input)} byte(s); Existing match")
            return {
                "name": name,
                # "input": input,
                "len": len(input),
                "input_rs": ", ".join(f"0x{i:02X}" for i in input),
                # "input_16": " ".join(f"{i:02X}" for i in input),
                "fail_type": k,
                "sys": {"ok": True, "err": "", "code": True},
                "new": {"ok": True, "err": "", "code": True},
            }
    base = TEST_OUTPUT_DIRS["wip"] / name
    sp_run(["rm", "-rf", *to_rm.values()], check=True)
    sys_base = base / "sys"
    new_base = base / "new"
    sys_base.mkdir(parents=True)
    new_base.mkdir(parents=True)
    (base / "input.dat").write_bytes(input)

    (sys_rc, sys_out, sys_err, sys_known_err) = run_exec(
        input, sys_base, UNZIP_EXEC_SYS
    )
    (new_rc, new_out, new_err, new_known_err) = run_exec(
        input, new_base, UNZIP_EXEC_NEW
    )
    if not sys_known_err:
        print("Unknown error: ", sys_err)
    if not new_known_err:
        print("Unknown error: ", new_err)

    if sys_rc == 0:
        if new_rc != 0:
            fail_type = "difference_err"
        elif sys_out != new_out:
            fail_type = "difference_out"
        else:
            fail_type = "match"
    elif sys_known_err:
        if new_known_err == sys_known_err:
            fail_type = "match_err"
        else:
            fail_type = "difference_err"
    elif new_known_err == "INV" and sys_rc in CRASH_STATUS_CODES:
        fail_type = "match_crash"
    elif {sys_rc, new_rc} & set(CRASH_STATUS_CODES):
        # Rust crash, Segfault/Abort, Timeout.
        fail_type = "crash"
    else:
        fail_type = "unknown"
        print("Unknown data: ", len(input), name)
        print(
            " sys:",
            (
                sys_rc,
                CRASH_STATUS_CODES.get(sys_rc, "Non-crash"),
                len(sys_out),
                sys_err,
                sys_known_err,
            ),
        )
        print(
            " new:",
            (
                new_rc,
                CRASH_STATUS_CODES.get(new_rc, "Non-crash"),
                len(new_out),
                new_err,
                new_known_err,
            ),
        )
    output_test_case(
        input,
        sys_out=sys_out,
        sys_err=sys_err,
        sys_cov=sys_base,
        new_out=new_out,
        new_err=new_err,
        new_cov=new_base,
        fail_type=fail_type,
    )
    sp_run(["rm", "-rf", base])
    return {
        "name": name,
        # "input": input,
        "len": len(input),
        "input_rs": ", ".join(f"0x{i:02X}" for i in input),
        # "input_16": " ".join(f"{i:02X}" for i in input),
        "fail_type": fail_type,
        "sys": {"ok": sys_rc == 0, "err": str(sys_err), "code": sys_known_err},
        "new": {"ok": new_rc == 0, "err": str(new_err), "code": new_known_err},
    }


def tmin_test_case(cout,tmin_out,item):
    if not item or len(item) > (2 ** 14):
        # tmin doesn't like the empty file
        return b''
    with tempfile.NamedTemporaryFile("rb") as tout:
        sp_run(
            [
                "cargo",
                "afl",
                "tmin",
                "-i",
                out,
                "-o",
                tout.name,
                "-t5000",
                "-m250",
                "--",
                "target/debug/alfuzz_afl",
            ]
        )
        tmin_data = pathlib.Path(tout.name).read_bytes()
    (tmin_out / test_case_name(tmin_data)).write_bytes(tmin_data)
    return tmin_data


def minimise_test_case_corpus(tmin_out):
    cmin_out = ROOT / "_cmin_corpus"
    cmin_edge_out = ROOT / "_cmin_edge_corpus"
    sp_run(["rm", "-rf", cmin_out
,cmin_edge_out], check=True)

    sp_run(
        [
            "cargo",
            "afl",
            "cmin",
            "-i",
            tmin_out,
            "-o",
            cmin_out,
            "-t5000",
            "-m250",
            "--",
            "target/debug/alfuzz_afl",
        ]
    )
    sp_run(
        [
            "cargo",
            "afl",
            "cmin",
            "-i",
            tmin_out,
            "-o",
            cmin_edge_out,
            "-e",
            "-t5000",
            "-m250",
            "--",
            "target/debug/alfuzz_afl",
        ]
    )
    return (cmin_out, cmin_edge_out)


def main():
    data = sorted(get_all_inputs(), key=len, reverse=True)
    lens = list(map(len, data))
    print(f"Total test cases: {len(data)}")
    print(f"Average length: {statistics.mean(lens):0.1f}")
    print(f"Median length:  {statistics.median(lens):0.1f}")

    sp_run(["cargo", "build"], check=True, cwd=ROOT)
    sp_run(
        ["cargo", "build"],
        check=True,
        # cwd=(ROOT / "archivelib-sys-orig"),
        cwd=(ROOT / "archivelib-sys-refactored"),
    )
    sp_run(
        ["cargo", "afl", "build", "--features=fuzz-afl"], check=True, cwd=(ROOT / "cli")
    )
    cout = ROOT / "_corpus"
    tmin_out = ROOT / "_tmin_corpus"
    sp_run(["rm", "-rf", tmin_out], check=True)
    tmin_out.mkdir(exist_ok=True)
    cout.mkdir(exist_ok=True)


    test_cases = {}
    changed_data = data
    with multiprocessing.Pool(16) as p:
        while changed_data:
            for item in changed_data:
                out = cout / test_case_name(item)
                out.write_bytes(item)
            case_min = p.apply_async(minimise_test_case_corpus, args=(cout,))
            for out in p.imap_unordered(run, changed_data):
                key = (
                    out["fail_type"],
                    out["sys"]["code"] or out["sys"]["err"],
                    out["new"]["code"] or out["new"]["err"],
                )
                test_cases.setdefault(key, []).append(out)
            (a_dir, b_dir) = case_min.get()
            minimal_cases = {
                file.read_bytes()
                for folder in [a_dir, b_dir]
                for file in folder.iterdir()
            }
            p.map(functools.partial(tmin_test_case, cout, tmin_out), data)

            old_data = set(data)
            new_data = get_all_inputs()
            changed_data = new_data - old_data


    data = []
    for key, cases in test_cases.items():
        cases.sort(key=lambda k: (k["len"], k["input_rs"]))
        data.append(
            {"key": dict(zip(["fail_type", "sys", "new"], key)), "cases": cases[:5]}
        )
    with (TEST_OUTPUT_ROOT_DIR / "interesting.json").open("w") as f:
        json.dump(data, f, sort_keys=True, default=repr, indent=2)


if __name__ == "__main__":
    main()
