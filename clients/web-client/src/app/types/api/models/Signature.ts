export interface LoginSignatureAction {
    T: "login";
}

export type SignatureAction = LoginSignatureAction;

export interface SignatureRequestBody {
    action: SignatureAction,
    address: string,
}

export interface SignatureRequestResponse {
    id: string;
    message: string;
}