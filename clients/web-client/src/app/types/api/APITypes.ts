import {APIFilter} from "./APIFilter";

export interface APIError {
    errorCode: string,
    param?: string,
    message?: string
}

export function isAPIError(obj: any): obj is APIError {
    return obj?.errorCode !== undefined;
}

export interface PaginatedRequest<T> {
    sortBy: PaginatedSortByRequest[],
    page: number,
    rowsPerPage: number,
    filterBy?: APIFilter[],
    results: T,
    fieldsFilter: T,
    countPages: boolean,
}

export interface PaginatedSortByRequest {
    field: string,
    descending: boolean,
}

export interface PaginatedResponse<T> {
    count: number,
    totalCount?: number,
    page: number,
    rowsPerPage: number,
    totalPages?: number,
    results: T[],
}