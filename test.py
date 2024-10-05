from mainfile import add


def test_add():
    assert add(2, 2, 2) == 6
    assert add(3, 3, 3) == 9
    assert add(4, 4, 4) == 12


if __name__ == "__mainfile___":
    test_add()
