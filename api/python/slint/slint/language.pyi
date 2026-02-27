# Copyright © SixtyFPS GmbH <info@slint.dev>
# SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-2.0 OR LicenseRef-Slint-Software-3.0

import typing

class StandardListViewItem(typing.NamedTuple):
    """Represents an item in a StandardListView and a StandardTableView."""

    text: str = ""
    """The text content of the item"""

class KeyboardModifiers:
    shift: bool
    control: bool
    alt: bool
    meta: bool
    def __new__(
        cls,
        shift: bool = False,
        control: bool = False,
        alt: bool = False,
        meta: bool = False,
    ) -> "KeyboardModifiers": ...
    def __eq__(self, other: object) -> bool: ...
