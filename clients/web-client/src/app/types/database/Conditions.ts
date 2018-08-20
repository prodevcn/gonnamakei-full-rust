export interface OrderedCondition<T> {
    condition: "anyOf" | "noneOf" | ">" | ">=" | "<" | "<=",
    value: T[]
}

export interface OptionCondition<T> {
    condition: "anyOf" | "noneOf";
    value: T[];
}