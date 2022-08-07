; ModuleID = 'probe5.b7b913bf-cgu.0'
source_filename = "probe5.b7b913bf-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.7.0"

%"core::panic::location::Location" = type { { [0 x i8]*, i64 }, i32, i32 }

@alloc2 = private unnamed_addr constant <{ [4 x i8] }> <{ [4 x i8] c"\02\00\00\00" }>, align 4
@alloc3 = private unnamed_addr constant <{ [77 x i8] }> <{ [77 x i8] c"/rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/ops/arith.rs" }>, align 1
@alloc4 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [77 x i8] }>, <{ [77 x i8] }>* @alloc3, i32 0, i32 0, i32 0), [16 x i8] c"M\00\00\00\00\00\00\00\F9\02\00\00\01\00\00\00" }>, align 8
@str.0 = internal constant [28 x i8] c"attempt to add with overflow"

; probe5::probe
; Function Attrs: uwtable
define void @_ZN6probe55probe17h921dca5d2915defdE() unnamed_addr #0 {
start:
  %x = alloca i32, align 4
  store i32 1, i32* %x, align 4
; call <i32 as core::ops::arith::AddAssign<&i32>>::add_assign
  call void @"_ZN66_$LT$i32$u20$as$u20$core..ops..arith..AddAssign$LT$$RF$i32$GT$$GT$10add_assign17h99a165d27d592bd7E"(i32* align 4 dereferenceable(4) %x, i32* align 4 dereferenceable(4) bitcast (<{ [4 x i8] }>* @alloc2 to i32*))
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; <i32 as core::ops::arith::AddAssign>::add_assign
; Function Attrs: inlinehint uwtable
define internal void @"_ZN51_$LT$i32$u20$as$u20$core..ops..arith..AddAssign$GT$10add_assign17hef08acf7e4d96356E"(i32* align 4 dereferenceable(4) %self, i32 %other) unnamed_addr #1 {
start:
  %0 = load i32, i32* %self, align 4
  %1 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %0, i32 %other)
  %_4.0 = extractvalue { i32, i1 } %1, 0
  %_4.1 = extractvalue { i32, i1 } %1, 1
  %2 = call i1 @llvm.expect.i1(i1 %_4.1, i1 false)
  br i1 %2, label %panic, label %bb1

bb1:                                              ; preds = %start
  store i32 %_4.0, i32* %self, align 4
  ret void

panic:                                            ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h709cad72bd37e428E([0 x i8]* nonnull align 1 bitcast ([28 x i8]* @str.0 to [0 x i8]*), i64 28, %"core::panic::location::Location"* align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc4 to %"core::panic::location::Location"*)) #5
  unreachable
}

; <i32 as core::ops::arith::AddAssign<&i32>>::add_assign
; Function Attrs: inlinehint uwtable
define internal void @"_ZN66_$LT$i32$u20$as$u20$core..ops..arith..AddAssign$LT$$RF$i32$GT$$GT$10add_assign17h99a165d27d592bd7E"(i32* align 4 dereferenceable(4) %self, i32* align 4 dereferenceable(4) %other) unnamed_addr #1 {
start:
  %_5 = load i32, i32* %other, align 4
; call <i32 as core::ops::arith::AddAssign>::add_assign
  call void @"_ZN51_$LT$i32$u20$as$u20$core..ops..arith..AddAssign$GT$10add_assign17hef08acf7e4d96356E"(i32* align 4 dereferenceable(4) %self, i32 %_5)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare { i32, i1 } @llvm.sadd.with.overflow.i32(i32, i32) #2

; Function Attrs: nofree nosync nounwind readnone willreturn
declare i1 @llvm.expect.i1(i1, i1) #3

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17h709cad72bd37e428E([0 x i8]* nonnull align 1, i64, %"core::panic::location::Location"* align 8 dereferenceable(24)) unnamed_addr #4

attributes #0 = { uwtable "frame-pointer"="all" "probe-stack"="__rust_probestack" "target-cpu"="core2" }
attributes #1 = { inlinehint uwtable "frame-pointer"="all" "probe-stack"="__rust_probestack" "target-cpu"="core2" }
attributes #2 = { nofree nosync nounwind readnone speculatable willreturn }
attributes #3 = { nofree nosync nounwind readnone willreturn }
attributes #4 = { cold noinline noreturn uwtable "frame-pointer"="all" "probe-stack"="__rust_probestack" "target-cpu"="core2" }
attributes #5 = { noreturn }

!llvm.module.flags = !{!0}

!0 = !{i32 7, !"PIC Level", i32 2}
