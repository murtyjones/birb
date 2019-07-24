
interface ResponseInfo {
    status: number,
    body: {
        [key: string]: any
    },
}

export const http = async (request: RequestInfo): Promise<ResponseInfo> => {
    const response = await fetch(request);
    const body = await response.json() || {};
    if (!response.ok) {
        return Promise.reject({
            status: response.status,
            body,
        })
    }
    return Promise.resolve({
        status: response.status,
        body,
    })
};
