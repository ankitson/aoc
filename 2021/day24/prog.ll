; ModuleID = 'prog.cpp'
source_filename = "prog.cpp"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

%"class.std::ios_base::Init" = type { i8 }
%"class.std::basic_istream" = type { i32 (...)**, i64, %"class.std::basic_ios" }
%"class.std::basic_ios" = type { %"class.std::ios_base", %"class.std::basic_ostream"*, i8, i8, %"class.std::basic_streambuf"*, %"class.std::ctype"*, %"class.std::num_put"*, %"class.std::num_get"* }
%"class.std::ios_base" = type { i32 (...)**, i64, i64, i32, i32, i32, %"struct.std::ios_base::_Callback_list"*, %"struct.std::ios_base::_Words", [8 x %"struct.std::ios_base::_Words"], i32, %"struct.std::ios_base::_Words"*, %"class.std::locale" }
%"struct.std::ios_base::_Callback_list" = type { %"struct.std::ios_base::_Callback_list"*, void (i32, %"class.std::ios_base"*, i32)*, i32, i32 }
%"struct.std::ios_base::_Words" = type { i8*, i64 }
%"class.std::locale" = type { %"class.std::locale::_Impl"* }
%"class.std::locale::_Impl" = type { i32, %"class.std::locale::facet"**, i64, %"class.std::locale::facet"**, i8** }
%"class.std::locale::facet" = type <{ i32 (...)**, i32, [4 x i8] }>
%"class.std::basic_ostream" = type { i32 (...)**, %"class.std::basic_ios" }
%"class.std::basic_streambuf" = type { i32 (...)**, i8*, i8*, i8*, i8*, i8*, i8*, %"class.std::locale" }
%"class.std::ctype" = type <{ %"class.std::locale::facet.base", [4 x i8], %struct.__locale_struct*, i8, [7 x i8], i32*, i32*, i16*, i8, [256 x i8], [256 x i8], i8, [6 x i8] }>
%"class.std::locale::facet.base" = type <{ i32 (...)**, i32 }>
%struct.__locale_struct = type { [13 x %struct.__locale_data*], i16*, i32*, i32*, [13 x i8*] }
%struct.__locale_data = type opaque
%"class.std::num_put" = type { %"class.std::locale::facet.base", [4 x i8] }
%"class.std::num_get" = type { %"class.std::locale::facet.base", [4 x i8] }

@_ZStL8__ioinit = internal global %"class.std::ios_base::Init" zeroinitializer, align 1
@__dso_handle = external hidden global i8
@_ZSt3cin = external dso_local global %"class.std::basic_istream", align 8
@_ZSt4cout = external dso_local global %"class.std::basic_ostream", align 8
@.str = private unnamed_addr constant [9 x i8] c"Finished\00", align 1
@.str.1 = private unnamed_addr constant [6 x i8] c"vars=\00", align 1
@.str.2 = private unnamed_addr constant [2 x i8] c",\00", align 1
@llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 65535, void ()* @_GLOBAL__sub_I_prog.cpp, i8* null }]

declare dso_local void @_ZNSt8ios_base4InitC1Ev(%"class.std::ios_base::Init"* nonnull align 1 dereferenceable(1)) unnamed_addr #0

; Function Attrs: nounwind
declare dso_local void @_ZNSt8ios_base4InitD1Ev(%"class.std::ios_base::Init"* nonnull align 1 dereferenceable(1)) unnamed_addr #1

; Function Attrs: nofree nounwind
declare dso_local i32 @__cxa_atexit(void (i8*)*, i8*, i8*) local_unnamed_addr #2

; Function Attrs: mustprogress norecurse uwtable
define dso_local i32 @main() local_unnamed_addr #3 !dbg !6 {
entry:
  %W = alloca i64, align 8
  %0 = bitcast i64* %W to i8*, !dbg !9
  call void @llvm.lifetime.start.p0i8(i64 8, i8* nonnull %0) #7, !dbg !9
  store i64 0, i64* %W, align 8, !dbg !10, !tbaa !11
  %call.i = call nonnull align 8 dereferenceable(16) %"class.std::basic_istream"* @_ZNSi10_M_extractIlEERSiRT_(%"class.std::basic_istream"* nonnull align 8 dereferenceable(16) @_ZSt3cin, i64* nonnull align 8 dereferenceable(8) %W), !dbg !15
  %1 = load i64, i64* %W, align 8, !dbg !19, !tbaa !11
  %cmp.not = icmp eq i64 %1, 10, !dbg !20
  %add11 = add nsw i64 %1, 10, !dbg !21
  %mul12 = select i1 %cmp.not, i64 0, i64 %add11, !dbg !22
  %call.i602 = call nonnull align 8 dereferenceable(16) %"class.std::basic_istream"* @_ZNSi10_M_extractIlEERSiRT_(%"class.std::basic_istream"* nonnull align 8 dereferenceable(16) @_ZSt3cin, i64* nonnull align 8 dereferenceable(8) %W), !dbg !23
  %rem17 = srem i64 %mul12, 26, !dbg !25
  %add19 = add nsw i64 %rem17, 13, !dbg !26
  %2 = load i64, i64* %W, align 8, !dbg !27, !tbaa !11
  %cmp20.not = icmp eq i64 %add19, %2, !dbg !28
  %add27 = select i1 %cmp20.not, i64 1, i64 26, !dbg !29
  %mul28 = mul nsw i64 %add27, %mul12, !dbg !30
  %add31 = add nsw i64 %2, 5, !dbg !31
  %mul32 = select i1 %cmp20.not, i64 0, i64 %add31, !dbg !32
  %add33 = add nsw i64 %mul28, %mul32, !dbg !33
  %call.i603 = call nonnull align 8 dereferenceable(16) %"class.std::basic_istream"* @_ZNSi10_M_extractIlEERSiRT_(%"class.std::basic_istream"* nonnull align 8 dereferenceable(16) @_ZSt3cin, i64* nonnull align 8 dereferenceable(8) %W), !dbg !34
  %rem37 = srem i64 %add33, 26, !dbg !36
  %add39 = add nsw i64 %rem37, 15, !dbg !37
  %3 = load i64, i64* %W, align 8, !dbg !38, !tbaa !11
  %cmp40.not = icmp eq i64 %add39, %3, !dbg !39
  %add47 = select i1 %cmp40.not, i64 1, i64 26, !dbg !40
  %mul48 = mul nsw i64 %add47, %add33, !dbg !41
  %add51 = add nsw i64 %3, 12, !dbg !42
  %mul52 = select i1 %cmp40.not, i64 0, i64 %add51, !dbg !43
  %add53 = add nsw i64 %mul48, %mul52, !dbg !44
  %call.i604 = call nonnull align 8 dereferenceable(16) %"class.std::basic_istream"* @_ZNSi10_M_extractIlEERSiRT_(%"class.std::basic_istream"* nonnull align 8 dereferenceable(16) @_ZSt3cin, i64* nonnull align 8 dereferenceable(8) %W), !dbg !45
  %rem57 = srem i64 %add53, 26, !dbg !47
  %div58 = sdiv i64 %add53, 26, !dbg !48
  %add59 = add nsw i64 %rem57, -12, !dbg !49
  %4 = load i64, i64* %W, align 8, !dbg !50, !tbaa !11
  %cmp60.not = icmp eq i64 %add59, %4, !dbg !51
  %add67 = select i1 %cmp60.not, i64 1, i64 26, !dbg !52
  %mul68 = mul nsw i64 %add67, %div58, !dbg !53
  %add71 = add nsw i64 %4, 12, !dbg !54
  %mul72 = select i1 %cmp60.not, i64 0, i64 %add71, !dbg !55
  %add73 = add nsw i64 %mul68, %mul72, !dbg !56
  %call.i605 = call nonnull align 8 dereferenceable(16) %"class.std::basic_istream"* @_ZNSi10_M_extractIlEERSiRT_(%"class.std::basic_istream"* nonnull align 8 dereferenceable(16) @_ZSt3cin, i64* nonnull align 8 dereferenceable(8) %W), !dbg !57
  %rem77 = srem i64 %add73, 26, !dbg !59
  %add79 = add nsw i64 %rem77, 14, !dbg !60
  %5 = load i64, i64* %W, align 8, !dbg !61, !tbaa !11
  %cmp80.not = icmp eq i64 %add79, %5, !dbg !62
  %add87 = select i1 %cmp80.not, i64 1, i64 26, !dbg !63
  %mul88 = mul nsw i64 %add87, %add73, !dbg !64
  %add91 = add nsw i64 %5, 6, !dbg !65
  %mul92 = select i1 %cmp80.not, i64 0, i64 %add91, !dbg !66
  %add93 = add nsw i64 %mul88, %mul92, !dbg !67
  %call.i606 = call nonnull align 8 dereferenceable(16) %"class.std::basic_istream"* @_ZNSi10_M_extractIlEERSiRT_(%"class.std::basic_istream"* nonnull align 8 dereferenceable(16) @_ZSt3cin, i64* nonnull align 8 dereferenceable(8) %W), !dbg !68
  %rem97 = srem i64 %add93, 26, !dbg !70
  %div98 = sdiv i64 %add93, 26, !dbg !71
  %add99 = add nsw i64 %rem97, -2, !dbg !72
  %6 = load i64, i64* %W, align 8, !dbg !73, !tbaa !11
  %cmp100.not = icmp eq i64 %add99, %6, !dbg !74
  %add107 = select i1 %cmp100.not, i64 1, i64 26, !dbg !75
  %mul108 = mul nsw i64 %add107, %div98, !dbg !76
  %add111 = add nsw i64 %6, 4, !dbg !77
  %mul112 = select i1 %cmp100.not, i64 0, i64 %add111, !dbg !78
  %add113 = add nsw i64 %mul108, %mul112, !dbg !79
  %call.i607 = call nonnull align 8 dereferenceable(16) %"class.std::basic_istream"* @_ZNSi10_M_extractIlEERSiRT_(%"class.std::basic_istream"* nonnull align 8 dereferenceable(16) @_ZSt3cin, i64* nonnull align 8 dereferenceable(8) %W), !dbg !80
  %rem117 = srem i64 %add113, 26, !dbg !82
  %add119 = add nsw i64 %rem117, 13, !dbg !83
  %7 = load i64, i64* %W, align 8, !dbg !84, !tbaa !11
  %cmp120.not = icmp eq i64 %add119, %7, !dbg !85
  %add127 = select i1 %cmp120.not, i64 1, i64 26, !dbg !86
  %mul128 = mul nsw i64 %add127, %add113, !dbg !87
  %add131 = add nsw i64 %7, 15, !dbg !88
  %mul132 = select i1 %cmp120.not, i64 0, i64 %add131, !dbg !89
  %add133 = add nsw i64 %mul128, %mul132, !dbg !90
  %call.i608 = call nonnull align 8 dereferenceable(16) %"class.std::basic_istream"* @_ZNSi10_M_extractIlEERSiRT_(%"class.std::basic_istream"* nonnull align 8 dereferenceable(16) @_ZSt3cin, i64* nonnull align 8 dereferenceable(8) %W), !dbg !91
  %rem137 = srem i64 %add133, 26, !dbg !93
  %div138 = sdiv i64 %add133, 26, !dbg !94
  %add139 = add nsw i64 %rem137, -12, !dbg !95
  %8 = load i64, i64* %W, align 8, !dbg !96, !tbaa !11
  %cmp140.not = icmp eq i64 %add139, %8, !dbg !97
  %add147 = select i1 %cmp140.not, i64 1, i64 26, !dbg !98
  %mul148 = mul nsw i64 %add147, %div138, !dbg !99
  %add151 = add nsw i64 %8, 3, !dbg !100
  %mul152 = select i1 %cmp140.not, i64 0, i64 %add151, !dbg !101
  %add153 = add nsw i64 %mul148, %mul152, !dbg !102
  %call.i609 = call nonnull align 8 dereferenceable(16) %"class.std::basic_istream"* @_ZNSi10_M_extractIlEERSiRT_(%"class.std::basic_istream"* nonnull align 8 dereferenceable(16) @_ZSt3cin, i64* nonnull align 8 dereferenceable(8) %W), !dbg !103
  %rem157 = srem i64 %add153, 26, !dbg !105
  %add159 = add nsw i64 %rem157, 15, !dbg !106
  %9 = load i64, i64* %W, align 8, !dbg !107, !tbaa !11
  %cmp160.not = icmp eq i64 %add159, %9, !dbg !108
  %add167 = select i1 %cmp160.not, i64 1, i64 26, !dbg !109
  %mul168 = mul nsw i64 %add167, %add153, !dbg !110
  %add171 = add nsw i64 %9, 7, !dbg !111
  %mul172 = select i1 %cmp160.not, i64 0, i64 %add171, !dbg !112
  %add173 = add nsw i64 %mul168, %mul172, !dbg !113
  %call.i610 = call nonnull align 8 dereferenceable(16) %"class.std::basic_istream"* @_ZNSi10_M_extractIlEERSiRT_(%"class.std::basic_istream"* nonnull align 8 dereferenceable(16) @_ZSt3cin, i64* nonnull align 8 dereferenceable(8) %W), !dbg !114
  %rem177 = srem i64 %add173, 26, !dbg !116
  %add179 = add nsw i64 %rem177, 11, !dbg !117
  %10 = load i64, i64* %W, align 8, !dbg !118, !tbaa !11
  %cmp180.not = icmp eq i64 %add179, %10, !dbg !119
  %add187 = select i1 %cmp180.not, i64 1, i64 26, !dbg !120
  %mul188 = mul nsw i64 %add187, %add173, !dbg !121
  %add191 = add nsw i64 %10, 11, !dbg !122
  %mul192 = select i1 %cmp180.not, i64 0, i64 %add191, !dbg !123
  %add193 = add nsw i64 %mul188, %mul192, !dbg !124
  %call.i611 = call nonnull align 8 dereferenceable(16) %"class.std::basic_istream"* @_ZNSi10_M_extractIlEERSiRT_(%"class.std::basic_istream"* nonnull align 8 dereferenceable(16) @_ZSt3cin, i64* nonnull align 8 dereferenceable(8) %W), !dbg !125
  %rem197 = srem i64 %add193, 26, !dbg !127
  %div198 = sdiv i64 %add193, 26, !dbg !128
  %add199 = add nsw i64 %rem197, -3, !dbg !129
  %11 = load i64, i64* %W, align 8, !dbg !130, !tbaa !11
  %cmp200.not = icmp eq i64 %add199, %11, !dbg !131
  %add207 = select i1 %cmp200.not, i64 1, i64 26, !dbg !132
  %mul208 = mul nsw i64 %add207, %div198, !dbg !133
  %add211 = add nsw i64 %11, 2, !dbg !134
  %mul212 = select i1 %cmp200.not, i64 0, i64 %add211, !dbg !135
  %add213 = add nsw i64 %mul208, %mul212, !dbg !136
  %call.i612 = call nonnull align 8 dereferenceable(16) %"class.std::basic_istream"* @_ZNSi10_M_extractIlEERSiRT_(%"class.std::basic_istream"* nonnull align 8 dereferenceable(16) @_ZSt3cin, i64* nonnull align 8 dereferenceable(8) %W), !dbg !137
  %rem217 = srem i64 %add213, 26, !dbg !139
  %div218 = sdiv i64 %add213, 26, !dbg !140
  %add219 = add nsw i64 %rem217, -13, !dbg !141
  %12 = load i64, i64* %W, align 8, !dbg !142, !tbaa !11
  %cmp220.not = icmp eq i64 %add219, %12, !dbg !143
  %add227 = select i1 %cmp220.not, i64 1, i64 26, !dbg !144
  %mul228 = mul nsw i64 %add227, %div218, !dbg !145
  %add231 = add nsw i64 %12, 12, !dbg !146
  %mul232 = select i1 %cmp220.not, i64 0, i64 %add231, !dbg !147
  %add233 = add nsw i64 %mul228, %mul232, !dbg !148
  %call.i613 = call nonnull align 8 dereferenceable(16) %"class.std::basic_istream"* @_ZNSi10_M_extractIlEERSiRT_(%"class.std::basic_istream"* nonnull align 8 dereferenceable(16) @_ZSt3cin, i64* nonnull align 8 dereferenceable(8) %W), !dbg !149
  %rem237 = srem i64 %add233, 26, !dbg !151
  %div238 = sdiv i64 %add233, 26, !dbg !152
  %add239 = add nsw i64 %rem237, -12, !dbg !153
  %13 = load i64, i64* %W, align 8, !dbg !154, !tbaa !11
  %cmp240.not = icmp eq i64 %add239, %13, !dbg !155
  %add247 = select i1 %cmp240.not, i64 1, i64 26, !dbg !156
  %mul248 = mul nsw i64 %add247, %div238, !dbg !157
  %add251 = add nsw i64 %13, 4, !dbg !158
  %mul252 = select i1 %cmp240.not, i64 0, i64 %add251, !dbg !159
  %add253 = add nsw i64 %mul248, %mul252, !dbg !160
  %call.i614 = call nonnull align 8 dereferenceable(16) %"class.std::basic_istream"* @_ZNSi10_M_extractIlEERSiRT_(%"class.std::basic_istream"* nonnull align 8 dereferenceable(16) @_ZSt3cin, i64* nonnull align 8 dereferenceable(8) %W), !dbg !161
  %rem257 = srem i64 %add253, 26, !dbg !163
  %div258 = sdiv i64 %add253, 26, !dbg !164
  %add259 = add nsw i64 %rem257, -13, !dbg !165
  %14 = load i64, i64* %W, align 8, !dbg !166, !tbaa !11
  %cmp260 = icmp ne i64 %add259, %14, !dbg !167
  %conv263 = zext i1 %cmp260 to i64, !dbg !168
  %add267 = select i1 %cmp260, i64 26, i64 1, !dbg !169
  %mul268 = mul nsw i64 %add267, %div258, !dbg !170
  %add271 = add nsw i64 %14, 11, !dbg !171
  %mul272 = select i1 %cmp260, i64 %add271, i64 0, !dbg !172
  %add273 = add nsw i64 %mul268, %mul272, !dbg !173
  %call1.i = call nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @_ZSt16__ostream_insertIcSt11char_traitsIcEERSt13basic_ostreamIT_T0_ES6_PKS3_l(%"class.std::basic_ostream"* nonnull align 8 dereferenceable(8) @_ZSt4cout, i8* nonnull getelementptr inbounds ([9 x i8], [9 x i8]* @.str, i64 0, i64 0), i64 8), !dbg !174
  %vtable.i = load i8*, i8** bitcast (%"class.std::basic_ostream"* @_ZSt4cout to i8**), align 8, !dbg !178, !tbaa !183
  %vbase.offset.ptr.i = getelementptr i8, i8* %vtable.i, i64 -24, !dbg !178
  %15 = bitcast i8* %vbase.offset.ptr.i to i64*, !dbg !178
  %vbase.offset.i = load i64, i64* %15, align 8, !dbg !178
  %add.ptr.i = getelementptr inbounds i8, i8* bitcast (%"class.std::basic_ostream"* @_ZSt4cout to i8*), i64 %vbase.offset.i, !dbg !178
  %_M_ctype.i.i = getelementptr inbounds i8, i8* %add.ptr.i, i64 240, !dbg !185
  %16 = bitcast i8* %_M_ctype.i.i to %"class.std::ctype"**, !dbg !185
  %17 = load %"class.std::ctype"*, %"class.std::ctype"** %16, align 8, !dbg !185, !tbaa !189
  %tobool.not.i.i.i = icmp eq %"class.std::ctype"* %17, null, !dbg !193
  br i1 %tobool.not.i.i.i, label %if.then.i.i.i, label %_ZSt13__check_facetISt5ctypeIcEERKT_PS3_.exit.i.i, !dbg !196

if.then.i.i.i:                                    ; preds = %entry
  call void @_ZSt16__throw_bad_castv() #8, !dbg !197
  unreachable, !dbg !197

_ZSt13__check_facetISt5ctypeIcEERKT_PS3_.exit.i.i: ; preds = %entry
  %_M_widen_ok.i.i.i = getelementptr inbounds %"class.std::ctype", %"class.std::ctype"* %17, i64 0, i32 8, !dbg !198
  %18 = load i8, i8* %_M_widen_ok.i.i.i, align 8, !dbg !198, !tbaa !202
  %tobool.not.i3.i.i = icmp eq i8 %18, 0, !dbg !198
  br i1 %tobool.not.i3.i.i, label %if.end.i.i.i, label %if.then.i4.i.i, !dbg !198

if.then.i4.i.i:                                   ; preds = %_ZSt13__check_facetISt5ctypeIcEERKT_PS3_.exit.i.i
  %arrayidx.i.i.i = getelementptr inbounds %"class.std::ctype", %"class.std::ctype"* %17, i64 0, i32 9, i64 10, !dbg !204
  %19 = load i8, i8* %arrayidx.i.i.i, align 1, !dbg !204, !tbaa !205
  br label %_ZSt4endlIcSt11char_traitsIcEERSt13basic_ostreamIT_T0_ES6_.exit, !dbg !206

if.end.i.i.i:                                     ; preds = %_ZSt13__check_facetISt5ctypeIcEERKT_PS3_.exit.i.i
  call void @_ZNKSt5ctypeIcE13_M_widen_initEv(%"class.std::ctype"* nonnull align 8 dereferenceable(570) %17), !dbg !207
  %20 = bitcast %"class.std::ctype"* %17 to i8 (%"class.std::ctype"*, i8)***, !dbg !208
  %vtable.i.i.i = load i8 (%"class.std::ctype"*, i8)**, i8 (%"class.std::ctype"*, i8)*** %20, align 8, !dbg !208, !tbaa !183
  %vfn.i.i.i = getelementptr inbounds i8 (%"class.std::ctype"*, i8)*, i8 (%"class.std::ctype"*, i8)** %vtable.i.i.i, i64 6, !dbg !208
  %21 = load i8 (%"class.std::ctype"*, i8)*, i8 (%"class.std::ctype"*, i8)** %vfn.i.i.i, align 8, !dbg !208
  %call.i.i.i = call signext i8 %21(%"class.std::ctype"* nonnull align 8 dereferenceable(570) %17, i8 signext 10), !dbg !208
  br label %_ZSt4endlIcSt11char_traitsIcEERSt13basic_ostreamIT_T0_ES6_.exit, !dbg !209

_ZSt4endlIcSt11char_traitsIcEERSt13basic_ostreamIT_T0_ES6_.exit: ; preds = %if.then.i4.i.i, %if.end.i.i.i
  %retval.0.i.i.i = phi i8 [ %19, %if.then.i4.i.i ], [ %call.i.i.i, %if.end.i.i.i ], !dbg !210
  %call1.i631 = call nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @_ZNSo3putEc(%"class.std::basic_ostream"* nonnull align 8 dereferenceable(8) @_ZSt4cout, i8 signext %retval.0.i.i.i), !dbg !211
  %call.i.i632 = call nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @_ZNSo5flushEv(%"class.std::basic_ostream"* nonnull align 8 dereferenceable(8) %call1.i631), !dbg !212
  %call1.i617 = call nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @_ZSt16__ostream_insertIcSt11char_traitsIcEERSt13basic_ostreamIT_T0_ES6_PKS3_l(%"class.std::basic_ostream"* nonnull align 8 dereferenceable(8) @_ZSt4cout, i8* nonnull getelementptr inbounds ([6 x i8], [6 x i8]* @.str.1, i64 0, i64 0), i64 5), !dbg !215
  %22 = load i64, i64* %W, align 8, !dbg !217, !tbaa !11
  %call.i618 = call nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @_ZNSo9_M_insertIlEERSoT_(%"class.std::basic_ostream"* nonnull align 8 dereferenceable(8) @_ZSt4cout, i64 %22), !dbg !218
  %call1.i620 = call nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @_ZSt16__ostream_insertIcSt11char_traitsIcEERSt13basic_ostreamIT_T0_ES6_PKS3_l(%"class.std::basic_ostream"* nonnull align 8 dereferenceable(8) %call.i618, i8* nonnull getelementptr inbounds ([2 x i8], [2 x i8]* @.str.2, i64 0, i64 0), i64 1), !dbg !221
  %call1.i622 = call nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @_ZSt16__ostream_insertIcSt11char_traitsIcEERSt13basic_ostreamIT_T0_ES6_PKS3_l(%"class.std::basic_ostream"* nonnull align 8 dereferenceable(8) %call.i618, i8* nonnull getelementptr inbounds ([2 x i8], [2 x i8]* @.str.2, i64 0, i64 0), i64 1), !dbg !223
  %call.i623 = call nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @_ZNSo9_M_insertIlEERSoT_(%"class.std::basic_ostream"* nonnull align 8 dereferenceable(8) %call.i618, i64 %conv263), !dbg !225
  %call1.i625 = call nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @_ZSt16__ostream_insertIcSt11char_traitsIcEERSt13basic_ostreamIT_T0_ES6_PKS3_l(%"class.std::basic_ostream"* nonnull align 8 dereferenceable(8) %call.i623, i8* nonnull getelementptr inbounds ([2 x i8], [2 x i8]* @.str.2, i64 0, i64 0), i64 1), !dbg !227
  %call.i626 = call nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @_ZNSo9_M_insertIlEERSoT_(%"class.std::basic_ostream"* nonnull align 8 dereferenceable(8) %call.i623, i64 %mul272), !dbg !229
  %call1.i628 = call nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @_ZSt16__ostream_insertIcSt11char_traitsIcEERSt13basic_ostreamIT_T0_ES6_PKS3_l(%"class.std::basic_ostream"* nonnull align 8 dereferenceable(8) %call.i626, i8* nonnull getelementptr inbounds ([2 x i8], [2 x i8]* @.str.2, i64 0, i64 0), i64 1), !dbg !231
  %call.i629 = call nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @_ZNSo9_M_insertIlEERSoT_(%"class.std::basic_ostream"* nonnull align 8 dereferenceable(8) %call.i626, i64 %add273), !dbg !233
  %23 = bitcast %"class.std::basic_ostream"* %call.i629 to i8**, !dbg !235
  %vtable.i633 = load i8*, i8** %23, align 8, !dbg !235, !tbaa !183
  %vbase.offset.ptr.i634 = getelementptr i8, i8* %vtable.i633, i64 -24, !dbg !235
  %24 = bitcast i8* %vbase.offset.ptr.i634 to i64*, !dbg !235
  %vbase.offset.i635 = load i64, i64* %24, align 8, !dbg !235
  %25 = bitcast %"class.std::basic_ostream"* %call.i629 to i8*, !dbg !235
  %add.ptr.i636 = getelementptr inbounds i8, i8* %25, i64 %vbase.offset.i635, !dbg !235
  %_M_ctype.i.i637 = getelementptr inbounds i8, i8* %add.ptr.i636, i64 240, !dbg !238
  %26 = bitcast i8* %_M_ctype.i.i637 to %"class.std::ctype"**, !dbg !238
  %27 = load %"class.std::ctype"*, %"class.std::ctype"** %26, align 8, !dbg !238, !tbaa !189
  %tobool.not.i.i.i638 = icmp eq %"class.std::ctype"* %27, null, !dbg !240
  br i1 %tobool.not.i.i.i638, label %if.then.i.i.i639, label %_ZSt13__check_facetISt5ctypeIcEERKT_PS3_.exit.i.i642, !dbg !242

if.then.i.i.i639:                                 ; preds = %_ZSt4endlIcSt11char_traitsIcEERSt13basic_ostreamIT_T0_ES6_.exit
  call void @_ZSt16__throw_bad_castv() #8, !dbg !243
  unreachable, !dbg !243

_ZSt13__check_facetISt5ctypeIcEERKT_PS3_.exit.i.i642: ; preds = %_ZSt4endlIcSt11char_traitsIcEERSt13basic_ostreamIT_T0_ES6_.exit
  %_M_widen_ok.i.i.i640 = getelementptr inbounds %"class.std::ctype", %"class.std::ctype"* %27, i64 0, i32 8, !dbg !244
  %28 = load i8, i8* %_M_widen_ok.i.i.i640, align 8, !dbg !244, !tbaa !202
  %tobool.not.i3.i.i641 = icmp eq i8 %28, 0, !dbg !244
  br i1 %tobool.not.i3.i.i641, label %if.end.i.i.i648, label %if.then.i4.i.i644, !dbg !244

if.then.i4.i.i644:                                ; preds = %_ZSt13__check_facetISt5ctypeIcEERKT_PS3_.exit.i.i642
  %arrayidx.i.i.i643 = getelementptr inbounds %"class.std::ctype", %"class.std::ctype"* %27, i64 0, i32 9, i64 10, !dbg !246
  %29 = load i8, i8* %arrayidx.i.i.i643, align 1, !dbg !246, !tbaa !205
  br label %_ZSt4endlIcSt11char_traitsIcEERSt13basic_ostreamIT_T0_ES6_.exit652, !dbg !247

if.end.i.i.i648:                                  ; preds = %_ZSt13__check_facetISt5ctypeIcEERKT_PS3_.exit.i.i642
  call void @_ZNKSt5ctypeIcE13_M_widen_initEv(%"class.std::ctype"* nonnull align 8 dereferenceable(570) %27), !dbg !248
  %30 = bitcast %"class.std::ctype"* %27 to i8 (%"class.std::ctype"*, i8)***, !dbg !249
  %vtable.i.i.i645 = load i8 (%"class.std::ctype"*, i8)**, i8 (%"class.std::ctype"*, i8)*** %30, align 8, !dbg !249, !tbaa !183
  %vfn.i.i.i646 = getelementptr inbounds i8 (%"class.std::ctype"*, i8)*, i8 (%"class.std::ctype"*, i8)** %vtable.i.i.i645, i64 6, !dbg !249
  %31 = load i8 (%"class.std::ctype"*, i8)*, i8 (%"class.std::ctype"*, i8)** %vfn.i.i.i646, align 8, !dbg !249
  %call.i.i.i647 = call signext i8 %31(%"class.std::ctype"* nonnull align 8 dereferenceable(570) %27, i8 signext 10), !dbg !249
  br label %_ZSt4endlIcSt11char_traitsIcEERSt13basic_ostreamIT_T0_ES6_.exit652, !dbg !250

_ZSt4endlIcSt11char_traitsIcEERSt13basic_ostreamIT_T0_ES6_.exit652: ; preds = %if.then.i4.i.i644, %if.end.i.i.i648
  %retval.0.i.i.i649 = phi i8 [ %29, %if.then.i4.i.i644 ], [ %call.i.i.i647, %if.end.i.i.i648 ], !dbg !251
  %call1.i650 = call nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @_ZNSo3putEc(%"class.std::basic_ostream"* nonnull align 8 dereferenceable(8) %call.i629, i8 signext %retval.0.i.i.i649), !dbg !252
  %call.i.i651 = call nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @_ZNSo5flushEv(%"class.std::basic_ostream"* nonnull align 8 dereferenceable(8) %call1.i650), !dbg !253
  %cmp286 = icmp eq i64 %add273, 0, !dbg !255
  %conv287 = zext i1 %cmp286 to i32, !dbg !256
  call void @llvm.lifetime.end.p0i8(i64 8, i8* nonnull %0) #7, !dbg !257
  ret i32 %conv287, !dbg !258
}

; Function Attrs: argmemonly mustprogress nofree nosync nounwind willreturn
declare void @llvm.lifetime.start.p0i8(i64 immarg, i8* nocapture) #4

; Function Attrs: argmemonly mustprogress nofree nosync nounwind willreturn
declare void @llvm.lifetime.end.p0i8(i64 immarg, i8* nocapture) #4

declare dso_local nonnull align 8 dereferenceable(16) %"class.std::basic_istream"* @_ZNSi10_M_extractIlEERSiRT_(%"class.std::basic_istream"* nonnull align 8 dereferenceable(16), i64* nonnull align 8 dereferenceable(8)) local_unnamed_addr #0

declare dso_local nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @_ZSt16__ostream_insertIcSt11char_traitsIcEERSt13basic_ostreamIT_T0_ES6_PKS3_l(%"class.std::basic_ostream"* nonnull align 8 dereferenceable(8), i8*, i64) local_unnamed_addr #0

declare dso_local nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @_ZNSo3putEc(%"class.std::basic_ostream"* nonnull align 8 dereferenceable(8), i8 signext) local_unnamed_addr #0

declare dso_local nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @_ZNSo5flushEv(%"class.std::basic_ostream"* nonnull align 8 dereferenceable(8)) local_unnamed_addr #0

; Function Attrs: noreturn
declare dso_local void @_ZSt16__throw_bad_castv() local_unnamed_addr #5

declare dso_local void @_ZNKSt5ctypeIcE13_M_widen_initEv(%"class.std::ctype"* nonnull align 8 dereferenceable(570)) local_unnamed_addr #0

declare dso_local nonnull align 8 dereferenceable(8) %"class.std::basic_ostream"* @_ZNSo9_M_insertIlEERSoT_(%"class.std::basic_ostream"* nonnull align 8 dereferenceable(8), i64) local_unnamed_addr #0

; Function Attrs: uwtable
define internal void @_GLOBAL__sub_I_prog.cpp() #6 section ".text.startup" !dbg !259 {
entry:
  tail call void @_ZNSt8ios_base4InitC1Ev(%"class.std::ios_base::Init"* nonnull align 1 dereferenceable(1) @_ZStL8__ioinit), !dbg !260
  %0 = tail call i32 @__cxa_atexit(void (i8*)* bitcast (void (%"class.std::ios_base::Init"*)* @_ZNSt8ios_base4InitD1Ev to void (i8*)*), i8* getelementptr inbounds (%"class.std::ios_base::Init", %"class.std::ios_base::Init"* @_ZStL8__ioinit, i64 0, i32 0), i8* nonnull @__dso_handle) #7, !dbg !265
  ret void
}

attributes #0 = { "frame-pointer"="none" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { nounwind "frame-pointer"="none" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #2 = { nofree nounwind }
attributes #3 = { mustprogress norecurse uwtable "frame-pointer"="none" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #4 = { argmemonly mustprogress nofree nosync nounwind willreturn }
attributes #5 = { noreturn "frame-pointer"="none" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #6 = { uwtable "frame-pointer"="none" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #7 = { nounwind }
attributes #8 = { noreturn }

!llvm.dbg.cu = !{!0}
!llvm.module.flags = !{!2, !3, !4}
!llvm.ident = !{!5}

!0 = distinct !DICompileUnit(language: DW_LANG_C_plus_plus_14, file: !1, producer: "Ubuntu clang version 14.0.0-++20211225104508+34558b039b3b-1~exp1~20211225104603.190", isOptimized: true, runtimeVersion: 0, emissionKind: LineTablesOnly, splitDebugInlining: false, nameTableKind: None)
!1 = !DIFile(filename: "prog.cpp", directory: "/home/ankit/code/aoc-2021/day24")
!2 = !{i32 2, !"Debug Info Version", i32 3}
!3 = !{i32 1, !"wchar_size", i32 4}
!4 = !{i32 7, !"uwtable", i32 1}
!5 = !{!"Ubuntu clang version 14.0.0-++20211225104508+34558b039b3b-1~exp1~20211225104603.190"}
!6 = distinct !DISubprogram(name: "main", scope: !1, file: !1, line: 6, type: !7, scopeLine: 6, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition | DISPFlagOptimized, unit: !0, retainedNodes: !8)
!7 = !DISubroutineType(types: !8)
!8 = !{}
!9 = !DILocation(line: 7, column: 3, scope: !6)
!10 = !DILocation(line: 7, column: 8, scope: !6)
!11 = !{!12, !12, i64 0}
!12 = !{!"long", !13, i64 0}
!13 = !{!"omnipotent char", !14, i64 0}
!14 = !{!"Simple C++ TBAA"}
!15 = !DILocation(line: 187, column: 16, scope: !16, inlinedAt: !18)
!16 = distinct !DISubprogram(name: "operator>>", scope: !17, file: !17, line: 186, type: !7, scopeLine: 187, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition | DISPFlagOptimized, unit: !0, retainedNodes: !8)
!17 = !DIFile(filename: "/usr/bin/../lib/gcc/x86_64-linux-gnu/11/../../../../include/c++/11/istream", directory: "")
!18 = distinct !DILocation(line: 13, column: 6, scope: !6)
!19 = !DILocation(line: 19, column: 11, scope: !6)
!20 = !DILocation(line: 19, column: 8, scope: !6)
!21 = !DILocation(line: 28, column: 8, scope: !6)
!22 = !DILocation(line: 29, column: 8, scope: !6)
!23 = !DILocation(line: 187, column: 16, scope: !16, inlinedAt: !24)
!24 = distinct !DILocation(line: 31, column: 6, scope: !6)
!25 = !DILocation(line: 34, column: 8, scope: !6)
!26 = !DILocation(line: 36, column: 8, scope: !6)
!27 = !DILocation(line: 37, column: 11, scope: !6)
!28 = !DILocation(line: 37, column: 8, scope: !6)
!29 = !DILocation(line: 42, column: 8, scope: !6)
!30 = !DILocation(line: 43, column: 8, scope: !6)
!31 = !DILocation(line: 46, column: 8, scope: !6)
!32 = !DILocation(line: 47, column: 8, scope: !6)
!33 = !DILocation(line: 48, column: 8, scope: !6)
!34 = !DILocation(line: 187, column: 16, scope: !16, inlinedAt: !35)
!35 = distinct !DILocation(line: 49, column: 6, scope: !6)
!36 = !DILocation(line: 52, column: 8, scope: !6)
!37 = !DILocation(line: 54, column: 8, scope: !6)
!38 = !DILocation(line: 55, column: 11, scope: !6)
!39 = !DILocation(line: 55, column: 8, scope: !6)
!40 = !DILocation(line: 60, column: 8, scope: !6)
!41 = !DILocation(line: 61, column: 8, scope: !6)
!42 = !DILocation(line: 64, column: 8, scope: !6)
!43 = !DILocation(line: 65, column: 8, scope: !6)
!44 = !DILocation(line: 66, column: 8, scope: !6)
!45 = !DILocation(line: 187, column: 16, scope: !16, inlinedAt: !46)
!46 = distinct !DILocation(line: 67, column: 6, scope: !6)
!47 = !DILocation(line: 70, column: 8, scope: !6)
!48 = !DILocation(line: 71, column: 8, scope: !6)
!49 = !DILocation(line: 72, column: 8, scope: !6)
!50 = !DILocation(line: 73, column: 11, scope: !6)
!51 = !DILocation(line: 73, column: 8, scope: !6)
!52 = !DILocation(line: 78, column: 8, scope: !6)
!53 = !DILocation(line: 79, column: 8, scope: !6)
!54 = !DILocation(line: 82, column: 8, scope: !6)
!55 = !DILocation(line: 83, column: 8, scope: !6)
!56 = !DILocation(line: 84, column: 8, scope: !6)
!57 = !DILocation(line: 187, column: 16, scope: !16, inlinedAt: !58)
!58 = distinct !DILocation(line: 85, column: 6, scope: !6)
!59 = !DILocation(line: 88, column: 8, scope: !6)
!60 = !DILocation(line: 90, column: 8, scope: !6)
!61 = !DILocation(line: 91, column: 11, scope: !6)
!62 = !DILocation(line: 91, column: 8, scope: !6)
!63 = !DILocation(line: 96, column: 8, scope: !6)
!64 = !DILocation(line: 97, column: 8, scope: !6)
!65 = !DILocation(line: 100, column: 8, scope: !6)
!66 = !DILocation(line: 101, column: 8, scope: !6)
!67 = !DILocation(line: 102, column: 8, scope: !6)
!68 = !DILocation(line: 187, column: 16, scope: !16, inlinedAt: !69)
!69 = distinct !DILocation(line: 103, column: 6, scope: !6)
!70 = !DILocation(line: 106, column: 8, scope: !6)
!71 = !DILocation(line: 107, column: 8, scope: !6)
!72 = !DILocation(line: 108, column: 8, scope: !6)
!73 = !DILocation(line: 109, column: 11, scope: !6)
!74 = !DILocation(line: 109, column: 8, scope: !6)
!75 = !DILocation(line: 114, column: 8, scope: !6)
!76 = !DILocation(line: 115, column: 8, scope: !6)
!77 = !DILocation(line: 118, column: 8, scope: !6)
!78 = !DILocation(line: 119, column: 8, scope: !6)
!79 = !DILocation(line: 120, column: 8, scope: !6)
!80 = !DILocation(line: 187, column: 16, scope: !16, inlinedAt: !81)
!81 = distinct !DILocation(line: 121, column: 6, scope: !6)
!82 = !DILocation(line: 124, column: 8, scope: !6)
!83 = !DILocation(line: 126, column: 8, scope: !6)
!84 = !DILocation(line: 127, column: 11, scope: !6)
!85 = !DILocation(line: 127, column: 8, scope: !6)
!86 = !DILocation(line: 132, column: 8, scope: !6)
!87 = !DILocation(line: 133, column: 8, scope: !6)
!88 = !DILocation(line: 136, column: 8, scope: !6)
!89 = !DILocation(line: 137, column: 8, scope: !6)
!90 = !DILocation(line: 138, column: 8, scope: !6)
!91 = !DILocation(line: 187, column: 16, scope: !16, inlinedAt: !92)
!92 = distinct !DILocation(line: 139, column: 6, scope: !6)
!93 = !DILocation(line: 142, column: 8, scope: !6)
!94 = !DILocation(line: 143, column: 8, scope: !6)
!95 = !DILocation(line: 144, column: 8, scope: !6)
!96 = !DILocation(line: 145, column: 11, scope: !6)
!97 = !DILocation(line: 145, column: 8, scope: !6)
!98 = !DILocation(line: 150, column: 8, scope: !6)
!99 = !DILocation(line: 151, column: 8, scope: !6)
!100 = !DILocation(line: 154, column: 8, scope: !6)
!101 = !DILocation(line: 155, column: 8, scope: !6)
!102 = !DILocation(line: 156, column: 8, scope: !6)
!103 = !DILocation(line: 187, column: 16, scope: !16, inlinedAt: !104)
!104 = distinct !DILocation(line: 157, column: 6, scope: !6)
!105 = !DILocation(line: 160, column: 8, scope: !6)
!106 = !DILocation(line: 162, column: 8, scope: !6)
!107 = !DILocation(line: 163, column: 11, scope: !6)
!108 = !DILocation(line: 163, column: 8, scope: !6)
!109 = !DILocation(line: 168, column: 8, scope: !6)
!110 = !DILocation(line: 169, column: 8, scope: !6)
!111 = !DILocation(line: 172, column: 8, scope: !6)
!112 = !DILocation(line: 173, column: 8, scope: !6)
!113 = !DILocation(line: 174, column: 8, scope: !6)
!114 = !DILocation(line: 187, column: 16, scope: !16, inlinedAt: !115)
!115 = distinct !DILocation(line: 175, column: 6, scope: !6)
!116 = !DILocation(line: 178, column: 8, scope: !6)
!117 = !DILocation(line: 180, column: 8, scope: !6)
!118 = !DILocation(line: 181, column: 11, scope: !6)
!119 = !DILocation(line: 181, column: 8, scope: !6)
!120 = !DILocation(line: 186, column: 8, scope: !6)
!121 = !DILocation(line: 187, column: 8, scope: !6)
!122 = !DILocation(line: 190, column: 8, scope: !6)
!123 = !DILocation(line: 191, column: 8, scope: !6)
!124 = !DILocation(line: 192, column: 8, scope: !6)
!125 = !DILocation(line: 187, column: 16, scope: !16, inlinedAt: !126)
!126 = distinct !DILocation(line: 193, column: 6, scope: !6)
!127 = !DILocation(line: 196, column: 8, scope: !6)
!128 = !DILocation(line: 197, column: 8, scope: !6)
!129 = !DILocation(line: 198, column: 8, scope: !6)
!130 = !DILocation(line: 199, column: 11, scope: !6)
!131 = !DILocation(line: 199, column: 8, scope: !6)
!132 = !DILocation(line: 204, column: 8, scope: !6)
!133 = !DILocation(line: 205, column: 8, scope: !6)
!134 = !DILocation(line: 208, column: 8, scope: !6)
!135 = !DILocation(line: 209, column: 8, scope: !6)
!136 = !DILocation(line: 210, column: 8, scope: !6)
!137 = !DILocation(line: 187, column: 16, scope: !16, inlinedAt: !138)
!138 = distinct !DILocation(line: 211, column: 6, scope: !6)
!139 = !DILocation(line: 214, column: 8, scope: !6)
!140 = !DILocation(line: 215, column: 8, scope: !6)
!141 = !DILocation(line: 216, column: 8, scope: !6)
!142 = !DILocation(line: 217, column: 11, scope: !6)
!143 = !DILocation(line: 217, column: 8, scope: !6)
!144 = !DILocation(line: 222, column: 8, scope: !6)
!145 = !DILocation(line: 223, column: 8, scope: !6)
!146 = !DILocation(line: 226, column: 8, scope: !6)
!147 = !DILocation(line: 227, column: 8, scope: !6)
!148 = !DILocation(line: 228, column: 8, scope: !6)
!149 = !DILocation(line: 187, column: 16, scope: !16, inlinedAt: !150)
!150 = distinct !DILocation(line: 229, column: 6, scope: !6)
!151 = !DILocation(line: 232, column: 8, scope: !6)
!152 = !DILocation(line: 233, column: 8, scope: !6)
!153 = !DILocation(line: 234, column: 8, scope: !6)
!154 = !DILocation(line: 235, column: 11, scope: !6)
!155 = !DILocation(line: 235, column: 8, scope: !6)
!156 = !DILocation(line: 240, column: 8, scope: !6)
!157 = !DILocation(line: 241, column: 8, scope: !6)
!158 = !DILocation(line: 244, column: 8, scope: !6)
!159 = !DILocation(line: 245, column: 8, scope: !6)
!160 = !DILocation(line: 246, column: 8, scope: !6)
!161 = !DILocation(line: 187, column: 16, scope: !16, inlinedAt: !162)
!162 = distinct !DILocation(line: 247, column: 6, scope: !6)
!163 = !DILocation(line: 250, column: 8, scope: !6)
!164 = !DILocation(line: 251, column: 8, scope: !6)
!165 = !DILocation(line: 252, column: 8, scope: !6)
!166 = !DILocation(line: 253, column: 11, scope: !6)
!167 = !DILocation(line: 253, column: 8, scope: !6)
!168 = !DILocation(line: 254, column: 6, scope: !6)
!169 = !DILocation(line: 258, column: 8, scope: !6)
!170 = !DILocation(line: 259, column: 8, scope: !6)
!171 = !DILocation(line: 262, column: 8, scope: !6)
!172 = !DILocation(line: 263, column: 8, scope: !6)
!173 = !DILocation(line: 264, column: 8, scope: !6)
!174 = !DILocation(line: 611, column: 2, scope: !175, inlinedAt: !177)
!175 = distinct !DISubprogram(name: "operator<<<std::char_traits<char> >", scope: !176, file: !176, line: 606, type: !7, scopeLine: 607, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition | DISPFlagOptimized, unit: !0, retainedNodes: !8)
!176 = !DIFile(filename: "/usr/bin/../lib/gcc/x86_64-linux-gnu/11/../../../../include/c++/11/ostream", directory: "")
!177 = distinct !DILocation(line: 267, column: 8, scope: !6)
!178 = !DILocation(line: 682, column: 29, scope: !179, inlinedAt: !180)
!179 = distinct !DISubprogram(name: "endl<char, std::char_traits<char> >", scope: !176, file: !176, line: 681, type: !7, scopeLine: 682, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition | DISPFlagOptimized, unit: !0, retainedNodes: !8)
!180 = distinct !DILocation(line: 113, column: 9, scope: !181, inlinedAt: !182)
!181 = distinct !DISubprogram(name: "operator<<", scope: !176, file: !176, line: 108, type: !7, scopeLine: 109, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition | DISPFlagOptimized, unit: !0, retainedNodes: !8)
!182 = distinct !DILocation(line: 267, column: 22, scope: !6)
!183 = !{!184, !184, i64 0}
!184 = !{!"vtable pointer", !14, i64 0}
!185 = !DILocation(line: 450, column: 30, scope: !186, inlinedAt: !188)
!186 = distinct !DISubprogram(name: "widen", scope: !187, file: !187, line: 449, type: !7, scopeLine: 450, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition | DISPFlagOptimized, unit: !0, retainedNodes: !8)
!187 = !DIFile(filename: "/usr/bin/../lib/gcc/x86_64-linux-gnu/11/../../../../include/c++/11/bits/basic_ios.h", directory: "")
!188 = distinct !DILocation(line: 682, column: 34, scope: !179, inlinedAt: !180)
!189 = !{!190, !191, i64 240}
!190 = !{!"_ZTSSt9basic_iosIcSt11char_traitsIcEE", !191, i64 216, !13, i64 224, !192, i64 225, !191, i64 232, !191, i64 240, !191, i64 248, !191, i64 256}
!191 = !{!"any pointer", !13, i64 0}
!192 = !{!"bool", !13, i64 0}
!193 = !DILocation(line: 49, column: 12, scope: !194, inlinedAt: !195)
!194 = distinct !DISubprogram(name: "__check_facet<std::ctype<char> >", scope: !187, file: !187, line: 47, type: !7, scopeLine: 48, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition | DISPFlagOptimized, unit: !0, retainedNodes: !8)
!195 = distinct !DILocation(line: 450, column: 16, scope: !186, inlinedAt: !188)
!196 = !DILocation(line: 49, column: 11, scope: !194, inlinedAt: !195)
!197 = !DILocation(line: 50, column: 2, scope: !194, inlinedAt: !195)
!198 = !DILocation(line: 877, column: 6, scope: !199, inlinedAt: !201)
!199 = distinct !DISubprogram(name: "widen", scope: !200, file: !200, line: 875, type: !7, scopeLine: 876, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition | DISPFlagOptimized, unit: !0, retainedNodes: !8)
!200 = !DIFile(filename: "/usr/bin/../lib/gcc/x86_64-linux-gnu/11/../../../../include/c++/11/bits/locale_facets.h", directory: "")
!201 = distinct !DILocation(line: 450, column: 40, scope: !186, inlinedAt: !188)
!202 = !{!203, !13, i64 56}
!203 = !{!"_ZTSSt5ctypeIcE", !191, i64 16, !192, i64 24, !191, i64 32, !191, i64 40, !191, i64 48, !13, i64 56, !13, i64 57, !13, i64 313, !13, i64 569}
!204 = !DILocation(line: 878, column: 11, scope: !199, inlinedAt: !201)
!205 = !{!13, !13, i64 0}
!206 = !DILocation(line: 878, column: 4, scope: !199, inlinedAt: !201)
!207 = !DILocation(line: 879, column: 8, scope: !199, inlinedAt: !201)
!208 = !DILocation(line: 880, column: 15, scope: !199, inlinedAt: !201)
!209 = !DILocation(line: 880, column: 2, scope: !199, inlinedAt: !201)
!210 = !DILocation(line: 0, scope: !199, inlinedAt: !201)
!211 = !DILocation(line: 682, column: 25, scope: !179, inlinedAt: !180)
!212 = !DILocation(line: 704, column: 19, scope: !213, inlinedAt: !214)
!213 = distinct !DISubprogram(name: "flush<char, std::char_traits<char> >", scope: !176, file: !176, line: 703, type: !7, scopeLine: 704, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition | DISPFlagOptimized, unit: !0, retainedNodes: !8)
!214 = distinct !DILocation(line: 682, column: 14, scope: !179, inlinedAt: !180)
!215 = !DILocation(line: 611, column: 2, scope: !175, inlinedAt: !216)
!216 = distinct !DILocation(line: 268, column: 8, scope: !6)
!217 = !DILocation(line: 268, column: 22, scope: !6)
!218 = !DILocation(line: 167, column: 16, scope: !219, inlinedAt: !220)
!219 = distinct !DISubprogram(name: "operator<<", scope: !176, file: !176, line: 166, type: !7, scopeLine: 167, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition | DISPFlagOptimized, unit: !0, retainedNodes: !8)
!220 = distinct !DILocation(line: 268, column: 19, scope: !6)
!221 = !DILocation(line: 611, column: 2, scope: !175, inlinedAt: !222)
!222 = distinct !DILocation(line: 268, column: 24, scope: !6)
!223 = !DILocation(line: 611, column: 2, scope: !175, inlinedAt: !224)
!224 = distinct !DILocation(line: 268, column: 31, scope: !6)
!225 = !DILocation(line: 167, column: 16, scope: !219, inlinedAt: !226)
!226 = distinct !DILocation(line: 268, column: 38, scope: !6)
!227 = !DILocation(line: 611, column: 2, scope: !175, inlinedAt: !228)
!228 = distinct !DILocation(line: 268, column: 43, scope: !6)
!229 = !DILocation(line: 167, column: 16, scope: !219, inlinedAt: !230)
!230 = distinct !DILocation(line: 268, column: 50, scope: !6)
!231 = !DILocation(line: 611, column: 2, scope: !175, inlinedAt: !232)
!232 = distinct !DILocation(line: 268, column: 55, scope: !6)
!233 = !DILocation(line: 167, column: 16, scope: !219, inlinedAt: !234)
!234 = distinct !DILocation(line: 268, column: 62, scope: !6)
!235 = !DILocation(line: 682, column: 29, scope: !179, inlinedAt: !236)
!236 = distinct !DILocation(line: 113, column: 9, scope: !181, inlinedAt: !237)
!237 = distinct !DILocation(line: 268, column: 67, scope: !6)
!238 = !DILocation(line: 450, column: 30, scope: !186, inlinedAt: !239)
!239 = distinct !DILocation(line: 682, column: 34, scope: !179, inlinedAt: !236)
!240 = !DILocation(line: 49, column: 12, scope: !194, inlinedAt: !241)
!241 = distinct !DILocation(line: 450, column: 16, scope: !186, inlinedAt: !239)
!242 = !DILocation(line: 49, column: 11, scope: !194, inlinedAt: !241)
!243 = !DILocation(line: 50, column: 2, scope: !194, inlinedAt: !241)
!244 = !DILocation(line: 877, column: 6, scope: !199, inlinedAt: !245)
!245 = distinct !DILocation(line: 450, column: 40, scope: !186, inlinedAt: !239)
!246 = !DILocation(line: 878, column: 11, scope: !199, inlinedAt: !245)
!247 = !DILocation(line: 878, column: 4, scope: !199, inlinedAt: !245)
!248 = !DILocation(line: 879, column: 8, scope: !199, inlinedAt: !245)
!249 = !DILocation(line: 880, column: 15, scope: !199, inlinedAt: !245)
!250 = !DILocation(line: 880, column: 2, scope: !199, inlinedAt: !245)
!251 = !DILocation(line: 0, scope: !199, inlinedAt: !245)
!252 = !DILocation(line: 682, column: 25, scope: !179, inlinedAt: !236)
!253 = !DILocation(line: 704, column: 19, scope: !213, inlinedAt: !254)
!254 = distinct !DILocation(line: 682, column: 14, scope: !179, inlinedAt: !236)
!255 = !DILocation(line: 269, column: 12, scope: !6)
!256 = !DILocation(line: 269, column: 10, scope: !6)
!257 = !DILocation(line: 270, column: 1, scope: !6)
!258 = !DILocation(line: 269, column: 3, scope: !6)
!259 = distinct !DISubprogram(linkageName: "_GLOBAL__sub_I_prog.cpp", scope: !1, file: !1, type: !7, flags: DIFlagArtificial, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !0, retainedNodes: !8)
!260 = !DILocation(line: 74, column: 25, scope: !261, inlinedAt: !264)
!261 = !DILexicalBlockFile(scope: !263, file: !262, discriminator: 0)
!262 = !DIFile(filename: "/usr/bin/../lib/gcc/x86_64-linux-gnu/11/../../../../include/c++/11/iostream", directory: "")
!263 = distinct !DISubprogram(name: "__cxx_global_var_init", scope: !1, file: !1, type: !7, flags: DIFlagArtificial, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !0, retainedNodes: !8)
!264 = distinct !DILocation(line: 0, scope: !259)
!265 = !DILocation(line: 0, scope: !263, inlinedAt: !264)
