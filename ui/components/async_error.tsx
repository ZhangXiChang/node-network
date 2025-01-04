import { Accessor, createSignal, JSX, Setter } from "solid-js";

export class AsyncError {
    error: Accessor<Error | undefined>;
    setError: Setter<Error | undefined>;
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
