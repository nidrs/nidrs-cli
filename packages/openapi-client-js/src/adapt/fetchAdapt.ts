import env from "../env";

export function fetchAdapt(fetchFn: typeof fetch) {
  return async (requestOptions: any) => {
    if (env.DEBUG) console.log("[FetchAdapt] request:", requestOptions);
    const { method, url, body, headers } = requestOptions;
    const accept = headers.accept;

    const response = await fetchFn(url, {
      method,
      body: body ? JSON.stringify(body) : undefined,
      headers,
    });

    const resHeaders = {};

    response.headers.forEach((value, key) => {
      resHeaders[key] = value;
    });

    const res = {
      status: response.status,
      statusText: response.statusText,
      headers: resHeaders,
      data:
        accept === "application/json"
          ? await response.json()
          : await response.text(),
    };
    if (env.DEBUG) console.log("[FetchAdapt] response:", res);

    return res;
  };
}
