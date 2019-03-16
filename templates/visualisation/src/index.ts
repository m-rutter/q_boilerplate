import "core-js"; // es6+ polyfills
import "whatwg-fetch"; // fetch polyfill
import "react-block-ui/style.css"; // css for a loading screen

import { initialProperties } from "./initialProperties";

import { paint, mounted } from "./paint";
import { definition } from "./definition";

import { ModernExtension } from "./types";

export default {
    definition,
    initialProperties,
    support: {
        snapshot: false,
        export: false,
        exportData: false,
    },
    mounted,
    paint,
} as ModernExtension;
