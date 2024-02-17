; ModuleID = 'probe4.c2452a189817723e-cgu.0'
source_filename = "probe4.c2452a189817723e-cgu.0"
target datalayout = "e-m:e-p:32:32-i64:64-n32-S128"
target triple = "riscv32"

@alloc_08ca25ac7aefe822e62c3c60da621bf4 = private unnamed_addr constant <{ [45 x i8] }> <{ [45 x i8] c"/Users/kevin/rust/library/core/src/num/mod.rs" }>, align 1
@alloc_3a547c8681d387136ff1c67e8ea4bca6 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_08ca25ac7aefe822e62c3c60da621bf4, [12 x i8] c"-\00\00\00y\04\00\00\05\00\00\00" }>, align 4
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe4::probe
; Function Attrs: nounwind
define dso_local void @_ZN6probe45probe17h9435ff81b42dc35bE() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17heb28d3ed9f9794baE.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h863ebc51f6325ec0E(ptr align 1 @str.0, i32 25, ptr align 4 @alloc_3a547c8681d387136ff1c67e8ea4bca6) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17heb28d3ed9f9794baE.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn nounwind
declare dso_local void @_ZN4core9panicking5panic17h863ebc51f6325ec0E(ptr align 1, i32, ptr align 4) unnamed_addr #2

attributes #0 = { nounwind "target-cpu"="generic-rv32" "target-features"="+m" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { cold noinline noreturn nounwind "target-cpu"="generic-rv32" "target-features"="+m" }
attributes #3 = { noreturn nounwind }

!llvm.ident = !{!0}

!0 = !{!"rustc version 1.75.0-dev"}
