import * as React from "react";
import * as ReactDOM from "react-dom";
import * as qlik from "qlik";

import { Layout } from "./types";
import { Root } from "./components/Root";
import { validate } from "./loadScriptIdentifiers";

export function mounted(elem: HTMLElement[], layout: Layout) {
    if (process.env.NODE_ENV === "development") {
        validate(qlik.currApp());
    }
}

// TODO: submit PR to fix this method def as the element is a jquery element
// wrapper and not a `HTMLElement`
export function paint(this: any, elem: HTMLElement, layout: Layout) {

    if (!this.beforeDestroy) {
        this.beforeDestroy = function() {
            const mountSucess = ReactDOM.unmountComponentAtNode(
                (elem as any)[0],
            );

            if (!mountSucess) {
                console.warn(
                    "opco-data-submission-extension did not dismount. Potential memory leak",
                );
            }
        };
    }

    ReactDOM.render(<Root />, (elem as any)[0]);
}
