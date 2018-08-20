export type DBDataType =
    DBDataType_bool
    | DBDataType_natural
    | DBDataType_integer
    | DBDataType_real
    | DBDataType_string
    | DBDataType_list
    | DBDataType_object
    | DBDataType_dateTime
    | DBDataType_date
    | DBDataType_dayTime
    | DBDataType_timeDuration;

export interface DBDataType_bool {
    T: "B",
    V: boolean
}

export interface DBDataType_natural {
    T: "Nu",
    V: number
}

export interface DBDataType_integer {
    T: "Ni",
    V: number
}

export interface DBDataType_real {
    T: "Nf",
    V: number
}

export interface DBDataType_string {
    T: "S",
    V: string
}

export interface DBDataType_list {
    T: "L",
    V: DBDataType[]
}

export interface DBDataType_object {
    T: "O",
    V: { [key: string]: DBDataType }
}

export interface DBDataType_dateTime {
    T: "DT",
    V: number
}

export interface DBDataType_date {
    T: "D",
    V: number
}

export interface DBDataType_dayTime {
    T: "T",
    V: number
}

export interface DBDataType_timeDuration {
    T: "TD",
    V: number
}