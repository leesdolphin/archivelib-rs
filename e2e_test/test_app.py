import itertools

import pytest

LEVEL_RANGE = [0, 1, 2, 3, 4]


def gen_all_inputs_range(minlen, maxlen, prefix=b"", levels=[0,4]):
    for length in range(minlen, maxlen + 1):
        if length >= 4:
            return
        for input in itertools.product(range(0, 256), repeat=length):
            for level in levels:
                yield level, prefix + bytes(input)


def test_smoke(al_runner):
    al_runner.smoketest()


@pytest.mark.parametrize("level", LEVEL_RANGE)
def test_single_byte(al_runner, level):
    out, err = al_runner.test_unzip(b"\xfe", level=level)


@pytest.mark.parametrize("level, input", gen_all_inputs_range(0, 1))
def test_single_byte(al_runner, level, input):
    out, err = al_runner.test_unzip(input, level=level)


# @pytest.mark.parametrize("level, input", gen_all_inputs_range())
# def test_single_byte(al_runner, level,input):
