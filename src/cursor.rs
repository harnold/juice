use clang_sys;
use libc::c_void;
use std::any::Any;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Kind {

    // Declarations

    /// A declaration whose specific kind is not exposed via this interface.
    UnexposedDecl = 1,
    /// A C or C++ struct.
    StructDecl = 2,
    /// A C or C++ union.
    UnionDecl = 3,
    /// A C++ class.
    ClassDecl = 4,
    /// An enumeration.
    EnumDecl = 5,
    /// A field (in C) or non-static data member (in C++) in a struct, union, or C++ class.
    FieldDecl = 6,
    /// An enumerator constant.
    EnumConstantDecl = 7,
    /// A function.
    FunctionDecl = 8,
    /// A variable.
    VarDecl = 9,
    /// A function or method parameter.
    ParmDecl = 10,
    /// An Objective-C \@interface.
    ObjCInterfaceDecl = 11,
    /// An Objective-C \@interface for a category.
    ObjCCategoryDecl = 12,
    /// An Objective-C \@protocol declaration.
    ObjCProtocolDecl = 13,
    /// An Objective-C \@property declaration.
    ObjCPropertyDecl = 14,
    /// An Objective-C instance variable.
    ObjCIvarDecl = 15,
    /// An Objective-C instance method.
    ObjCInstanceMethodDecl = 16,
    /// An Objective-C class method.
    ObjCClassMethodDecl = 17,
    /// An Objective-C \@implementation.
    ObjCImplementationDecl = 18,
    /// An Objective-C \@implementation for a category.
    ObjCCategoryImplDecl = 19,
    /// A typedef.
    TypedefDecl = 20,
    /// A C++ class method.
    CXXMethod = 21,
    /// A C++ namespace.
    Namespace = 22,
    /// A linkage specification, e.g. 'extern "C"'.
    LinkageSpec = 23,
    /// A C++ constructor.
    Constructor = 24,
    /// A C++ destructor.
    Destructor = 25,
    /// A C++ conversion function.
    ConversionFunction = 26,
    /// A C++ template type parameter.
    TemplateTypeParameter = 27,
    /// A C++ non-type template parameter.
    NonTypeTemplateParameter = 28,
    /// A C++ template template parameter.
    TemplateTemplateParameter = 29,
    /// A C++ function template.
    FunctionTemplate = 30,
    /// A C++ class template.
    ClassTemplate = 31,
    /// A C++ class template partial specialization.
    ClassTemplatePartialSpecialization = 32,
    /// A C++ namespace alias declaration.
    NamespaceAlias = 33,
    /// A C++ using directive.
    UsingDirective = 34,
    /// A C++ using declaration.
    UsingDeclaration = 35,
    /// A C++ alias declaration
    TypeAliasDecl = 36,
    /// An Objective-C \@synthesize definition.
    ObjCSynthesizeDecl = 37,
    /// An Objective-C \@dynamic definition.
    ObjCDynamicDecl = 38,
    /// An access specifier.
    CXXAccessSpecifier = 39,

    // References

    ObjCSuperClassRef = 40,
    ObjCProtocolRef = 41,
    ObjCClassRef = 42,
    /// A reference to a type declaration.
    TypeRef = 43,
    CXXBaseSpecifier = 44,
    /// A reference to a class template, function template, template template parameter, or class
    /// template partial specialization.
    TemplateRef = 45,
    /// A reference to a namespace or namespace alias.
    NamespaceRef = 46,
    /// A reference to a member of a struct, union, or class that occurs in some non-expression
    /// context, e.g., a designated initializer.
    MemberRef = 47,
    /// A reference to a labeled statement.
    LabelRef = 48,
    /// A reference to a set of overloaded functions or function templates that has not yet been
    /// resolved to a specific function or function template.
    OverloadedDeclRef = 49,
    /// A reference to a variable that occurs in some non-expression context, e.g., a C++ lambda
    /// capture list.
    VariableRef = 50,

    // Error conditions

    InvalidFile = 70,
    NoDeclFound = 71,
    NotImplemented = 72,
    InvalidCode = 73,

    // Expressions

    /// An expression whose specific kind is not exposed via this interface.
    UnexposedExpr = 100,
    /// An expression that refers to some value declaration, such as a function, variable, or
    /// enumerator.
    DeclRefExpr = 101,
    /// An expression that refers to a member of a struct, union, class, Objective-C class, etc.
    MemberRefExpr = 102,
    /// An expression that calls a function.
    CallExpr = 103,
    /// An expression that sends a message to an Objective-C object or class.
    ObjCMessageExpr = 104,
    /// An expression that represents a block literal.
    BlockExpr = 105,
    /// An integer literal.
    IntegerLiteral = 106,
    /// A floating point number literal.
    FloatingLiteral = 107,
    /// An imaginary number literal.
    ImaginaryLiteral = 108,
    /// A string literal.
    StringLiteral = 109,
    /// A character literal.
    CharacterLiteral = 110,
    /// A parenthesized expression, e.g. "(1)".
    ParenExpr = 111,
    /// This represents the unary-expression's (except sizeof and alignof).
    UnaryOperator = 112,
    /// [C99 6.5.2.1] Array Subscripting.
    ArraySubscriptExpr = 113,
    /// A builtin binary operation expression such as "x + y" or "x <= y".
    BinaryOperator = 114,
    /// Compound assignment such as "+=".
    CompoundAssignOperator = 115,
    /// The ?: ternary operator.
    ConditionalOperator = 116,
    /// An explicit cast in C (C99 6.5.4) or a C-style cast in C++ (C++ [expr.cast]), which uses the
    /// syntax (Type)expr.
    CStyleCastExpr = 117,
    /// [C99 6.5.2.5]
    CompoundLiteralExpr = 118,
    /// Describes an C or C++ initializer list.
    InitListExpr = 119,
    /// The GNU address of label extension, representing &&label.
    AddrLabelExpr = 120,
    /// This is the GNU Statement Expression extension: ({int X=4; X;})
    StmtExpr = 121,
    /// Represents a C11 generic selection.
    GenericSelectionExpr = 122,
    /// Implements the GNU __null extension, which is a name for a null pointer constant that has
    /// integral type (e.g., int or long) and is the same size and alignment as a pointer.
    GNUNullExpr = 123,
    /// C++'s static_cast<> expression.
    CXXStaticCastExpr = 124,
    /// C++'s dynamic_cast<> expression.
    CXXDynamicCastExpr = 125,
    /// C++'s reinterpret_cast<> expression.
    CXXReinterpretCastExpr = 126,
    /// C++'s const_cast<> expression.
    CXXConstCastExpr = 127,
    /// Represents an explicit C++ type conversion that uses "functional" notion (C++
    /// [expr.type.conv]).
    CXXFunctionalCastExpr = 128,
    /// A C++ typeid expression (C++ [expr.typeid]).
    CXXTypeidExpr = 129,
    /// [C++ 2.13.5] C++ Boolean Literal.
    CXXBoolLiteralExpr = 130,
    /// [C++0x 2.14.7] C++ Pointer Literal.
    CXXNullPtrLiteralExpr = 131,
    /// Represents the "this" expression in C++.
    CXXThisExpr = 132,
    /// [C++ 15] C++ Throw Expression.
    CXXThrowExpr = 133,
    /// A new expression for memory allocation and constructor calls, e.g: "new CXXNewExpr(foo)".
    CXXNewExpr = 134,
    /// A delete expression for memory deallocation and destructor calls, e.g. "delete[] pArray".
    CXXDeleteExpr = 135,
    /// A unary expression. (noexcept, sizeof, or other traits).
    UnaryExpr = 136,
    /// An Objective-C string literal i.e. @"foo".
    ObjCStringLiteral = 137,
    /// An Objective-C \@encode expression.
    ObjCEncodeExpr = 138,
    /// An Objective-C \@selector expression.
    ObjCSelectorExpr = 139,
    /// An Objective-C \@protocol expression.
    ObjCProtocolExpr = 140,
    /// An Objective-C "bridged" cast expression, which casts between Objective-C pointers and C
    /// pointers, transferring ownership in the process.
    ObjCBridgedCastExpr = 141,
    /// Represents a C++0x pack expansion that produces a sequence of expressions.
    PackExpansionExpr = 142,
    /// Represents an expression that computes the length of a parameter pack.
    SizeOfPackExpr = 143,
    /// Represents a C++ lambda expression that produces a local function object.
    LambdaExpr = 144,
    /// Objective-c Boolean Literal.
    ObjCBoolLiteralExpr = 145,
    /// Represents the "self" expression in an Objective-C method.
    ObjCSelfExpr = 146,
    /// OpenMP 4.0 [2.4, Array Section].
    OMPArraySectionExpr = 147,
    /// Represents an @available(...) check.
    ObjCAvailabilityCheckExpr = 148,

    // Statements

    /// A statement whose specific kind is not exposed via this interface.
    UnexposedStmt = 200,
    /// A labelled statement in a function.
    LabelStmt = 201,
    /// A group of statements like { stmt stmt }.
    CompoundStmt = 202,
    /// A case statement.
    CaseStmt = 203,
    /// A default statement.
    DefaultStmt = 204,
    /// An if statement
    IfStmt = 205,
    /// A switch statement.
    SwitchStmt = 206,
    /// A while statement.
    WhileStmt = 207,
    /// A do statement.
    DoStmt = 208,
    /// A for statement.
    ForStmt = 209,
    /// A goto statement.
    GotoStmt = 210,
    /// An indirect goto statement.
    IndirectGotoStmt = 211,
    /// A continue statement.
    ContinueStmt = 212,
    /// A break statement.
    BreakStmt = 213,
    /// A return statement.
    ReturnStmt = 214,
    /// A GCC inline assembly statement extension.
    GCCAsmStmt = 215,    // AsmStmt
    /// Objective-C's overall \@try-\@catch-\@finally statement.
    ObjCAtTryStmt = 216,
    /// Objective-C's \@catch statement.
    ObjCAtCatchStmt = 217,
    /// Objective-C's \@finally statement.
    ObjCAtFinallyStmt = 218,
    /// Objective-C's \@throw statement.
    ObjCAtThrowStmt = 219,
    /// Objective-C's \@synchronized statement.
    ObjCAtSynchronizedStmt = 220,
    /// Objective-C's autorelease pool statement.
    ObjCAutoreleasePoolStmt = 221,
    /// Objective-C's collection statement.
    ObjCForCollectionStmt = 222,
    /// C++'s catch statement.
    CXXCatchStmt = 223,
    /// C++'s try statement.
    CXXTryStmt = 224,
    /// C++'s for (* : *) statement.
    CXXForRangeStmt = 225,
    /// Windows Structured Exception Handling's try statement.
    SEHTryStmt = 226,
    /// Windows Structured Exception Handling's except statement.
    SEHExceptStmt = 227,
    /// Windows Structured Exception Handling's finally statement.
    SEHFinallyStmt = 228,
    /// A MS inline assembly statement extension.
    MSAsmStmt = 229,
    /// The null statement ";": C99 6.8.3p3.
    NullStmt = 230,
    /// Adaptor class for mixing declarations with statements and expressions.
    DeclStmt = 231,
    /// OpenMP parallel directive.
    OMPParallelDirective = 232,
    /// OpenMP SIMD directive.
    OMPSimdDirective = 233,
    /// OpenMP for directive.
    OMPForDirective = 234,
    /// OpenMP sections directive.
    OMPSectionsDirective = 235,
    /// OpenMP section directive.
    OMPSectionDirective = 236,
    /// OpenMP single directive.
    OMPSingleDirective = 237,
    /// OpenMP parallel for directive.
    OMPParallelForDirective = 238,
    /// OpenMP parallel sections directive.
    OMPParallelSectionsDirective = 239,
    /// OpenMP task directive.
    OMPTaskDirective = 240,
    /// OpenMP master directive.
    OMPMasterDirective = 241,
    /// OpenMP critical directive.
    OMPCriticalDirective = 242,
    /// OpenMP taskyield directive.
    OMPTaskyieldDirective = 243,
    /// OpenMP barrier directive.
    OMPBarrierDirective = 244,
    /// OpenMP taskwait directive.
    OMPTaskwaitDirective = 245,
    /// OpenMP flush directive.
    OMPFlushDirective = 246,
    /// Windows Structured Exception Handling's leave statement.
    SEHLeaveStmt = 247,
    /// OpenMP ordered directive.
    OMPOrderedDirective = 248,
    /// OpenMP atomic directive.
    OMPAtomicDirective = 249,
    /// OpenMP for SIMD directive.
    OMPForSimdDirective = 250,
    /// OpenMP parallel for SIMD directive.
    OMPParallelForSimdDirective = 251,
    /// OpenMP target directive.
    OMPTargetDirective = 252,
    /// OpenMP teams directive.
    OMPTeamsDirective = 253,
    /// OpenMP taskgroup directive.
    OMPTaskgroupDirective = 254,
    /// OpenMP cancellation point directive.
    OMPCancellationPointDirective = 255,
    /// OpenMP cancel directive.
    OMPCancelDirective = 256,
    /// OpenMP target data directive.
    OMPTargetDataDirective = 257,
    /// OpenMP taskloop directive.
    OMPTaskLoopDirective = 258,
    /// OpenMP taskloop simd directive.
    OMPTaskLoopSimdDirective = 259,
    /// OpenMP distribute directive.
    OMPDistributeDirective = 260,
    /// OpenMP target enter data directive.
    OMPTargetEnterDataDirective = 261,
    /// OpenMP target exit data directive.
    OMPTargetExitDataDirective = 262,
    /// OpenMP target parallel directive.
    OMPTargetParallelDirective = 263,
    /// OpenMP target parallel for directive.
    OMPTargetParallelForDirective = 264,
    /// OpenMP target update directive.
    OMPTargetUpdateDirective = 265,
    /// OpenMP distribute parallel for directive.
    OMPDistributeParallelForDirective = 266,
    /// OpenMP distribute parallel for simd directive.
    OMPDistributeParallelForSimdDirective = 267,
    /// OpenMP distribute simd directive.
    OMPDistributeSimdDirective = 268,
    /// OpenMP target parallel for simd directive.
    OMPTargetParallelForSimdDirective = 269,
    /// OpenMP target simd directive.
    OMPTargetSimdDirective = 270,
    /// OpenMP teams distribute directive.
    OMPTeamsDistributeDirective = 271,
    /// OpenMP teams distribute simd directive.
    OMPTeamsDistributeSimdDirective = 272,
    /// OpenMP teams distribute parallel for simd directive.
    OMPTeamsDistributeParallelForSimdDirective = 273,
    /// OpenMP teams distribute parallel for directive.
    OMPTeamsDistributeParallelForDirective = 274,
    /// OpenMP target teams directive.
    OMPTargetTeamsDirective = 275,
    /// OpenMP target teams distribute directive.
    OMPTargetTeamsDistributeDirective = 276,
    /// OpenMP target teams distribute parallel for directive.
    OMPTargetTeamsDistributeParallelForDirective = 277,
    /// OpenMP target teams distribute parallel for simd directive.
    OMPTargetTeamsDistributeParallelForSimdDirective = 278,
    /// OpenMP target teams distribute simd directive.
    OMPTargetTeamsDistributeSimdDirective = 279,

    /// Cursor that represents the translation unit itself.
    TranslationUnit = 300,

    // Attributes

    /// An attribute whose specific kind is not exposed via this interface.
    UnexposedAttr = 400,
    IBActionAttr = 401,
    IBOutletAttr = 402,
    IBOutletCollectionAttr = 403,
    CXXFinalAttr = 404,
    CXXOverrideAttr = 405,
    AnnotateAttr = 406,
    AsmLabelAttr = 407,
    PackedAttr = 408,
    PureAttr = 409,
    ConstAttr = 410,
    NoDuplicateAttr = 411,
    CUDAConstantAttr = 412,
    CUDADeviceAttr = 413,
    CUDAGlobalAttr = 414,
    CUDAHostAttr = 415,
    CUDASharedAttr = 416,
    VisibilityAttr = 417,
    DLLExport = 418,
    DLLImport = 419,

    // Preprocessing

    PreprocessingDirective = 500,
    MacroDefinition = 501,
    MacroExpansion = 502,    // MacroInstantiation
    InclusionDirective = 503,

    // Extra Declarations

    /// A module import declaration.
    ModuleImportDecl = 600,
    TypeAliasTemplateDecl = 601,
    /// A static_assert or _Static_assert node
    StaticAssert = 602,
    /// A friend declaration.
    FriendDecl = 603,

    /// A code completion overload candidate.
    OverloadCandidate = 700
}

const FIRST_DECL: i32 = Kind::UnexposedDecl as i32;
const LAST_DECL: i32 = Kind::CXXAccessSpecifier as i32;
const FIRST_REF: i32 = Kind::ObjCSuperClassRef as i32;
const LAST_REF: i32 = Kind::VariableRef as i32;
const FIRST_INVALID: i32 = Kind::InvalidFile as i32;
const LAST_INVALID: i32 = Kind::InvalidCode as i32;
const FIRST_EXPR: i32 = Kind::UnexposedExpr as i32;
const LAST_EXPR: i32 = Kind::ObjCAvailabilityCheckExpr as i32;
const FIRST_STMT: i32 = Kind::UnexposedStmt as i32;
const LAST_STMT: i32 = Kind::OMPTargetTeamsDistributeSimdDirective as i32;
const FIRST_ATTR: i32 = Kind::UnexposedAttr as i32;
const LAST_ATTR: i32 = Kind::DLLImport as i32;
const FIRST_PREPROCESSING: i32 = Kind::PreprocessingDirective as i32;
const LAST_PREPROCESSING: i32 = Kind::InclusionDirective as i32;
const FIRST_EXTRA_DECL: i32 = Kind::ModuleImportDecl as i32;
const LAST_EXTRA_DECL: i32 = Kind::FriendDecl as i32;

impl Kind {
    pub fn is_declaration(self) -> bool {
        (self as i32 >= FIRST_DECL && self as i32 <= LAST_DECL) ||
        (self as i32 >= FIRST_EXTRA_DECL && self as i32 <= LAST_EXTRA_DECL)
    }

    pub fn is_reference(self) -> bool {
        self as i32 >= FIRST_REF && self as i32 <= LAST_REF
    }

    pub fn is_invalid(self) -> bool {
        self as i32 >= FIRST_INVALID && self as i32 <= LAST_INVALID
    }

    pub fn is_expression(self) -> bool {
        self as i32 >= FIRST_EXPR && self as i32 <= LAST_EXPR
    }

    pub fn is_statement(self) -> bool {
        self as i32 >= FIRST_STMT && self as i32 <= LAST_STMT
    }

    pub fn is_translation_unit(self) -> bool {
        self == Kind::TranslationUnit
    }

    pub fn is_attribute(self) -> bool {
        self as i32 >= FIRST_ATTR && self as i32 <= LAST_ATTR
    }

    pub fn is_preprocessing(self) -> bool {
        self as i32 >= FIRST_PREPROCESSING && self as i32 <= LAST_PREPROCESSING
    }

    pub fn is_unexposed(self) -> bool {
        (self as i32 == Kind::UnexposedDecl as i32) ||
        (self as i32 == Kind::UnexposedExpr as i32) ||
        (self as i32 == Kind::UnexposedStmt as i32) ||
        (self as i32 == Kind::UnexposedAttr as i32)
    }
}

impl From<clang_sys::CXCursorKind> for Kind {
    fn from(k: clang_sys::CXCursorKind) -> Kind {
        unsafe {
            let kind_ptr = &k as *const clang_sys::CXCursorKind as *const Kind;
            *kind_ptr
        }
    }
}

pub enum ChildVisitResult {
    Break,
    Continue,
    Recurse
}

impl From<ChildVisitResult> for clang_sys::CXVisitorResult {
    fn from(r: ChildVisitResult) -> clang_sys::CXVisitorResult {
        match r {
            ChildVisitResult::Break => clang_sys::CXChildVisit_Break,
            ChildVisitResult::Continue => clang_sys::CXChildVisit_Continue,
            ChildVisitResult::Recurse => clang_sys::CXChildVisit_Recurse
        }
    }
}

pub struct Cursor {
    obj: clang_sys::CXCursor
}

pub type CursorVisitor = fn(_: Cursor, _: Cursor, _: &mut Any) -> ChildVisitResult;

struct VisitorData <'a> {
    visitor: CursorVisitor,
    data: &'a mut Any
}

#[no_mangle]
pub extern "C" fn visit_cursor(
    cursor_obj: clang_sys::CXCursor,
    parent_obj: clang_sys::CXCursor,
    visitor_data_ptr: clang_sys::CXClientData,
) -> clang_sys::CXVisitorResult {

    let visitor_data = unsafe { &mut *(visitor_data_ptr as *mut VisitorData) };

    let result = (visitor_data.visitor)(
        Cursor::from_obj(cursor_obj),
        Cursor::from_obj(parent_obj),
        visitor_data.data
    );

    clang_sys::CXVisitorResult::from(result)
}

impl Cursor {
    pub fn from_obj(obj: clang_sys::CXCursor) -> Cursor {
        Cursor { obj: obj }
    }

    pub fn kind(&self) -> Kind {
        unsafe { Kind::from(clang_sys::clang_getCursorKind(self.obj)) }
    }

    pub fn display_name(&self) -> ::String {
        let s = unsafe { clang_sys::clang_getCursorDisplayName(self.obj) };
        ::String::from(s)
    }

    pub fn visit_children<T: Any>(
        &self,
        visitor: CursorVisitor,
        client_data: &mut T
    ) {
        let mut visitor_data = VisitorData {
            visitor: visitor,
            data: client_data as &mut Any
        };
        let visitor_data_ptr: *mut c_void = &mut visitor_data as *mut _ as *mut c_void;

        unsafe {
            clang_sys::clang_visitChildren(self.obj, visit_cursor, visitor_data_ptr);
            ()
        }
    }
}
