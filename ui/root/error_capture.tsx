import { children, createSignal, ErrorBoundary, JSX } from "solid-js";

const [error, setError] = createSignal(undefined as Error | undefined);
export function captureError(err: Error) {
    setError(err);
}
export function ErrorCapture(props: { children?: JSX.Element }) {
    const selfChildren = children(() => props.children);
    return (
        <ErrorBoundary fallback={(err: Error) => err.message}>
            <>{(() => { if (error()) throw error(); })()}</>
            {selfChildren()}
        </ErrorBoundary>
    );
}
