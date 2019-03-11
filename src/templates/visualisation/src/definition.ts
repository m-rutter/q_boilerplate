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
                "An extension for the Vodafone Data Quality Dashboard " +
                "providing user feedback mechanism via integration with the " +
                " Voafone Qlik Writeback API",
            component: "text",
        },
        header2: {
            label: "Versions",
            style: "header",
            component: "text",
        },
        paragraph2: {
            label: `Extension: v${PKG.version}`,
            component: "text",
        },
        paragraph3: {
            label: "Writeback API: v3.0.0",
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
