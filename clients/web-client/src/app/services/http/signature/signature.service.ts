import {Injectable} from "@angular/core";
import {HttpClient} from "@angular/common/http";

import {SignatureRequestBody, SignatureRequestResponse} from "../../../types/api/models/Signature";

import {environment} from "../../../../environments/environment";

const {API_SERVER_URL} = environment;

@Injectable({
    providedIn: "root",
})
export class SignatureService {

    constructor(private http: HttpClient) {
    }

    request(params: SignatureRequestBody): Promise<SignatureRequestResponse> {
        return this.http.post<SignatureRequestResponse>(`${API_SERVER_URL}/signature/request`, params).toPromise();
    }
}