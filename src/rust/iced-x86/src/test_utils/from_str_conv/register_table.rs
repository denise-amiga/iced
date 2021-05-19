// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

use crate::Register;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
	pub(super) static ref TO_REGISTER_HASH: HashMap<&'static str, Register> = {
		// GENERATOR-BEGIN: RegisterHash
		// ⚠️This was generated by GENERATOR!🦹‍♂️
		let mut h = HashMap::with_capacity(256);
		let _ = h.insert("none", Register::None);
		let _ = h.insert("al", Register::AL);
		let _ = h.insert("cl", Register::CL);
		let _ = h.insert("dl", Register::DL);
		let _ = h.insert("bl", Register::BL);
		let _ = h.insert("ah", Register::AH);
		let _ = h.insert("ch", Register::CH);
		let _ = h.insert("dh", Register::DH);
		let _ = h.insert("bh", Register::BH);
		let _ = h.insert("spl", Register::SPL);
		let _ = h.insert("bpl", Register::BPL);
		let _ = h.insert("sil", Register::SIL);
		let _ = h.insert("dil", Register::DIL);
		let _ = h.insert("r8l", Register::R8L);
		let _ = h.insert("r9l", Register::R9L);
		let _ = h.insert("r10l", Register::R10L);
		let _ = h.insert("r11l", Register::R11L);
		let _ = h.insert("r12l", Register::R12L);
		let _ = h.insert("r13l", Register::R13L);
		let _ = h.insert("r14l", Register::R14L);
		let _ = h.insert("r15l", Register::R15L);
		let _ = h.insert("ax", Register::AX);
		let _ = h.insert("cx", Register::CX);
		let _ = h.insert("dx", Register::DX);
		let _ = h.insert("bx", Register::BX);
		let _ = h.insert("sp", Register::SP);
		let _ = h.insert("bp", Register::BP);
		let _ = h.insert("si", Register::SI);
		let _ = h.insert("di", Register::DI);
		let _ = h.insert("r8w", Register::R8W);
		let _ = h.insert("r9w", Register::R9W);
		let _ = h.insert("r10w", Register::R10W);
		let _ = h.insert("r11w", Register::R11W);
		let _ = h.insert("r12w", Register::R12W);
		let _ = h.insert("r13w", Register::R13W);
		let _ = h.insert("r14w", Register::R14W);
		let _ = h.insert("r15w", Register::R15W);
		let _ = h.insert("eax", Register::EAX);
		let _ = h.insert("ecx", Register::ECX);
		let _ = h.insert("edx", Register::EDX);
		let _ = h.insert("ebx", Register::EBX);
		let _ = h.insert("esp", Register::ESP);
		let _ = h.insert("ebp", Register::EBP);
		let _ = h.insert("esi", Register::ESI);
		let _ = h.insert("edi", Register::EDI);
		let _ = h.insert("r8d", Register::R8D);
		let _ = h.insert("r9d", Register::R9D);
		let _ = h.insert("r10d", Register::R10D);
		let _ = h.insert("r11d", Register::R11D);
		let _ = h.insert("r12d", Register::R12D);
		let _ = h.insert("r13d", Register::R13D);
		let _ = h.insert("r14d", Register::R14D);
		let _ = h.insert("r15d", Register::R15D);
		let _ = h.insert("rax", Register::RAX);
		let _ = h.insert("rcx", Register::RCX);
		let _ = h.insert("rdx", Register::RDX);
		let _ = h.insert("rbx", Register::RBX);
		let _ = h.insert("rsp", Register::RSP);
		let _ = h.insert("rbp", Register::RBP);
		let _ = h.insert("rsi", Register::RSI);
		let _ = h.insert("rdi", Register::RDI);
		let _ = h.insert("r8", Register::R8);
		let _ = h.insert("r9", Register::R9);
		let _ = h.insert("r10", Register::R10);
		let _ = h.insert("r11", Register::R11);
		let _ = h.insert("r12", Register::R12);
		let _ = h.insert("r13", Register::R13);
		let _ = h.insert("r14", Register::R14);
		let _ = h.insert("r15", Register::R15);
		let _ = h.insert("eip", Register::EIP);
		let _ = h.insert("rip", Register::RIP);
		let _ = h.insert("es", Register::ES);
		let _ = h.insert("cs", Register::CS);
		let _ = h.insert("ss", Register::SS);
		let _ = h.insert("ds", Register::DS);
		let _ = h.insert("fs", Register::FS);
		let _ = h.insert("gs", Register::GS);
		let _ = h.insert("xmm0", Register::XMM0);
		let _ = h.insert("xmm1", Register::XMM1);
		let _ = h.insert("xmm2", Register::XMM2);
		let _ = h.insert("xmm3", Register::XMM3);
		let _ = h.insert("xmm4", Register::XMM4);
		let _ = h.insert("xmm5", Register::XMM5);
		let _ = h.insert("xmm6", Register::XMM6);
		let _ = h.insert("xmm7", Register::XMM7);
		let _ = h.insert("xmm8", Register::XMM8);
		let _ = h.insert("xmm9", Register::XMM9);
		let _ = h.insert("xmm10", Register::XMM10);
		let _ = h.insert("xmm11", Register::XMM11);
		let _ = h.insert("xmm12", Register::XMM12);
		let _ = h.insert("xmm13", Register::XMM13);
		let _ = h.insert("xmm14", Register::XMM14);
		let _ = h.insert("xmm15", Register::XMM15);
		let _ = h.insert("xmm16", Register::XMM16);
		let _ = h.insert("xmm17", Register::XMM17);
		let _ = h.insert("xmm18", Register::XMM18);
		let _ = h.insert("xmm19", Register::XMM19);
		let _ = h.insert("xmm20", Register::XMM20);
		let _ = h.insert("xmm21", Register::XMM21);
		let _ = h.insert("xmm22", Register::XMM22);
		let _ = h.insert("xmm23", Register::XMM23);
		let _ = h.insert("xmm24", Register::XMM24);
		let _ = h.insert("xmm25", Register::XMM25);
		let _ = h.insert("xmm26", Register::XMM26);
		let _ = h.insert("xmm27", Register::XMM27);
		let _ = h.insert("xmm28", Register::XMM28);
		let _ = h.insert("xmm29", Register::XMM29);
		let _ = h.insert("xmm30", Register::XMM30);
		let _ = h.insert("xmm31", Register::XMM31);
		let _ = h.insert("ymm0", Register::YMM0);
		let _ = h.insert("ymm1", Register::YMM1);
		let _ = h.insert("ymm2", Register::YMM2);
		let _ = h.insert("ymm3", Register::YMM3);
		let _ = h.insert("ymm4", Register::YMM4);
		let _ = h.insert("ymm5", Register::YMM5);
		let _ = h.insert("ymm6", Register::YMM6);
		let _ = h.insert("ymm7", Register::YMM7);
		let _ = h.insert("ymm8", Register::YMM8);
		let _ = h.insert("ymm9", Register::YMM9);
		let _ = h.insert("ymm10", Register::YMM10);
		let _ = h.insert("ymm11", Register::YMM11);
		let _ = h.insert("ymm12", Register::YMM12);
		let _ = h.insert("ymm13", Register::YMM13);
		let _ = h.insert("ymm14", Register::YMM14);
		let _ = h.insert("ymm15", Register::YMM15);
		let _ = h.insert("ymm16", Register::YMM16);
		let _ = h.insert("ymm17", Register::YMM17);
		let _ = h.insert("ymm18", Register::YMM18);
		let _ = h.insert("ymm19", Register::YMM19);
		let _ = h.insert("ymm20", Register::YMM20);
		let _ = h.insert("ymm21", Register::YMM21);
		let _ = h.insert("ymm22", Register::YMM22);
		let _ = h.insert("ymm23", Register::YMM23);
		let _ = h.insert("ymm24", Register::YMM24);
		let _ = h.insert("ymm25", Register::YMM25);
		let _ = h.insert("ymm26", Register::YMM26);
		let _ = h.insert("ymm27", Register::YMM27);
		let _ = h.insert("ymm28", Register::YMM28);
		let _ = h.insert("ymm29", Register::YMM29);
		let _ = h.insert("ymm30", Register::YMM30);
		let _ = h.insert("ymm31", Register::YMM31);
		let _ = h.insert("zmm0", Register::ZMM0);
		let _ = h.insert("zmm1", Register::ZMM1);
		let _ = h.insert("zmm2", Register::ZMM2);
		let _ = h.insert("zmm3", Register::ZMM3);
		let _ = h.insert("zmm4", Register::ZMM4);
		let _ = h.insert("zmm5", Register::ZMM5);
		let _ = h.insert("zmm6", Register::ZMM6);
		let _ = h.insert("zmm7", Register::ZMM7);
		let _ = h.insert("zmm8", Register::ZMM8);
		let _ = h.insert("zmm9", Register::ZMM9);
		let _ = h.insert("zmm10", Register::ZMM10);
		let _ = h.insert("zmm11", Register::ZMM11);
		let _ = h.insert("zmm12", Register::ZMM12);
		let _ = h.insert("zmm13", Register::ZMM13);
		let _ = h.insert("zmm14", Register::ZMM14);
		let _ = h.insert("zmm15", Register::ZMM15);
		let _ = h.insert("zmm16", Register::ZMM16);
		let _ = h.insert("zmm17", Register::ZMM17);
		let _ = h.insert("zmm18", Register::ZMM18);
		let _ = h.insert("zmm19", Register::ZMM19);
		let _ = h.insert("zmm20", Register::ZMM20);
		let _ = h.insert("zmm21", Register::ZMM21);
		let _ = h.insert("zmm22", Register::ZMM22);
		let _ = h.insert("zmm23", Register::ZMM23);
		let _ = h.insert("zmm24", Register::ZMM24);
		let _ = h.insert("zmm25", Register::ZMM25);
		let _ = h.insert("zmm26", Register::ZMM26);
		let _ = h.insert("zmm27", Register::ZMM27);
		let _ = h.insert("zmm28", Register::ZMM28);
		let _ = h.insert("zmm29", Register::ZMM29);
		let _ = h.insert("zmm30", Register::ZMM30);
		let _ = h.insert("zmm31", Register::ZMM31);
		let _ = h.insert("k0", Register::K0);
		let _ = h.insert("k1", Register::K1);
		let _ = h.insert("k2", Register::K2);
		let _ = h.insert("k3", Register::K3);
		let _ = h.insert("k4", Register::K4);
		let _ = h.insert("k5", Register::K5);
		let _ = h.insert("k6", Register::K6);
		let _ = h.insert("k7", Register::K7);
		let _ = h.insert("bnd0", Register::BND0);
		let _ = h.insert("bnd1", Register::BND1);
		let _ = h.insert("bnd2", Register::BND2);
		let _ = h.insert("bnd3", Register::BND3);
		let _ = h.insert("cr0", Register::CR0);
		let _ = h.insert("cr1", Register::CR1);
		let _ = h.insert("cr2", Register::CR2);
		let _ = h.insert("cr3", Register::CR3);
		let _ = h.insert("cr4", Register::CR4);
		let _ = h.insert("cr5", Register::CR5);
		let _ = h.insert("cr6", Register::CR6);
		let _ = h.insert("cr7", Register::CR7);
		let _ = h.insert("cr8", Register::CR8);
		let _ = h.insert("cr9", Register::CR9);
		let _ = h.insert("cr10", Register::CR10);
		let _ = h.insert("cr11", Register::CR11);
		let _ = h.insert("cr12", Register::CR12);
		let _ = h.insert("cr13", Register::CR13);
		let _ = h.insert("cr14", Register::CR14);
		let _ = h.insert("cr15", Register::CR15);
		let _ = h.insert("dr0", Register::DR0);
		let _ = h.insert("dr1", Register::DR1);
		let _ = h.insert("dr2", Register::DR2);
		let _ = h.insert("dr3", Register::DR3);
		let _ = h.insert("dr4", Register::DR4);
		let _ = h.insert("dr5", Register::DR5);
		let _ = h.insert("dr6", Register::DR6);
		let _ = h.insert("dr7", Register::DR7);
		let _ = h.insert("dr8", Register::DR8);
		let _ = h.insert("dr9", Register::DR9);
		let _ = h.insert("dr10", Register::DR10);
		let _ = h.insert("dr11", Register::DR11);
		let _ = h.insert("dr12", Register::DR12);
		let _ = h.insert("dr13", Register::DR13);
		let _ = h.insert("dr14", Register::DR14);
		let _ = h.insert("dr15", Register::DR15);
		let _ = h.insert("st0", Register::ST0);
		let _ = h.insert("st1", Register::ST1);
		let _ = h.insert("st2", Register::ST2);
		let _ = h.insert("st3", Register::ST3);
		let _ = h.insert("st4", Register::ST4);
		let _ = h.insert("st5", Register::ST5);
		let _ = h.insert("st6", Register::ST6);
		let _ = h.insert("st7", Register::ST7);
		let _ = h.insert("mm0", Register::MM0);
		let _ = h.insert("mm1", Register::MM1);
		let _ = h.insert("mm2", Register::MM2);
		let _ = h.insert("mm3", Register::MM3);
		let _ = h.insert("mm4", Register::MM4);
		let _ = h.insert("mm5", Register::MM5);
		let _ = h.insert("mm6", Register::MM6);
		let _ = h.insert("mm7", Register::MM7);
		let _ = h.insert("tr0", Register::TR0);
		let _ = h.insert("tr1", Register::TR1);
		let _ = h.insert("tr2", Register::TR2);
		let _ = h.insert("tr3", Register::TR3);
		let _ = h.insert("tr4", Register::TR4);
		let _ = h.insert("tr5", Register::TR5);
		let _ = h.insert("tr6", Register::TR6);
		let _ = h.insert("tr7", Register::TR7);
		let _ = h.insert("tmm0", Register::TMM0);
		let _ = h.insert("tmm1", Register::TMM1);
		let _ = h.insert("tmm2", Register::TMM2);
		let _ = h.insert("tmm3", Register::TMM3);
		let _ = h.insert("tmm4", Register::TMM4);
		let _ = h.insert("tmm5", Register::TMM5);
		let _ = h.insert("tmm6", Register::TMM6);
		let _ = h.insert("tmm7", Register::TMM7);
		let _ = h.insert("dontusef9", Register::DontUseF9);
		let _ = h.insert("dontusefa", Register::DontUseFA);
		let _ = h.insert("dontusefb", Register::DontUseFB);
		let _ = h.insert("dontusefc", Register::DontUseFC);
		let _ = h.insert("dontusefd", Register::DontUseFD);
		let _ = h.insert("dontusefe", Register::DontUseFE);
		let _ = h.insert("dontuseff", Register::DontUseFF);
		// GENERATOR-END: RegisterHash
		h
	};
}
