import {APIEndpoints} from "src/types/api/APIEndpoints";
import {isAPIError, PaginatedResponse} from "src/types/api/APITypes";
import {ChallengeAPIDocument} from "src/types/database/Challenge";
import {reactive, watch} from "vue";

export default class ChallengeStore {
    private requireReload = false;
    pagination: PaginatedResponse<ChallengeAPIDocument>;

    // CONSTRUCTORS -----------------------------------------------------------

    constructor() {
        let pagination = reactive<PaginatedResponse<ChallengeAPIDocument>>({
            count: 0,
            totalCount: 0,
            page: 0,
            rowsPerPage: 30,
            totalPages: 0,
            results: [],
        });

        // This watch checks the pagination has changes and reloads the challenge according to it.
        watch(pagination, () => {
            this.requireReload = true;
        }, {
            deep: true,
        });

        this.pagination = pagination;
    }

    // GETTERS ----------------------------------------------------------------

    // METHODS ----------------------------------------------------------------

    async reloadChallengeList(force: boolean = false) {
        if (!this.requireReload && !force) {
            return;
        }

        let response = await APIEndpoints.challenge.list({
            responses: {
                page: this.pagination.page,
                rowsPerPage: this.pagination.rowsPerPage,
            },
        });

        if (isAPIError(response)) {
            throw response;
        }

        this.pagination = response;
    };
}
