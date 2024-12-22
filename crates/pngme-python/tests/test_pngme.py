import pngme

import pytest

def test_pngme_encode():
    file_location = "./crates/pngme-python/tests/dice.png"
    pngme.encode(file_location, "ruSt", "some message")
    output = pngme.decode(file_location, "ruSt")
    assert output == "some message"
    pngme.remove(file_location, "ruSt")
    nothing = pngme.decode(file_location, "ruSt")
    assert nothing == "No secret message found"

def test_pngme_unknown_file():
    with pytest.raises(FileNotFoundError) as exc:
        pngme.encode("unknown.png", "ruSt", "some message")
    assert 'File not found "unknown.png"' in str(exc.value)

def test_pngme_bad_chunk_type():
    file_location = "./crates/pngme-python/tests/dice.png"
    with pytest.raises(ValueError) as exc:
        pngme.encode(file_location, "bad", "some message")
    assert "chunk type must be 4 bytes long" in str(exc.value)