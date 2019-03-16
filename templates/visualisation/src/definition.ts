import * as PKG from "../package.json";
import { QlikProps } from "./types.js";
import { DeepReadonly } from "utility-types";

export const defaultQlikProps: DeepReadonly<QlikProps> = {};

const about = {
    label: "About",
    component: "items",
    items: {
        header1: {
            label: `${PKG.name}`,
            style: "header",
            component: "text",
        },
        paragraph1: {
            label:
                "An extension",
            component: "text",
        },
        header2: {
            label: "Version",
            style: "header",
            component: "text",
        },
        paragraph2: {
            label: `Extension: v${PKG.version}`,
            component: "text",
        },
       
    },
};

export const definition = {
    type: "items",
    component: "accordion",
    items: {
        settings: {
            uses: "settings",
        },
        about,
    },
} as ExtensionAPI.IDefinition;
