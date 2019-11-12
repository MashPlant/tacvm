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

FUNC<_Matrix_New> {
    parm 4
    _T0 = call _Alloc
    _T1 = VTBL<_Matrix>
    *(_T0 + 0) = _T1
    return _T0
}

FUNC<_DenseMatrix_New> {
    parm 8
    _T0 = call _Alloc
    _T1 = VTBL<_DenseMatrix>
    *(_T0 + 0) = _T1
    *(_T0 + 4) = 0
    return _T0
}

FUNC<_SparseItem_New> {
    parm 16
    _T0 = call _Alloc
    _T1 = VTBL<_SparseItem>
    *(_T0 + 0) = _T1
    *(_T0 + 4) = 0
    *(_T0 + 8) = 0
    *(_T0 + 12) = 0
    return _T0
}

FUNC<_SparseMatrix_New> {
    parm 8
    _T0 = call _Alloc
    _T1 = VTBL<_SparseMatrix>
    *(_T0 + 0) = _T1
    *(_T0 + 4) = 0
    return _T0
}

FUNC<_Main_New> {
    parm 4
    _T0 = call _Alloc
    _T1 = VTBL<_Main>
    *(_T0 + 0) = _T1
    return _T0
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
    _T1 =  0
    branch _L5
    _L1:
    _T2 =  0
    branch _L3
    _L2:
    parm _T0
    parm _T1
    parm _T2
    _T4 = *(_T0 + 0)
    _T4 = *(_T4 + 16)
    _T3 = call _T4
    parm _T3
    call _PrintInt
    _T5 = "\t"
    parm _T5
    call _PrintString
    _T6 = (_T2 + 1)
    _T2 =  _T6
    _L3:
    _T7 = (_T2 < 10)
    if (_T7 != 0) branch _L2
    _T8 = "\n"
    parm _T8
    call _PrintString
    _T9 = (_T1 + 1)
    _T1 =  _T9
    _L5:
    _T10 = (_T1 < 10)
    if (_T10 != 0) branch _L1
    return
}

FUNC<_Matrix.SeedMatrix> {
    _T1 =  0
    branch _L5
    _L1:
    _T2 =  0
    branch _L3
    _L2:
    _T3 = (_T1 + _T2)
    parm _T0
    parm _T1
    parm _T2
    parm _T3
    _T4 = *(_T0 + 0)
    _T4 = *(_T4 + 12)
    call _T4
    _T5 = (_T2 + 1)
    _T2 =  _T5
    _L3:
    _T6 = (_T2 < 5)
    if (_T6 != 0) branch _L2
    _T7 = (_T1 + 1)
    _T1 =  _T7
    _L5:
    _T8 = (_T1 < 5)
    if (_T8 != 0) branch _L1
    parm _T0
    parm 2
    parm 3
    parm 4
    _T15 = *(_T0 + 0)
    _T20 = *(_T15 + 12)
    call _T20
    parm _T0
    parm 4
    parm 6
    parm 2
    call _T20
    parm _T0
    parm 2
    parm 3
    parm 5
    call _T20
    parm _T0
    parm 0
    parm 0
    parm 1
    call _T20
    parm _T0
    parm 1
    parm 6
    parm 3
    call _T20
    parm _T0
    parm 7
    parm 7
    parm 7
    call _T20
    return
}

FUNC<_DenseMatrix.Init> {
    parm 44
    _T4 = call _Alloc
    _T2 = (_T4 + 44)
    _T4 = (_T4 + 4)
    branch _L3
    _L2:
    _T2 = (_T2 - 4)
    *(_T2 + 0) = 0
    _L3:
    _T1 = (_T2 == _T4)
    if (_T1 == 0) branch _L2
    *(_T4 - 4) = 10
    *(_T0 + 4) = _T4
    _T5 =  0
    branch _L13
    _L5:
    parm 44
    _T9 = call _Alloc
    _T7 = (_T9 + 44)
    _T9 = (_T9 + 4)
    branch _L8
    _L7:
    _T7 = (_T7 - 4)
    *(_T7 + 0) = 0
    _L8:
    _T6 = (_T7 == _T9)
    if (_T6 == 0) branch _L7
    *(_T9 - 4) = 10
    _T10 = *(_T0 + 4)
    _T12 = *(_T10 - 4)
    _T11 = (_T5 >= 0)
    _T13 = (_T5 < _T12)
    _T11 = (_T11 && _T13)
    if (_T11 == 0) branch _L11
    _T14 = (_T5 * 4)
    _T14 = (_T14 + _T10)
    *(_T14 + 0) = _T9
    branch _L12
    _L11:
    _T15 = "Decaf runtime error: Array subscript out of bounds\n"
    parm _T15
    call _PrintString
    call _Halt
    _L12:
    _T16 = (_T5 + 1)
    _T5 =  _T16
    _L13:
    _T17 = (_T5 < 10)
    if (_T17 != 0) branch _L5
    _T18 =  0
    branch _L25
    _L15:
    _T19 =  0
    branch _L23
    _L16:
    _T20 = *(_T0 + 4)
    _T22 = *(_T20 - 4)
    _T21 = (_T18 >= 0)
    _T23 = (_T18 < _T22)
    _T21 = (_T21 && _T23)
    if (_T21 == 0) branch _L18
    _T24 = (_T18 * 4)
    _T24 = (_T24 + _T20)
    _T25 = *(_T24 + 0)
    branch _L19
    _L18:
    _T26 = "Decaf runtime error: Array subscript out of bounds\n"
    parm _T26
    call _PrintString
    call _Halt
    _L19:
    _T28 = *(_T25 - 4)
    _T27 = (_T19 >= 0)
    _T29 = (_T19 < _T28)
    _T27 = (_T27 && _T29)
    if (_T27 == 0) branch _L21
    _T30 = (_T19 * 4)
    _T30 = (_T30 + _T25)
    *(_T30 + 0) = 0
    branch _L22
    _L21:
    _T31 = "Decaf runtime error: Array subscript out of bounds\n"
    parm _T31
    call _PrintString
    call _Halt
    _L22:
    _T32 = (_T19 + 1)
    _T19 =  _T32
    _L23:
    _T33 = (_T19 < 10)
    if (_T33 != 0) branch _L16
    _T34 = (_T18 + 1)
    _T18 =  _T34
    _L25:
    _T35 = (_T18 < 10)
    if (_T35 != 0) branch _L15
    return
}

FUNC<_DenseMatrix.Set> {
    _T4 = *(_T0 + 4)
    _T6 = *(_T4 - 4)
    _T5 = (_T1 >= 0)
    _T7 = (_T1 < _T6)
    _T5 = (_T5 && _T7)
    if (_T5 == 0) branch _L2
    _T8 = (_T1 * 4)
    _T8 = (_T8 + _T4)
    _T9 = *(_T8 + 0)
    branch _L3
    _L2:
    _T10 = "Decaf runtime error: Array subscript out of bounds\n"
    parm _T10
    call _PrintString
    call _Halt
    _L3:
    _T12 = *(_T9 - 4)
    _T11 = (_T2 >= 0)
    _T13 = (_T2 < _T12)
    _T11 = (_T11 && _T13)
    if (_T11 == 0) branch _L5
    _T14 = (_T2 * 4)
    _T14 = (_T14 + _T9)
    *(_T14 + 0) = _T3
    branch _L6
    _L5:
    _T15 = "Decaf runtime error: Array subscript out of bounds\n"
    parm _T15
    call _PrintString
    call _Halt
    _L6:
    return
}

FUNC<_DenseMatrix.Get> {
    _T3 = *(_T0 + 4)
    _T5 = *(_T3 - 4)
    _T4 = (_T1 >= 0)
    _T6 = (_T1 < _T5)
    _T4 = (_T4 && _T6)
    if (_T4 == 0) branch _L2
    _T7 = (_T1 * 4)
    _T7 = (_T7 + _T3)
    _T8 = *(_T7 + 0)
    branch _L3
    _L2:
    _T9 = "Decaf runtime error: Array subscript out of bounds\n"
    parm _T9
    call _PrintString
    call _Halt
    _L3:
    _T11 = *(_T8 - 4)
    _T10 = (_T2 >= 0)
    _T12 = (_T2 < _T11)
    _T10 = (_T10 && _T12)
    if (_T10 == 0) branch _L5
    _T13 = (_T2 * 4)
    _T13 = (_T13 + _T8)
    _T14 = *(_T13 + 0)
    branch _L6
    _L5:
    _T15 = "Decaf runtime error: Array subscript out of bounds\n"
    parm _T15
    call _PrintString
    call _Halt
    _L6:
    return _T14
}

FUNC<_SparseItem.Init> {
    *(_T0 + 4) = _T1
    *(_T0 + 8) = _T2
    *(_T0 + 12) = _T3
    return
}

FUNC<_SparseItem.GetNext> {
    _T1 = *(_T0 + 12)
    return _T1
}

FUNC<_SparseItem.GetY> {
    _T1 = *(_T0 + 8)
    return _T1
}

FUNC<_SparseItem.GetData> {
    _T1 = *(_T0 + 4)
    return _T1
}

FUNC<_SparseItem.SetData> {
    *(_T0 + 4) = _T1
    return
}

FUNC<_SparseMatrix.Init> {
    parm 44
    _T4 = call _Alloc
    _T2 = (_T4 + 44)
    _T4 = (_T4 + 4)
    branch _L3
    _L2:
    _T2 = (_T2 - 4)
    *(_T2 + 0) = 0
    _L3:
    _T1 = (_T2 == _T4)
    if (_T1 == 0) branch _L2
    *(_T4 - 4) = 10
    *(_T0 + 4) = _T4
    _T5 =  0
    branch _L9
    _L5:
    _T6 = *(_T0 + 4)
    _T8 = *(_T6 - 4)
    _T7 = (_T5 >= 0)
    _T9 = (_T5 < _T8)
    _T7 = (_T7 && _T9)
    if (_T7 == 0) branch _L7
    _T10 = (_T5 * 4)
    _T10 = (_T10 + _T6)
    *(_T10 + 0) = 0
    branch _L8
    _L7:
    _T11 = "Decaf runtime error: Array subscript out of bounds\n"
    parm _T11
    call _PrintString
    call _Halt
    _L8:
    _T12 = (_T5 + 1)
    _T5 =  _T12
    _L9:
    _T13 = (_T5 < 10)
    if (_T13 != 0) branch _L5
    return
}

FUNC<_SparseMatrix.Find> {
    _T4 = *(_T0 + 4)
    _T6 = *(_T4 - 4)
    _T5 = (_T1 >= 0)
    _T7 = (_T1 < _T6)
    _T5 = (_T5 && _T7)
    if (_T5 == 0) branch _L2
    _T8 = (_T1 * 4)
    _T8 = (_T8 + _T4)
    _T9 = *(_T8 + 0)
    branch _L3
    _L2:
    _T10 = "Decaf runtime error: Array subscript out of bounds\n"
    parm _T10
    call _PrintString
    call _Halt
    _L3:
    _T3 =  _T9
    branch _L7
    _L4:
    parm _T3
    _T17 = *(_T3 + 0)
    _T12 = *(_T17 + 16)
    _T11 = call _T12
    _T13 = (_T11 == _T2)
    if (_T13 == 0) branch _L6
    return _T3
    _L6:
    parm _T3
    _T15 = *(_T17 + 12)
    _T14 = call _T15
    _T3 =  _T14
    _L7:
    _T16 = (_T3 != 0)
    if (_T16 != 0) branch _L4
    return 0
}

FUNC<_SparseMatrix.Set> {
    parm _T0
    parm _T1
    parm _T2
    _T6 = *(_T0 + 0)
    _T6 = *(_T6 + 28)
    _T5 = call _T6
    _T7 = (_T5 != 0)
    if (_T7 == 0) branch _L2
    parm _T5
    parm _T3
    _T8 = *(_T5 + 0)
    _T8 = *(_T8 + 24)
    call _T8
    branch _L8
    _L2:
    _T9 = call _SparseItem_New
    _T10 = *(_T0 + 4)
    _T12 = *(_T10 - 4)
    _T24 = (_T1 >= 0)
    _T13 = (_T1 < _T12)
    _T11 = (_T24 && _T13)
    if (_T11 == 0) branch _L4
    _T25 = (_T1 * 4)
    _T14 = (_T25 + _T10)
    _T15 = *(_T14 + 0)
    branch _L5
    _L4:
    _T16 = "Decaf runtime error: Array subscript out of bounds\n"
    parm _T16
    call _PrintString
    call _Halt
    _L5:
    parm _T9
    parm _T3
    parm _T2
    parm _T15
    _T17 = *(_T9 + 0)
    _T17 = *(_T17 + 8)
    call _T17
    _T18 = *(_T0 + 4)
    _T20 = *(_T18 - 4)
    _T21 = (_T1 < _T20)
    _T19 = (_T24 && _T21)
    if (_T19 == 0) branch _L7
    _T22 = (_T25 + _T18)
    *(_T22 + 0) = _T9
    branch _L8
    _L7:
    _T23 = "Decaf runtime error: Array subscript out of bounds\n"
    parm _T23
    call _PrintString
    call _Halt
    _L8:
    return
}

FUNC<_SparseMatrix.Get> {
    parm _T0
    parm _T1
    parm _T2
    _T5 = *(_T0 + 0)
    _T5 = *(_T5 + 28)
    _T4 = call _T5
    _T6 = (_T4 != 0)
    if (_T6 == 0) branch _L2
    parm _T4
    _T8 = *(_T4 + 0)
    _T8 = *(_T8 + 20)
    _T7 = call _T8
    return _T7
    _L2:
    return 0
}

FUNC<main> {
    _T1 = "Dense Rep \n"
    parm _T1
    call _PrintString
    _T2 = call _DenseMatrix_New
    parm _T2
    _T11 = *(_T2 + 0)
    _T3 = *(_T11 + 8)
    call _T3
    parm _T2
    _T4 = *(_T11 + 24)
    call _T4
    parm _T2
    _T5 = *(_T11 + 20)
    call _T5
    _T6 = "Sparse Rep \n"
    parm _T6
    call _PrintString
    _T7 = call _SparseMatrix_New
    parm _T7
    _T13 = *(_T7 + 0)
    _T8 = *(_T13 + 8)
    call _T8
    parm _T7
    _T9 = *(_T13 + 24)
    call _T9
    parm _T7
    _T10 = *(_T13 + 20)
    call _T10
    return
}

