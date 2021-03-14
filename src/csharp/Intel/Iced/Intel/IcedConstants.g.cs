// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

// ⚠️This file was generated by GENERATOR!🦹‍♂️

#nullable enable

namespace Iced.Intel {
	static class IcedConstants {
		internal const int MaxOpCount = 5;
		internal const int MaxInstructionLength = 15;
		internal const int RegisterBits = 8;
		internal const Register VMM_first = Register.ZMM0;
		internal const Register VMM_last = Register.ZMM31;
		internal const int VMM_count = 32;
		internal const Register XMM_last = Register.XMM31;
		internal const Register YMM_last = Register.YMM31;
		internal const Register ZMM_last = Register.ZMM31;
		internal const Register TMM_last = Register.TMM7;
		internal const int MaxCpuidFeatureInternalValues = 178;
		internal const MemorySize FirstBroadcastMemorySize = MemorySize.Broadcast64_UInt32;
		internal const int CC_a_EnumCount = 2;
		internal const int CC_ae_EnumCount = 3;
		internal const int CC_b_EnumCount = 3;
		internal const int CC_be_EnumCount = 2;
		internal const int CC_e_EnumCount = 2;
		internal const int CC_g_EnumCount = 2;
		internal const int CC_ge_EnumCount = 2;
		internal const int CC_l_EnumCount = 2;
		internal const int CC_le_EnumCount = 2;
		internal const int CC_ne_EnumCount = 2;
		internal const int CC_np_EnumCount = 2;
		internal const int CC_p_EnumCount = 2;
		internal const int CodeEnumCount = 4323;
		internal const int CodeSizeEnumCount = 4;
		internal const int ConditionCodeEnumCount = 17;
		internal const int CpuidFeatureEnumCount = 158;
		internal const int DecoderErrorEnumCount = 3;
		internal const int DecoratorKindEnumCount = 4;
		internal const int EncodingKindEnumCount = 5;
		internal const int FlowControlEnumCount = 10;
		internal const int FormatterSyntaxEnumCount = 4;
		internal const int FormatterTextKindEnumCount = 16;
		internal const int MandatoryPrefixEnumCount = 5;
		internal const int MemorySizeEnumCount = 141;
		internal const int MemorySizeOptionsEnumCount = 4;
		internal const int MnemonicEnumCount = 1632;
		internal const int NumberBaseEnumCount = 4;
		internal const int NumberKindEnumCount = 8;
		internal const int OpAccessEnumCount = 8;
		internal const int OpCodeOperandKindEnumCount = 109;
		internal const int OpCodeTableKindEnumCount = 7;
		internal const int OpKindEnumCount = 26;
		internal const int PrefixKindEnumCount = 18;
		internal const int RegisterEnumCount = 249;
		internal const int RelocKindEnumCount = 1;
		internal const int RepPrefixKindEnumCount = 3;
		internal const int RoundingControlEnumCount = 5;
		internal const int TupleTypeEnumCount = 14;
	}
}
