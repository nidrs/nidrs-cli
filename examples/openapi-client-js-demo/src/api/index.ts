import { fetchAdapt } from "@nidrs/openapi-client-js";
import { Api } from "@nidist/api-client";

export default new Api(fetchAdapt(fetch));
