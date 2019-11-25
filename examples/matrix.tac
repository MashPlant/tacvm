VTBL<_Matrix> {
    0
    "Matrix"
    FUNC<_Matrix.Init>
    FUNC<_Matrix.Set>
    FUNC<_Matrix.Get>
    FUNC<_Matrix.PrintMatrix>
    FUNC<_Matrix.SeedMatrix>
}

VTBL<_DenseMatrix> {
    VTBL<_Matrix>
    "DenseMatrix"
    FUNC<_DenseMatrix.Init>
    FUNC<_DenseMatrix.Set>
    FUNC<_DenseMatrix.Get>
    FUNC<_Matrix.PrintMatrix>
    FUNC<_Matrix.SeedMatrix>
}

VTBL<_SparseItem> {
    0
    "SparseItem"
    FUNC<_SparseItem.Init>
    FUNC<_SparseItem.GetNext>
    FUNC<_SparseItem.GetY>
    FUNC<_SparseItem.GetData>
    FUNC<_SparseItem.SetData>
}

VTBL<_SparseMatrix> {
    VTBL<_Matrix>
    "SparseMatrix"
    FUNC<_SparseMatrix.Init>
    FUNC<_SparseMatrix.Set>
    FUNC<_SparseMatrix.Get>
    FUNC<_Matrix.PrintMatrix>
    FUNC<_Matrix.SeedMatrix>
    FUNC<_SparseMatrix.Find>
}

VTBL<_Main> {
    0
    "Main"
}

FUNC<_Matrix._new> {
    parm 4
    %0 = call _Alloc
    %1 = VTBL<_Matrix>
    *(%0 + 0) = %1
    return %0
}

FUNC<_DenseMatrix._new> {
    parm 8
    %0 = call _Alloc
    %1 = VTBL<_DenseMatrix>
    *(%0 + 0) = %1
    *(%0 + 4) = 0
    return %0
}

FUNC<_SparseItem._new> {
    parm 16
    %0 = call _Alloc
    %1 = VTBL<_SparseItem>
    *(%0 + 0) = %1
    *(%0 + 4) = 0
    *(%0 + 8) = 0
    *(%0 + 12) = 0
    return %0
}

FUNC<_SparseMatrix._new> {
    parm 8
    %0 = call _Alloc
    %1 = VTBL<_SparseMatrix>
    *(%0 + 0) = %1
    *(%0 + 4) = 0
    return %0
}

FUNC<_Main._new> {
    parm 4
    %0 = call _Alloc
    %1 = VTBL<_Main>
    *(%0 + 0) = %1
    return %0
}

FUNC<_Matrix.Init> {
    return
}

FUNC<_Matrix.Set> {
    return
}

FUNC<_Matrix.Get> {
    return 0
}

FUNC<_Matrix.PrintMatrix> {
    %1 = 0
    branch %5
    %1:
    %2 = 0
    branch %3
    %2:
    parm %0
    parm %1
    parm %2
    %4 = *(%0 + 0)
    %4 = *(%4 + 16)
    %3 = call %4
    parm %3
    call _PrintInt
    %5 = "\t"
    parm %5
    call _PrintString
    %6 = (%2 + 1)
    %2 = %6
    %3:
    %7 = (%2 < 10)
    if (%7 != 0) branch %2
    %8 = "\n"
    parm %8
    call _PrintString
    %9 = (%1 + 1)
    %1 = %9
    %5:
    %10 = (%1 < 10)
    if (%10 != 0) branch %1
    return
}

FUNC<_Matrix.SeedMatrix> {
    %1 = 0
    branch %5
    %1:
    %2 = 0
    branch %3
    %2:
    %3 = (%1 + %2)
    parm %0
    parm %1
    parm %2
    parm %3
    %4 = *(%0 + 0)
    %4 = *(%4 + 12)
    call %4
    %5 = (%2 + 1)
    %2 = %5
    %3:
    %6 = (%2 < 5)
    if (%6 != 0) branch %2
    %7 = (%1 + 1)
    %1 = %7
    %5:
    %8 = (%1 < 5)
    if (%8 != 0) branch %1
    parm %0
    parm 2
    parm 3
    parm 4
    %15 = *(%0 + 0)
    %20 = *(%15 + 12)
    call %20
    parm %0
    parm 4
    parm 6
    parm 2
    call %20
    parm %0
    parm 2
    parm 3
    parm 5
    call %20
    parm %0
    parm 0
    parm 0
    parm 1
    call %20
    parm %0
    parm 1
    parm 6
    parm 3
    call %20
    parm %0
    parm 7
    parm 7
    parm 7
    call %20
    return
}

FUNC<_DenseMatrix.Init> {
    parm 44
    %4 = call _Alloc
    %2 = (%4 + 44)
    %4 = (%4 + 4)
    branch %3
    %2:
    %2 = (%2 - 4)
    *(%2 + 0) = 0
    %3:
    %1 = (%2 == %4)
    if (%1 == 0) branch %2
    *(%4 - 4) = 10
    *(%0 + 4) = %4
    %5 = 0
    branch %13
    %5:
    parm 44
    %9 = call _Alloc
    %7 = (%9 + 44)
    %9 = (%9 + 4)
    branch %8
    %7:
    %7 = (%7 - 4)
    *(%7 + 0) = 0
    %8:
    %6 = (%7 == %9)
    if (%6 == 0) branch %7
    *(%9 - 4) = 10
    %10 = *(%0 + 4)
    %12 = *(%10 - 4)
    %11 = (%5 >= 0)
    %13 = (%5 < %12)
    %11 = (%11 && %13)
    if (%11 == 0) branch %11
    %14 = (%5 * 4)
    %14 = (%14 + %10)
    *(%14 + 0) = %9
    branch %12
    %11:
    %15 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %15
    call _PrintString
    call _Halt
    %12:
    %16 = (%5 + 1)
    %5 = %16
    %13:
    %17 = (%5 < 10)
    if (%17 != 0) branch %5
    %18 = 0
    branch %25
    %15:
    %19 = 0
    branch %23
    %16:
    %20 = *(%0 + 4)
    %22 = *(%20 - 4)
    %21 = (%18 >= 0)
    %23 = (%18 < %22)
    %21 = (%21 && %23)
    if (%21 == 0) branch %18
    %24 = (%18 * 4)
    %24 = (%24 + %20)
    %25 = *(%24 + 0)
    branch %19
    %18:
    %26 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %26
    call _PrintString
    call _Halt
    %19:
    %28 = *(%25 - 4)
    %27 = (%19 >= 0)
    %29 = (%19 < %28)
    %27 = (%27 && %29)
    if (%27 == 0) branch %21
    %30 = (%19 * 4)
    %30 = (%30 + %25)
    *(%30 + 0) = 0
    branch %22
    %21:
    %31 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %31
    call _PrintString
    call _Halt
    %22:
    %32 = (%19 + 1)
    %19 = %32
    %23:
    %33 = (%19 < 10)
    if (%33 != 0) branch %16
    %34 = (%18 + 1)
    %18 = %34
    %25:
    %35 = (%18 < 10)
    if (%35 != 0) branch %15
    return
}

FUNC<_DenseMatrix.Set> {
    %4 = *(%0 + 4)
    %6 = *(%4 - 4)
    %5 = (%1 >= 0)
    %7 = (%1 < %6)
    %5 = (%5 && %7)
    if (%5 == 0) branch %2
    %8 = (%1 * 4)
    %8 = (%8 + %4)
    %9 = *(%8 + 0)
    branch %3
    %2:
    %10 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %10
    call _PrintString
    call _Halt
    %3:
    %12 = *(%9 - 4)
    %11 = (%2 >= 0)
    %13 = (%2 < %12)
    %11 = (%11 && %13)
    if (%11 == 0) branch %5
    %14 = (%2 * 4)
    %14 = (%14 + %9)
    *(%14 + 0) = %3
    branch %6
    %5:
    %15 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %15
    call _PrintString
    call _Halt
    %6:
    return
}

FUNC<_DenseMatrix.Get> {
    %3 = *(%0 + 4)
    %5 = *(%3 - 4)
    %4 = (%1 >= 0)
    %6 = (%1 < %5)
    %4 = (%4 && %6)
    if (%4 == 0) branch %2
    %7 = (%1 * 4)
    %7 = (%7 + %3)
    %8 = *(%7 + 0)
    branch %3
    %2:
    %9 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %9
    call _PrintString
    call _Halt
    %3:
    %11 = *(%8 - 4)
    %10 = (%2 >= 0)
    %12 = (%2 < %11)
    %10 = (%10 && %12)
    if (%10 == 0) branch %5
    %13 = (%2 * 4)
    %13 = (%13 + %8)
    %14 = *(%13 + 0)
    branch %6
    %5:
    %15 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %15
    call _PrintString
    call _Halt
    %6:
    return %14
}

FUNC<_SparseItem.Init> {
    *(%0 + 4) = %1
    *(%0 + 8) = %2
    *(%0 + 12) = %3
    return
}

FUNC<_SparseItem.GetNext> {
    %1 = *(%0 + 12)
    return %1
}

FUNC<_SparseItem.GetY> {
    %1 = *(%0 + 8)
    return %1
}

FUNC<_SparseItem.GetData> {
    %1 = *(%0 + 4)
    return %1
}

FUNC<_SparseItem.SetData> {
    *(%0 + 4) = %1
    return
}

FUNC<_SparseMatrix.Init> {
    parm 44
    %4 = call _Alloc
    %2 = (%4 + 44)
    %4 = (%4 + 4)
    branch %3
    %2:
    %2 = (%2 - 4)
    *(%2 + 0) = 0
    %3:
    %1 = (%2 == %4)
    if (%1 == 0) branch %2
    *(%4 - 4) = 10
    *(%0 + 4) = %4
    %5 = 0
    branch %9
    %5:
    %6 = *(%0 + 4)
    %8 = *(%6 - 4)
    %7 = (%5 >= 0)
    %9 = (%5 < %8)
    %7 = (%7 && %9)
    if (%7 == 0) branch %7
    %10 = (%5 * 4)
    %10 = (%10 + %6)
    *(%10 + 0) = 0
    branch %8
    %7:
    %11 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %11
    call _PrintString
    call _Halt
    %8:
    %12 = (%5 + 1)
    %5 = %12
    %9:
    %13 = (%5 < 10)
    if (%13 != 0) branch %5
    return
}

FUNC<_SparseMatrix.Find> {
    %4 = *(%0 + 4)
    %6 = *(%4 - 4)
    %5 = (%1 >= 0)
    %7 = (%1 < %6)
    %5 = (%5 && %7)
    if (%5 == 0) branch %2
    %8 = (%1 * 4)
    %8 = (%8 + %4)
    %9 = *(%8 + 0)
    branch %3
    %2:
    %10 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %10
    call _PrintString
    call _Halt
    %3:
    %3 = %9
    branch %7
    %4:
    parm %3
    %17 = *(%3 + 0)
    %12 = *(%17 + 16)
    %11 = call %12
    %13 = (%11 == %2)
    if (%13 == 0) branch %6
    return %3
    %6:
    parm %3
    %15 = *(%17 + 12)
    %14 = call %15
    %3 = %14
    %7:
    %16 = (%3 != 0)
    if (%16 != 0) branch %4
    return 0
}

FUNC<_SparseMatrix.Set> {
    parm %0
    parm %1
    parm %2
    %6 = *(%0 + 0)
    %6 = *(%6 + 28)
    %5 = call %6
    %7 = (%5 != 0)
    if (%7 == 0) branch %2
    parm %5
    parm %3
    %8 = *(%5 + 0)
    %8 = *(%8 + 24)
    call %8
    branch %8
    %2:
    %9 = call _SparseItem._new
    %10 = *(%0 + 4)
    %12 = *(%10 - 4)
    %24 = (%1 >= 0)
    %13 = (%1 < %12)
    %11 = (%24 && %13)
    if (%11 == 0) branch %4
    %25 = (%1 * 4)
    %14 = (%25 + %10)
    %15 = *(%14 + 0)
    branch %5
    %4:
    %16 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %16
    call _PrintString
    call _Halt
    %5:
    parm %9
    parm %3
    parm %2
    parm %15
    %17 = *(%9 + 0)
    %17 = *(%17 + 8)
    call %17
    %18 = *(%0 + 4)
    %20 = *(%18 - 4)
    %21 = (%1 < %20)
    %19 = (%24 && %21)
    if (%19 == 0) branch %7
    %22 = (%25 + %18)
    *(%22 + 0) = %9
    branch %8
    %7:
    %23 = "Decaf runtime error: Array subscript out of bounds\n"
    parm %23
    call _PrintString
    call _Halt
    %8:
    return
}

FUNC<_SparseMatrix.Get> {
    parm %0
    parm %1
    parm %2
    %5 = *(%0 + 0)
    %5 = *(%5 + 28)
    %4 = call %5
    %6 = (%4 != 0)
    if (%6 == 0) branch %2
    parm %4
    %8 = *(%4 + 0)
    %8 = *(%8 + 20)
    %7 = call %8
    return %7
    %2:
    return 0
}

FUNC<main> {
    %1 = "Dense Rep \n"
    parm %1
    call _PrintString
    %2 = call _DenseMatrix._new
    parm %2
    %11 = *(%2 + 0)
    %3 = *(%11 + 8)
    call %3
    parm %2
    %4 = *(%11 + 24)
    call %4
    parm %2
    %5 = *(%11 + 20)
    call %5
    %6 = "Sparse Rep \n"
    parm %6
    call _PrintString
    %7 = call _SparseMatrix._new
    parm %7
    %13 = *(%7 + 0)
    %8 = *(%13 + 8)
    call %8
    parm %7
    %9 = *(%13 + 24)
    call %9
    parm %7
    %10 = *(%13 + 20)
    call %10
    return
}

