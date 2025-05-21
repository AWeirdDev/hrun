import pytest
import brun


def test_sum_as_string():
    assert brun.sum_as_string(1, 1) == "2"
