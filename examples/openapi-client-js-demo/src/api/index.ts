import { fetchAdapt } from "@nidrs/openapi-client-js";

import { Api } from "./api";

export default new Api(fetchAdapt(fetch));
