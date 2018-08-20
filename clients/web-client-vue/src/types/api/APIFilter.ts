import {DBDataType} from "src/types/database/DBDataType";

export type APIFilter = APIFilter_Expression | APIFilter_Or | APIFilter_And;

export interface APIFilter_Expression {
    T: "expr",
    V: APIFilterExpression
}

export interface APIFilter_Or {
    T: "or",
    V: APIFilter[]
}

export interface APIFilter_And {
    T: "and",
    V: APIFilter[]
}

export interface APIFilterExpression {
    left: APIFilterField,
    operator: FilterOperator,
    right: APIFilterField,
}

export type APIFilterField = APIFilterField_Field | APIFilterField_Constant | APIFilterField_Function;

export interface APIFilterField_Field {
    T: "field",
    V: string
}

export interface APIFilterField_Constant {
    T: "value",
    V: DBDataType
}

export interface APIFilterField_Function {
    T: "func",
    V: APIFilterFieldFunction
}

export interface APIFilterFieldFunction {
    name: APIFilterFieldFunctionKind,
    args: APIFilterField[],
}

export type FilterOperator = FilterOperator_simple | FilterOperator_array;

export interface FilterOperator_simple {
    T: "==" | "!=" | ">" | ">=" | "<" | "<=" | "in" | "!in" | "like" | "!like" | "regex" | "!regex";
}

export interface FilterOperator_array {
    T: "[all]" | "[any]" | "[none]";
    V: FilterOperator_simple;
}

export enum APIFilterFieldFunctionKind {
    Length = "length",

    // ARRAYS
    Nth = "nth", IndexOf = "indexOf", ListContains = "listContains",

    // OBJECTS
    HasKey = "hasKey",

    // LOGIC
    Not = "not",

    // STRING
    Trim = "trim", Lowercase = "lowercase", Uppercase = "uppercase", StartsWith = "startsWith", EndsWith = "endsWith", Like = "like", Regex = "regex", CharAt = "charAt", Contains = "contains",

    // TYPE CHECKING
    Typename = "typename",
}