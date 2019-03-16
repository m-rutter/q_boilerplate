import {
    useEffect,
    useReducer,
    useRef,
    Dispatch,
    MutableRefObject,
    Reducer,
} from "react";

interface GenericObjectState<T> {
    initialised: boolean;
    loading: boolean;
    error: Error | null;
    layout: T | null;
}

const initalState: Readonly<GenericObjectState<null>> = {
    initialised: false,
    loading: true,
    error: null,
    layout: null,
};

/**
 * React hook for subscribing to and interacting with a Qlik hypercube
 * @param qlik - Qlik Root API
 * @param def - HyperCubeDef
 */
export function useGenericObject<
    QlikLayoutType extends EngineAPI.IGenericBaseLayout
>(qlik: RootAPI.IRoot, def: any): GenericObjectState<QlikLayoutType> {
    const [state, dispatch] = useReducer<
        Reducer<GenericObjectState<QlikLayoutType>, Action<QlikLayoutType>>
    >(reducer, initalState);

    const objRef = useRef<EngineAPI.IGenericObject | null>(null);

    useEffect(() => {
        const sub = subcribeToGenericObject(qlik, def, dispatch, objRef);

        return () => {
            sub.then(unsub => unsub());
            dispatch({ type: "reset" });
            objRef.current = null;
        };
    }, [def]);

    return state;
}

function reducer<T>(
    state: Readonly<GenericObjectState<T>>,
    action: Action<T>,
): GenericObjectState<T> {
    switch (action.type) {
        case "reset":
            return initalState;

        case "changed":
        case "initialised":
            return {
                ...initalState,
                loading: false,
                initialised: true,
                layout: action.payload,
            };

        case "loading":
            return { ...state, loading: true };

        case "layoutError":
            return {
                ...initalState,
                loading: false,
                initialised: true,
                error: action.error,
                layout: action.payload,
            };

        case "error":
            return {
                ...state,
                loading: false,
                error: action.error,
            };

        default:
            return state;
    }
}

async function subcribeToGenericObject<T>(
    qlik: RootAPI.IRoot,
    def: any,
    dispatch: Dispatch<Action<T>>,
    ref: MutableRefObject<EngineAPI.IGenericObject | null>,
): Promise<() => void> {
    let app: AppAPI.IApp;

    try {
        app = qlik.currApp();
    } catch (err) {
        dispatch({
            type: "error",
            error: new Error(
                `Error when attempting to access current app object: ${err}`,
            ),
        });

        return () => {};
    }

    let obj: EngineAPI.IGenericObject;

    try {
        obj = await app.createGenericObject(def);
        ref.current = obj;
    } catch (err) {
        dispatch({
            type: "error",
            error: new Error(
                `Error attempting to create new generic object, ${err}`,
            ),
        });

        return () => app.destroySessionObject(obj.id);
    }

    let layout: any;
    try {
        layout = await obj.getLayout();
        dispatch({
            type: "initialised",
            payload: layout,
        });
    } catch (err) {
        dispatch({
            type: "error",
            error: new Error(
                `Error attempting to get the layout object, ${err}`,
            ),
        });
    }

    obj.on("changed", async () => {
        dispatch({ type: "loading" });

        try {
            const layout: any = await obj.getLayout();

            if (layout.qError) {
                dispatch({
                    type: "layoutError",
                    payload: layout,
                    error: new Error(
                        `Error in qlik layout object: ${layout.qError}`,
                    ),
                });
            } else {
                dispatch({ type: "changed", payload: layout });
            }
        } catch (err) {
            dispatch({
                type: "error",
                error: new Error(`Error attempting to update: ${err}`),
            });
        }
    });

    return () => app.destroySessionObject(obj.id);
}


interface ResetAction {
    type: "reset";
}

interface InitalLoadAction<T> {
    type: "initialised";
    payload: T;
}

interface ChangedAction<T> {
    type: "changed";
    payload: T;
}

interface LoadingAction {
    type: "loading";
}

interface ErrorAction {
    type: "error";
    error: Error;
}

interface ErrorLayoutAction<T> {
    type: "layoutError";
    error: Error;
    payload: T;
}

type Action<T> =
    | ResetAction
    | InitalLoadAction<T>
    | ChangedAction<T>
    | LoadingAction
    | ErrorAction
    | ErrorLayoutAction<T>;
