# Copyright © SixtyFPS GmbH <info@slint.dev>
# SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-2.0 OR LicenseRef-Slint-Software-3.0

from slint.language import KeyboardModifiers


def test_keyboard_modifiers() -> None:
    # Test initialization with default values
    mods = KeyboardModifiers()
    assert mods.shift is False
    assert mods.control is False
    assert mods.alt is False
    assert mods.meta is False

    # Test initialization with arguments
    mods = KeyboardModifiers(shift=True, control=True, alt=True, meta=True)
    assert mods.shift is True
    assert mods.control is True
    assert mods.alt is True
    assert mods.meta is True

    # Test setters
    mods.shift = False
    assert mods.shift is False
    mods.control = False
    assert mods.control is False
    mods.alt = False
    assert mods.alt is False
    mods.meta = False
    assert mods.meta is False

    # Test equality
    mods2 = KeyboardModifiers()
    assert mods == mods2
    mods2.shift = True
    assert mods != mods2
