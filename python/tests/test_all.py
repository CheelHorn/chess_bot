import pytest
import chess_bot


def test_sum_as_string():
    assert chess_bot.sum_as_string(1, 1) == "2"
