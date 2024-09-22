export * from "./adapt/fetchAdapt";
export * from "./errors";
export declare function reqHandler(dto: any, method: string, pathKey: string, { paths }: any): {
    method: string;
    url: string;
    body: any;
    headers: {
        accept: string;
        "content-type": string;
    };
};
export declare function resHandler(response: any): any;
//# sourceMappingURL=index.d.ts.map