#
# SPDX-License-Identifier: MIT
# Copyright wtfsckgh@gmail.com
# Copyright iced contributors
#

# ⚠️This file was generated by GENERATOR!🦹‍♂️

# pylint: disable=invalid-name
# pylint: disable=line-too-long
# pylint: disable=too-many-lines

"""
Decoder options
"""

from typing import List

NONE: int = 0x0000_0000
"""
No option is enabled
"""
NO_INVALID_CHECK: int = 0x0000_0001
"""
Disable some checks for invalid encodings of instructions, eg. most instructions can't use a ``LOCK`` prefix so if one is found, they're decoded as :class:`iced_x86.Code.INVALID` unless this option is enabled.
"""
AMD: int = 0x0000_0002
"""
: AMD decoder: allow 16-bit branch/ret instructions in 64-bit mode, no ``o64 CALL/JMP FAR [mem], o64 LSS/LFS/LGS``, ``UD0`` has no modr/m byte, decode ``LOCK MOV CR``. The AMD decoder can still decode Intel instructions.
"""
FORCE_RESERVED_NOP: int = 0x0000_0004
"""
Decode opcodes ``0F0D`` and ``0F18-0F1F`` as reserved-nop instructions (eg. :class:`iced_x86.Code.RESERVEDNOP_RM32_R32_0F1D`)
"""
UMOV: int = 0x0000_0008
"""
Decode ``UMOV`` instructions
"""
XBTS: int = 0x0000_0010
"""
Decode ``XBTS``/``IBTS``
"""
CMPXCHG486A: int = 0x0000_0020
"""
Decode ``0FA6``/``0FA7`` as ``CMPXCHG``
"""
OLD_FPU: int = 0x0000_0040
"""
Decode some old removed FPU instructions (eg. ``FRSTPM``)
"""
PCOMMIT: int = 0x0000_0080
"""
Decode ``PCOMMIT``
"""
LOADALL286: int = 0x0000_0100
"""
Decode 286 ``LOADALL`` (``0F04`` and ``0F05``)
"""
LOADALL386: int = 0x0000_0200
"""
Decode ``LOADALL386``
"""
CL1INVMB: int = 0x0000_0400
"""
Decode ``CL1INVMB``
"""
MOV_TR: int = 0x0000_0800
"""
Decode ``MOV r32,tr`` and ``MOV tr,r32``
"""
JMPE: int = 0x0000_1000
"""
Decode ``JMPE`` instructions
"""
NO_PAUSE: int = 0x0000_2000
"""
Don't decode ``PAUSE``, decode ``NOP`` instead
"""
NO_WBNOINVD: int = 0x0000_4000
"""
Don't decode ``WBNOINVD``, decode ``WBINVD`` instead
"""
NO_LOCK_MOV_CR: int = 0x0000_8000
"""
DEPRECATED(1.11.0)
"""
NO_MPFX_0FBC: int = 0x0001_0000
"""
Don't decode ``TZCNT``, decode ``BSF`` instead
"""
NO_MPFX_0FBD: int = 0x0002_0000
"""
Don't decode ``LZCNT``, decode ``BSR`` instead
"""
NO_LAHF_SAHF_64: int = 0x0004_0000
"""
Don't decode ``LAHF`` and ``SAHF`` in 64-bit mode
"""
MPX: int = 0x0008_0000
"""
Decode ``MPX`` instructions
"""
CYRIX: int = 0x0010_0000
"""
: Decode most Cyrix instructions: ``FPU``, ``EMMI``, ``SMM``, ``DDI``
"""
CYRIX_SMINT_0F7E: int = 0x0020_0000
"""
Decode Cyrix ``SMINT 0F7E`` (Cyrix 6x86 or earlier)
"""
CYRIX_DMI: int = 0x0040_0000
"""
Decode Cyrix ``DMI`` instructions (AMD Geode GX/LX)
"""
ALTINST: int = 0x0080_0000
"""
Decode Centaur ``ALTINST``
"""

__all__: List[str] = []
