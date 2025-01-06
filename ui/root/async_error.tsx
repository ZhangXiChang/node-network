import { createSignal, JSX } from "solid-js";

export class AsyncError {
    private error;
    private setError;

    constructor() {
        const [error, setError] = createSignal(undefined as Error | undefined);
        this.error = error;
        this.setError = setError;
    }
    trigger(): JSX.Element {
        return <>{(() => { if (this.error()) throw this.error(); })()}</>;
    }
    capture(err: Error) {
        this.setError(err);
    }
}
